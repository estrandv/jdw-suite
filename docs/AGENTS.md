# AGENTS.md — jdw-suite

## Architecture

**jdw-suite** is the end-user entry point. It produces the `jdw` binary — the
only CLI the user ever needs to run. Everything else is a library consumed by it:

| Crate | Role |
|---|---|
| `jdw-suite` | `jdw` binary — launch backends, send songs, manage suite |
| `jdw-billboarding-backend` | Library — parse `.bbd` files, convert to OSC, sample loading |
| `jdw-osc-lib` | Library — `TimedOSCPacket` model |
| `jdw-osc-router` | Service — routes OSC messages between components |
| `jdw-sequencer` | Service — beat-synchronous sequencer |
| `jdw-sc` | Service — SuperCollider wrapper |

The billboarding backend is a **library only** (no binary, no install.sh). It is
pulled in via `Cargo.toml` git dependency and consumed by `client.rs`.

## Installation

```bash
jdw-suite/install.sh    # builds and installs the `jdw` binary
```

## Typical Workflow

1. `jdw all` — start router + sequencer + jdw-sc
2. `jdw setup <song.bbd>` — load samples + synthdefs + create effects + drones + commands
3. `jdw update <song.bbd>` — reconfigure effects/drones/commands (live, without restart)
4. `jdw play <song.bbd>` — send composition to sequencer (queue update)
5. `jdw stop` — stop playback
6. `jdw quiet <song.bbd>` — stop + silence drones
7. `jdw terminate` — shut down suite

## Commands

| Command | What it does |
|---|---|
| `jdw all` | Launch all backends (router, sequencer, SC) |
| `jdw router` | Launch only the OSC router |
| `jdw sc` | Launch only the SuperCollider wrapper |
| `jdw sequencer` | Launch only the sequencer |
| `jdw setup <file>` | Load samples, synthdefs, create effects/drones, send commands |
| `jdw update <file>` | Reconfigure effects/drones/commands for a live song |
| `jdw play <file>` | Send composition to the sequencer (queue update) |
| `jdw stop` | Stop playback |
| `jdw quiet <file>` | Stop playback + silence drones |
| `jdw terminate` | Shut down the suite |

## Config (`~/.config/jdw.toml`)

Required `[pycompose]` section:
```toml
[pycompose]
bbd_root = "/path/to/jdw-pycompose"
synthdefs_scd_path = "/path/to/synthDefs.scd"
template_synths_path = "/path/to/template_synths.txt"
sample_pack_dir = "~/sample_packs"       # default: ~/sample_packs
first_buffer_index = 100                  # default: 100
```

## Port Convention

| Port  | Service         |
|-------|-----------------|
| 13339 | OSC Router      |
| 13331 | jdw-sc          |
| 14441 | Sequencer       |
| 13340 | Suite control   |
