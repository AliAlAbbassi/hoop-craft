fn main() {
    println!("cargo:rerun-if-changed=src/audio_capture_bridge.m");

    cc::Build::new()
        .file("src/audio_capture_bridge.m")
        .flag("-fobjc-arc")
        .compile("audio_capture_bridge");

    println!("cargo:rustc-link-lib=framework=ScreenCaptureKit");
    println!("cargo:rustc-link-lib=framework=CoreMedia");
    println!("cargo:rustc-link-lib=framework=CoreAudio");
    println!("cargo:rustc-link-lib=framework=Accelerate");
    println!("cargo:rustc-link-lib=framework=Foundation");
    println!("cargo:rustc-link-lib=framework=CoreGraphics");
}
