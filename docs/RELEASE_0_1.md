# Release 0.1 Plan

First downloadable release of `jdw-suite`. Target: Linux (primary), macOS (secondary).

## Artifact

Platform `.zip` per OS/arch, built by CI on tag push:

```
jdw-v0.1-x86_64-unknown-linux-gnu.zip
├── jdw                      # release binary
├── install.sh               # copies to /usr/local/bin, ~/.config, /usr/local/share
├── hello.bbd                # simple song
├── synthdefs.scd            # pluck synthdef
└── example.jdw.toml         # config template
```

## Release Steps

```bash
./scripts/release.sh    # prompts for version, runs tests, tags, pushes
```

CI picks up the tag, builds Linux + macOS binaries, packages them into
platform `.zip` files, and attaches them to the GitHub Release automatically.

## Install Scripts

| Script | Audience | What it does |
|---|---|---|
| `install.sh` (repo root) | Developers building from source | `cargo build --release`, copies to `~/.local/bin` |
| `assets/install.sh` (in release zip) | Users with pre-built binary | Copies to `/usr/local/bin`, `~/.config`, `/usr/local/share/jdw` |

## macOS Notes

- `install.sh` paths (`/usr/local/bin`, `~/.config`) are identical on macOS
- Binary may need `xattr -d com.apple.quarantine jdw` on first run (Gatekeeper)
- No codesigning needed for CLI distributed as `.zip`

## Windows

Deferred. scsynth and Jack aren't easily available via package managers.
Config path convention (`~/.config/jdw.toml`) would need a Windows equivalent
(`%APPDATA%\jdw\config.toml`). Revisit when there's demand.

## hello.bbd

```bbd
/set_bpm 120
DEFAULT sus0.5,amp0.4
>>> main
@pluck:main
(c4 d4 e4 f4 g4 a4 b4 c5):1,tot4
```

## synthdefs.scd

Pluck SynthDef (extracted from full synthdefs).

## example.jdw.toml

```toml
[pycompose]
bbd_root = "/usr/local/share/jdw"
synthdefs_scd_path = "/usr/local/share/jdw/synthdefs.scd"
template_synths_path = "/usr/local/share/jdw/synthdefs.scd"
sample_pack_dir = "~/sample_packs"
nrt_output_dir = "./output"
```

## CI

Enabled. All git dependencies are public repos. CI uses nightly Rust
(`jdw-osc-lib` requires `#![feature]`). Builds on tag `v*` or manual dispatch.

## Dependencies (user must install)

- **SuperCollider** (`sclang` and `scsynth` on PATH). Their installer handles
  Jack audio setup per platform.
- No Python, Rust toolchain, or other services required.
