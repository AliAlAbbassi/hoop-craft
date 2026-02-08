#import <ScreenCaptureKit/ScreenCaptureKit.h>
#import <CoreMedia/CoreMedia.h>
#import <Accelerate/Accelerate.h>
#import <Foundation/Foundation.h>
#include <stdatomic.h>

// ─── Thread-safe audio levels ───
static _Atomic float g_bass = 0.0f;
static _Atomic float g_mid = 0.0f;
static _Atomic float g_treble = 0.0f;
static _Atomic float g_volume = 0.0f;
static _Atomic int g_active = 0;

// Persistent FFT setup (created once)
static FFTSetup g_fft_setup = NULL;
static int g_fft_log2n = 11; // 2048 samples

// ─── Stream output delegate ───
@interface AudioCaptureDelegate : NSObject <SCStreamOutput>
@end

@implementation AudioCaptureDelegate

- (void)stream:(SCStream *)stream
    didOutputSampleBuffer:(CMSampleBufferRef)sampleBuffer
               ofType:(SCStreamOutputType)type {
    if (type != SCStreamOutputTypeAudio) return;

    // Extract audio buffer list
    AudioBufferList audioBufferList;
    CMBlockBufferRef blockBuffer = NULL;

    OSStatus status = CMSampleBufferGetAudioBufferListWithRetainedBlockBuffer(
        sampleBuffer,
        NULL,
        &audioBufferList,
        sizeof(AudioBufferList),
        NULL, NULL,
        kCMSampleBufferFlag_AudioBufferList_Assure16ByteAlignment,
        &blockBuffer);

    if (status != noErr || audioBufferList.mNumberBuffers == 0) {
        if (blockBuffer) CFRelease(blockBuffer);
        return;
    }

    float *channelData = (float *)audioBufferList.mBuffers[0].mData;
    int sampleCount = audioBufferList.mBuffers[0].mDataByteSize / sizeof(float);

    if (!channelData || sampleCount < 64) {
        if (blockBuffer) CFRelease(blockBuffer);
        return;
    }

    // Overall RMS volume
    float rms = 0.0f;
    vDSP_rmsqv(channelData, 1, &rms, sampleCount);
    atomic_store(&g_volume, rms);

    // FFT analysis
    int fftSize = 1 << g_fft_log2n; // 2048
    if (sampleCount < fftSize) fftSize = sampleCount;

    // Ensure power of 2
    int log2n = 0;
    int temp = fftSize;
    while (temp > 1) { temp >>= 1; log2n++; }
    fftSize = 1 << log2n;

    if (fftSize < 64) {
        if (blockBuffer) CFRelease(blockBuffer);
        return;
    }

    // Apply Hann window
    float *windowed = (float *)malloc(fftSize * sizeof(float));
    float *window = (float *)malloc(fftSize * sizeof(float));
    vDSP_hann_window(window, fftSize, vDSP_HANN_NORM);
    vDSP_vmul(channelData, 1, window, 1, windowed, 1, fftSize);

    // Pack into split complex for real FFT
    DSPSplitComplex split;
    split.realp = (float *)malloc((fftSize / 2) * sizeof(float));
    split.imagp = (float *)malloc((fftSize / 2) * sizeof(float));
    vDSP_ctoz((DSPComplex *)windowed, 2, &split, 1, fftSize / 2);

    // Create FFT setup if needed (lazy init, matches current size)
    if (!g_fft_setup || g_fft_log2n != log2n) {
        if (g_fft_setup) vDSP_destroy_fftsetup(g_fft_setup);
        g_fft_setup = vDSP_create_fftsetup(log2n, FFT_RADIX2);
        g_fft_log2n = log2n;
    }

    // Forward FFT
    vDSP_fft_zrip(g_fft_setup, &split, 1, log2n, FFT_FORWARD);

    // Compute magnitudes
    int halfSize = fftSize / 2;
    float *magnitudes = (float *)malloc(halfSize * sizeof(float));
    vDSP_zvmags(&split, 1, magnitudes, 1, halfSize);

    // Scale
    float scale = 1.0f / (float)(fftSize * fftSize);
    vDSP_vsmul(magnitudes, 1, &scale, magnitudes, 1, halfSize);

    // Frequency band boundaries (assuming 48000 Hz sample rate)
    float sampleRate = 48000.0f;
    float binWidth = sampleRate / (float)fftSize;

    int bassEnd   = (int)(300.0f / binWidth);
    int midEnd    = (int)(2000.0f / binWidth);
    int trebleEnd = (int)(16000.0f / binWidth);

    if (bassEnd > halfSize)   bassEnd = halfSize;
    if (midEnd > halfSize)    midEnd = halfSize;
    if (trebleEnd > halfSize) trebleEnd = halfSize;

    // Sum each band
    float bassSum = 0, midSum = 0, trebleSum = 0;
    int bassCount = 0, midCount = 0, trebleCount = 0;

    for (int i = 1; i < bassEnd; i++)   { bassSum += magnitudes[i]; bassCount++; }
    for (int i = bassEnd; i < midEnd; i++)    { midSum += magnitudes[i]; midCount++; }
    for (int i = midEnd; i < trebleEnd; i++) { trebleSum += magnitudes[i]; trebleCount++; }

    // Average and normalize
    if (bassCount > 0)   bassSum /= (float)bassCount;
    if (midCount > 0)    midSum /= (float)midCount;
    if (trebleCount > 0) trebleSum /= (float)trebleCount;

    // Apply sqrt for perceptual scaling and amplify
    bassSum   = sqrtf(bassSum) * 6.0f;
    midSum    = sqrtf(midSum) * 8.0f;
    trebleSum = sqrtf(trebleSum) * 12.0f;

    // Clamp to 0-1
    if (bassSum > 1.0f)   bassSum = 1.0f;
    if (midSum > 1.0f)    midSum = 1.0f;
    if (trebleSum > 1.0f) trebleSum = 1.0f;

    atomic_store(&g_bass, bassSum);
    atomic_store(&g_mid, midSum);
    atomic_store(&g_treble, trebleSum);

    free(magnitudes);
    free(split.realp);
    free(split.imagp);
    free(windowed);
    free(window);
    if (blockBuffer) CFRelease(blockBuffer);
}

