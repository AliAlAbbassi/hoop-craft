# Hoop Craft

Genshin Impact-style anime action RPG set in a nightclub, built with Rust and Bevy 0.18. Features cel-shaded VRM characters, elemental combat, and audio-reactive visuals that pulse to your system audio.

## Features

- **4-Character Party** — Switch between Anya (Pyro), Mira (Hydro), Kai (Electro), and Suki (Cryo) with number keys
- **Combo Combat** — 5-hit combo chain with damage multipliers, elemental skills (E), and bursts (Q)
- **7 Elements** — Pyro, Hydro, Electro, Cryo, Anemo, Geo, Dendro with elemental auras and reactions
- **Cel Shading** — Custom WGSL shaders for anime-style rendering with outlines and bloom
- **VRM Characters** — Loads VRM 1.0 anime models for both party members and enemies
- **Audio-Reactive Nightclub** — Floor tiles pulse with bass, spotlights with mids, neon strips with treble — driven by your system audio via ScreenCaptureKit
- **Stamina System** — Sprint costs stamina, regenerates over time
- **Third-Person Camera** — Orbiting camera with collision detection

## Controls

| Key | Action |
|-----|--------|
| WASD | Move |
| Mouse | Look / Orbit camera |
| Left Click | Attack (5-hit combo) |
| Space | Jump |
| Left Shift | Sprint |
| E | Elemental Skill |
| Q | Elemental Burst |
| 1-4 | Switch party member |

## Tech Stack

| Crate | Version | Purpose |
|-------|---------|---------|
| bevy | 0.18 | Game engine |
| bevy_vrm1 | 0.5 | VRM model loading |
| avian3d | 0.5 | 3D physics |
| bevy_kira_audio | 0.25 | SFX playback |

## Requirements

- macOS 13+ (ScreenCaptureKit for audio capture)
- Rust nightly or stable with 2024 edition support
- Screen Recording permission (System Settings > Privacy & Security > Screen Recording) for audio-reactive visuals

## Build & Run

```sh
cargo run
```

Release build:

```sh
cargo run --release
```

## Project Structure

```
src/
├── main.rs              # App setup and plugin registration
├── app_state.rs         # Loading → MainMenu → InGame states
├── audio/               # SFX playback (sword, hit, footstep sounds)
├── audio_reactive/      # System audio capture + reactive visuals
├── camera/              # Third-person orbit camera with collision
├── character/           # Stats, movement, animation, weapon
├── combat/              # Combo system, hitboxes, damage, elemental reactions
├── elements/            # 7 element types, auras, reaction triggers
├── enemy/               # Enemy AI, spawning, health
├── input/               # Centralized input actions
├── loading/             # Asset loading tracker
├── party/               # 4-slot party with switching
├── rendering/           # Cel shading, outlines, bloom, lighting
├── stamina/             # Sprint stamina system
├── ui/                  # HUD, HP bars, action bar, party portraits
├── world/               # Nightclub terrain (floor tiles, neons, DJ booth, bar, VIP lounge)
└── audio_capture_bridge.m  # Objective-C ScreenCaptureKit FFI bridge

assets/
├── models/characters/   # VRM anime character models
├── audio/sfx/           # Sound effects (OGG)
└── shaders/             # WGSL cel shading, outline, post-processing shaders
```

## Audio-Reactive System

The nightclub visuals react to whatever audio is playing on your system (Apple Music, Spotify, YouTube, etc.) — no need to play music through the app itself.

Uses ScreenCaptureKit to capture system audio, then runs a 2048-sample FFT via Apple's Accelerate framework to split into bass/mid/treble bands. Requires Screen Recording permission on macOS.
