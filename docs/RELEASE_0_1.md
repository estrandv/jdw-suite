# Release 0.1 Plan

First downloadable release of `jdw-suite`. Target: Linux (primary), macOS (secondary).

## Artifact

```
jdw-v0.1-linux-x86_64.zip
├── jdw                           # binary
├── install.sh                    # copies files to standard dirs, adds to PATH
├── hello.bbd                     # simple song
├── synthdefs.scd                 # minimal synthdefs
├── example.jdw.toml              # config template
└── README.txt
```

## install.sh

```bash
#!/usr/bin/env bash
# Install jdw: copy binary to /usr/local/bin, config to ~/.config, synthdefs to
# a standard location.
set -e

# Binary
sudo cp jdw /usr/local/bin/jdw
sudo chmod +x /usr/local/bin/jdw

# Config
mkdir -p ~/.config
cp example.jdw.toml ~/.config/jdw.toml

# Synthdefs
mkdir -p /usr/local/share/jdw
cp synthdefs.scd /usr/local/share/jdw/

echo "Installed. Edit ~/.config/jdw.toml to set synthdefs_scd_path."
echo "Run: jdw all"
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

No effects, drones, or macros — minimal dependency surface.

## synthdefs.scd

Just `pluck`:

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
bbd_root = "/usr/local/share/jdw"
synthdefs_scd_path = "/usr/local/share/jdw/synthdefs.scd"
template_synths_path = "/usr/local/share/jdw/synthdefs.scd"
sample_pack_dir = "~/sample_packs"
nrt_output_dir = "./output"
```

## README.txt (in zip)

```
Quick Start
===========

1. Install SuperCollider: https://supercollider.github.io/downloads
   This also installs Jack audio server if needed. Follow their platform guide.
2. Run: ./install.sh
3. Run: jdw all          (starts the suite)
4. In another terminal:
   jdw setup hello.bbd
   jdw play hello.bbd
5. Listen! Ctrl-C to stop.

WAV export:
   jdw nrt hello.bbd     (writes to ./output/)

Merge + playback of all NRT tracks:
   scripts/merge-nrt.sh
```

## Dependencies (user must install)

- **SuperCollider** — `sclang` and `scsynth` on PATH. Their installer handles
  Jack audio setup per platform. See https://supercollider.github.io/downloads
- No Python, Rust toolchain, or other services required.

## CI

**Enabled.** All git dependencies are public repos. One was private
(`jdw-billboarding-backend`), causing CI auth failures since `GITHUB_TOKEN`
only has access to the current repo. Making it public fixed everything —
no auth hacks needed.

If repos are made private again, either publish crates to crates.io
or use a PAT + `url.insteadOf`.

## macOS Codesigning

Not needed for a CLI binary distributed as a `.zip`. Users may need to run
`xattr -d com.apple.quarantine jdw` the first time (Gatekeeper). Official
codesigning requires an Apple Developer account ($99/year) — defer until
the app has a GUI or is distributed outside GitHub.

## Windows

Deferred. scsynth and Jack aren't easily available via package managers.
Config path convention (`~/.config/jdw.toml`) would also need a Windows
equivalent (`%APPDATA%\jdw\config.toml`). Revisit when there's demand.

## Release Steps

```bash
# 1. Ensure all tests pass
cd jdw-billboarding-backend && cargo test
cd jdw-suite && cargo build --release

# 2. Tag and push
git tag v0.1.0
git push origin v0.1.0

# 3. CI builds + uploads artifacts to GitHub Release automatically (ci.yml)
# 4. Verify the release page has the .zip with all assets

# Manual fallback if CI isn't ready:
gh release create v0.1.0 \
    --title "jdw v0.1.0" \
    --notes "First release. Linux (x86_64), macOS (x86_64+arm64)." \
    target/release/jdw \
    assets/hello.bbd \
    assets/synthdefs.scd \
    assets/example.jdw.toml \
    assets/install.sh
```

## Asset Files

All in `assets/`:

| File | Purpose |
|---|---|
| `hello.bbd` | Simple demo song |
| `synthdefs.scd` | Pluck synthdef |
| `example.jdw.toml` | Config template |
| `install.sh` | Install to standard system paths |
