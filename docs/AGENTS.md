# AGENTS.md — jdw-suite

## Architecture

**jdw-suite** is the end-user entry point. The `jdw` binary provides all CLI commands.

| Crate | Role |
|---|---|
| `jdw-suite` | `jdw` binary — launch backends, send songs, manage suite |
| `jdw-billboarding-backend` | Library — parse `.bbd`, convert to OSC, sample loading, NRT |
| `jdw-osc-lib` | Library — `TimedOSCPacket` model |
| `jdw-osc-router` | Service — routes OSC messages between components |
| `jdw-sequencer` | Service — beat-synchronous sequencer |
| `jdw-sc` | Service — SuperCollider wrapper + NRT rendering |

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