@end

// ─── Global state ───
static SCStream *g_stream = nil;
static AudioCaptureDelegate *g_delegate = nil;

// ─── C interface for Rust ───

void audio_capture_start(void) {
    if (atomic_load(&g_active)) return;

    [SCShareableContent getShareableContentExcludingDesktopWindows:NO
        onScreenWindowsOnly:NO
        completionHandler:^(SCShareableContent *content, NSError *error) {
            if (error || !content || content.displays.count == 0) {
                NSLog(@"[AudioCapture] Failed to get content: %@", error);
                return;
            }

            SCDisplay *display = content.displays.firstObject;
            SCContentFilter *filter = [[SCContentFilter alloc]
                initWithDisplay:display excludingWindows:@[]];

            SCStreamConfiguration *config = [[SCStreamConfiguration alloc] init];
            config.capturesAudio = YES;
            config.excludesCurrentProcessAudio = YES;
            config.channelCount = 1; // mono for simpler analysis
            config.sampleRate = 48000;
            // Minimal video (can't fully disable)
            config.width = 2;
            config.height = 2;
            config.minimumFrameInterval = CMTimeMake(1, 1);

            g_delegate = [[AudioCaptureDelegate alloc] init];
            g_stream = [[SCStream alloc] initWithFilter:filter
                                          configuration:config
                                               delegate:nil];

            NSError *addErr = nil;
            [g_stream addStreamOutput:g_delegate
                                 type:SCStreamOutputTypeAudio
                    sampleHandlerQueue:dispatch_get_global_queue(
                        QOS_CLASS_USER_INTERACTIVE, 0)
                                error:&addErr];

            if (addErr) {
                NSLog(@"[AudioCapture] Failed to add output: %@", addErr);
                return;
            }

            [g_stream startCaptureWithCompletionHandler:^(NSError *startErr) {
                if (startErr) {
                    NSLog(@"[AudioCapture] Failed to start: %@", startErr);
                } else {
                    NSLog(@"[AudioCapture] Started successfully");
                    atomic_store(&g_active, 1);
                }
            }];
        }];
}

void audio_capture_stop(void) {
    if (!g_stream) return;
    [g_stream stopCaptureWithCompletionHandler:^(NSError *_error) {
        (void)_error;
        NSLog(@"[AudioCapture] Stopped");
        atomic_store(&g_active, 0);
    }];
    g_stream = nil;
    g_delegate = nil;
}

int audio_capture_is_active(void) { return atomic_load(&g_active); }
float audio_capture_get_bass(void)   { return atomic_load(&g_bass); }
float audio_capture_get_mid(void)    { return atomic_load(&g_mid); }
float audio_capture_get_treble(void) { return atomic_load(&g_treble); }
float audio_capture_get_volume(void) { return atomic_load(&g_volume); }
