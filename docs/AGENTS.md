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

## Known Issue — NRT tracks hang on `Preloaded nrt packets: 0`

Some NRT tracks hang. The jdw-sc log shows `Preloaded nrt packets: 0` followed by
a hang where `/nrt_done` never arrives. After timeout, jdw-sc sends FAILURE.

The listener eventually times out (NRT CLI says "Timed out"), but jdw-sc DID send
`/nrt_record_finished "FAILURE"` — it was just lost (listener port race?).

See `jdw-billboarding-backend/AGENTS.md` for detailed pro/con analysis.

### What we know:
- Tracks with `Preloaded nrt packets: 0` have 168-640 notes in the main bundle
- The main bundle IS being sent and IS being processed by jdw-sc
- jdw-sc generates the SCD file, sends to sclang, awaits `/nrt_done`
- sclang appears to render the SCD but never sends `/nrt_done`
- jdw-sc's `await_internal_response` had a bug (fixed: set_read_timeout)
- Sample filtering fixed (10x fewer buffer loads per SCD)

### To investigate:
- Does the `set_read_timeout` fix resolve the hang?
- Does Python also have empty-preload tracks, and do they work?
- Is `server_osc_socket_name` ("o") correct for sclang NRT mode?
- Is `/nrt_done` being sent by sclang but getting lost in routing?
