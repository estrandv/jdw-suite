# Release 0.1 Plan

First downloadable release of `jdw-suite`.

## Artifact

A `.zip` per platform containing:

```
jdw-v0.1-linux-x86_64.zip
├── jdw                           # binary
├── example.jdw.toml              # minimal config (user copies to ~/.config/jdw.toml)
├── hello.bbd                     # simple song: one synth, one melody
├── synthdefs.scd                 # minimal synthdefs needed by hello.bbd
└── README.txt                    # quick-start instructions
```

## hello.bbd

Single synth section with a filter, one track, basic melody:

```bbd
/set_bpm 120

DEFAULT sus0.5,amp0.4

>>> main

@pluck:main

(c4 d4 e4 f4 g4 a4 b4 c5):1,tot4
```

One filter (`>>> main`), one track, ascending C-major scale looped once. No
effects, no drones, no macros — minimal dependency surface.

## synthdefs.scd

Just the `pluck` synthdef (same one from the full synthdefs):

```
SynthDef("pluck", { |freq=440, amp=0.5, gate=1, out=0, pan=0, attT=0.001, decT=0.25, susL=0.0, relT=0.001, cut=5500, res=0.15, dur=0.25, fEnv=1.0, fSus=1.0, lfoS=0.5, lfoD=0.0|
    var env = EnvGen.kr(Env.perc(attT, decT + (dur*0.001) + relT, 1, -4), gate: gate, doneAction: Done.freeSelf);
    var sig = Pulse.ar(freq, LFNoise2.kr(10).range(0.05, 0.4));
    sig = RLPF.ar(sig, cut, res);
    sig = sig * env * amp;
    sig = Pan2.ar(sig, pan);
    Out.ar(out, sig);
})
```

## example.jdw.toml

```toml
[pycompose]
bbd_root = "."
synthdefs_scd_path = "./synthdefs.scd"
template_synths_path = "./synthdefs.scd"
sample_pack_dir = "~/sample_packs"
nrt_output_dir = "./output"
```

## CI (`ci.yml`)

GitHub Actions matrix build using `taiki-e/upload-rust-binary-action` (same
approach as tree-sitter repos). Builds per platform, creates GitHub Release
on tag push.

Matrix:
- **Linux** x86_64 (ubuntu-latest)
- **macOS** x86_64 + arm64 (macos-latest)
- ~~Windows~~ deferred (scsynth not easily available)

Steps per job:
1. Checkout with submodules/git deps
2. Install SuperCollider (`sudo apt install supercollider` / `brew install supercollider`)
3. `cargo build --release`
4. Package: `jdw` binary + example files → `.zip`
5. Upload to GitHub Release (on `v*` tag)

## Zip Contents

```
jdw-v0.1-linux-x86_64.zip
├── jdw                      # release binary
├── hello.bbd                # simple song
├── synthdefs.scd            # minimal synthdefs
├── example.jdw.toml         # config template
├── run.sh                   # one-step: all + setup + play
└── README.txt               # quick-start
```

## run.sh

```bash
#!/usr/bin/env bash
# One-step: launch suite, setup, and play a song.
# Usage: ./run.sh [song.bbd]   (defaults to hello.bbd)
set -e
SONG="${1:-hello.bbd}"
./jdw all &
sleep 3
./jdw setup "$SONG"
sleep 2
./jdw play "$SONG"
wait
```

## User Quick-Start (README.txt in zip)

```
1. Install SuperCollider: https://supercollider.github.io/downloads
2. Copy example.jdw.toml to ~/.config/jdw.toml
3. Run:  ./jdw all          (starts the suite)
4. Run:  ./jdw setup hello.bbd
5. Run:  ./jdw play hello.bbd
6. Listen! Ctrl-C to stop.

For WAV export:
7. Run:  ./jdw nrt hello.bbd   (writes to ./output/)
```

## Dependencies (user must install)

- **SuperCollider** (`sclang` and `scsynth` on PATH)
- No Python, no Rust toolchain, no other services

## Open Questions

- macOS codesigning? (requires Apple Developer account)
- Windows: scsynth not available via winget — bundle it?
- Jack audio server required on Linux? (currently uses default SC audio backend)
