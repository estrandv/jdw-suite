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
| `jdw terminate` | Working | Shut down suite |
| `jdw nrt <file>` | Mostly | NRT render to WAV (see known issue below) |

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
    2. Subscribe /nrt_record_finished to listener port
    3. Send preload messages (/clear_nrt, /create_synthdef, /load_sample)
    4. Send preload bundle (commands + effects at t=0)
    5. Send main nrt_record bundle (timed notes)
    6. wait_for_nrt() — blocks until /nrt_record_finished or timeout
    7. Drop listener, sleep 200ms, next track
```

## Config (`~/.config/jdw.toml`)

```toml
[pycompose]
bbd_root = "/path/to/jdw-pycompose"
synthdefs_scd_path = "/path/to/synthDefs.scd"
template_synths_path = "/path/to/template_synths.txt"
sample_pack_dir = "~/sample_packs"
```

## 🔴 BLOCKING: NRT tracks hang — `/nrt_done` never arrives from sclang

**This is NOT a timeout problem. "Timing out" is an error — the ONLY acceptable
outcome is receiving `/nrt_done`.**

The pattern: tracks where jdw-sc logs `Preloaded nrt packets: 0` never receive
`/nrt_done` from sclang. The SCD file is generated correctly (verified). jdw-sc
sends it to sclang. sclang renders but the `action:` callback never fires.

The `set_read_timeout` fix (57fc9c2) only makes the hang visible — it does NOT
fix why `/nrt_done` never arrives. That fix should be reverted or kept only as
a safety net.

**Top theory**: The SC `action: { o.sendMsg("/nrt_done", "ok"); }` in the NRT
SCD template might not fire for NRT servers. Python's SCD may use a different
mechanism. Compare the Python-generated SCD with the Rust-generated SCD for
the same track.

See `jdw-billboarding-backend/AGENTS.md` for full analysis.
