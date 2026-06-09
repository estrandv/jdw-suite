# AGENTS.md — jdw-suite

> **This is the canonical entry point for any agent working on the jdw project.**
> Start here. Each sub-project has its own AGENTS.md with deeper module-level
> details, but this file establishes the full ecosystem map.

## Project Map

All repositories owned by [`estrandv`](https://github.com/estrandv). Every
crate is on `master` branch; we always use latest, never pin to specific revs.

| Crate | Type | Role |
|---|---|---|
| `jdw-suite` | Binary + orchestrator | `jdw` CLI — launch backends, send songs, manage suite |
| `jdw-billboarding-backend` | Library | Parse `.bbd`, convert to OSC, sample loading, NRT |
| `jdw-osc-lib` | Library | Shared OSC data models (`TimedOSCPacket`, etc.) |
| `jdw-osc-router` | Service | Routes OSC messages between components |
| `jdw-sequencer` | Service | Beat-synchronous sequencer |
| `jdw-sc` | Service | SuperCollider wrapper + NRT rendering |
| `jdw-songs` | Data | `.bbd` songs, synthdefs, synth templates (private) |

GitHub: `https://github.com/estrandv/<crate-name>`  
Cargo: `{ git = "https://github.com/estrandv/<crate>.git", branch = "master" }`

## Python Heritage

The Rust ecosystem replaces a fully working Python "stable alpha" — these repos
are the reference implementation and source of truth for business logic:

| Python repo | Replaced by |
|---|---|
| `jdw-pycompose` | `jdw-suite` (orchestration, NRT, live play) |
| `jdw-billboarding-lib` | `jdw-billboarding-backend` (parsing, OSC conversion) |
| `shuttle-notation-python` | shuttle parser in `jdw-billboarding-backend` |
| `jdw-helper-scripts` | `scripts/` in `jdw-suite` |
| `jdw-pycompose/songs/` + `scd/` | `jdw-songs` (private) |

All hand-written by the same author. When in doubt about behavior, diff against
the Python output. The old_method SCD exports live at
`~/tmp/nrt_bug_export/old_method/` for NRT comparison.

## Commands

| Command | Status | What it does |
|---|---|---|
| `jdw all` | Working | Launch all backends (router, sequencer, SC) |
| `jdw setup <file>` | Working | Load samples, synthdefs, effects, drones, commands |
| `jdw update <file>` | Working | Reconfigure effects/drones/commands live |
| `jdw play <file>` | Working | Send composition to sequencer |
| `jdw stop` | Working | Stop playback |
| `jdw quiet <file>` | Working | Stop + silence drones |
| `jdw nrt <file>` | Working | NRT render each track to WAV |
| `jdw terminate` | Working | Shut down suite |

## Setup Flow (matches Python)

```
send_samples → send_full_setup (synthdefs)
→ send_effects_clear → send_full_commands (routers)
→ send_effects_create → send_drones_create → beep
```

## NRT Flow

```
parse_billboard_file → get_nrt_record_bundles
→ for each track:
    1. Start Listener on incremental port (13456+)
    2. Send preload messages (/clear_nrt, /create_synthdef, /load_sample)
    3. Send preload bundles (setup + timed notes, batched per 100 packets)
    4. Send nrt_record bundle (metadata-only)
    5. wait_for_nrt() — blocks until /nrt_record_finished or timeout
    6. Drop listener, sleep 200ms, next track
```

## Config (`~/.config/jdw.toml`)

```toml
[pycompose]
bbd_root = "/path/to/jdw-pycompose"
synthdefs_scd_path = "/path/to/synthDefs.scd"
template_synths_path = "/path/to/template_synths.txt"
sample_pack_dir = "~/sample_packs"
nrt_output_dir = "~/jdw_output"     # optional, defaults to ~/jdw_output
```

## Helper Scripts

| Script | Purpose |
|---|---|
| `scripts/merge-nrt.sh` | Merge all WAVs in nrt_output_dir into /tmp/nrt_merged.wav + play |
| `scripts/release.sh` | Prompt for version, run tests, tag, push (CI builds + uploads) |

## Development Principles

### config-reminder
Never hardcode paths, ports, binary names, or magic strings. Every configurable
value lives in `~/.config/jdw.toml` with a default in the relevant crate's
`config.rs`. If you need a new value, add it to the config struct, the TOML
merge logic, and the default — then reference it from there.

The live config at `~/.config/jdw.toml` is the canonical master copy.
`assets/example.jdw.toml` is a sanitized snapshot of it (absolute paths,
no user-specific home dirs) for distribution. When adding a new key,
always add it to the live config first, then copy to the example.

### plan-tracking
When a task in `docs/TODO.md`, `docs/RELEASE_0_1.md`, or any other plan doc is
completed, update the source document. Stale docs are worse than no docs. This
includes code-level TODOs — if you fix one, mark it resolved in both the source
file and `TODO.md`.

### os-agnostic
Linux is the primary target, but new code must not assume Unix-only APIs.
Windows paths (`%APPDATA%` vs `~/.config`), process management (`pkill` vs
`taskkill`), and platform binaries (`.exe` extension) should be abstracted or
gated with `#[cfg]`. Use `std::path::Path` for path manipulation, never string
concatenation.

### tree-sitter supremacy
The tree-sitter grammar definitions in each language project are the source of
truth for the `jdw` language spec. If a behavior differs between the Rust
billboard parser and the tree-sitter parser, tree-sitter wins. Report
discrepancies as bugs in the Rust parser.

### suite as a husk
`jdw-suite` is a thin orchestrator. It must not contain parsing logic, OSC
conversion, synth loading, or any domain logic. Its sole job is to call into
library crates and microservices. If you find yourself writing business logic
in `src/client.rs` or `src/launch.rs`, move it to the appropriate crate.

### diff-first
When porting Python logic to Rust (or fixing NRT/SCD output), always start by
diffing the old_method Python output against the new Rust output. The
`scripts/compare_scds.py` tool in billboarding-backend does structural
comparison. Guessing without a diff wastes time.

### warn-free
`cargo build` and `cargo test` must produce zero warnings. Fix warnings
immediately — don't let them accumulate. Use `#[allow(dead_code)]` sparingly
and only with a comment explaining why the code is kept.

### todo-driven
Code-level `// TODO` comments must have a corresponding entry in
`docs/TODO.md`. This prevents TODOs from being forgotten in source files.
When adding a TODO in code, add it to the doc too.

### git-deps-only
Never use `path = "../other-crate"` in Cargo.toml. Always use git dependencies
with `branch = "master"` — never pin to a specific `rev` or `tag`.
Path dependencies only work on the developer's machine; pinned revs break when
the dependency is updated. crates.io releases are acceptable when available.
