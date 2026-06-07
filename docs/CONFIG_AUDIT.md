# Configuration Audit — All Hardcoded Values

## Central Config Migration

Central file: `~/.config/jdw.toml` (override via `$JDW_CONFIG` env var)

Lookup order: compiled-in defaults → `[app]` section in `~/.config/jdw.toml` → per-app `config.toml`

The central config is **required** — apps error/exit if `~/.config/jdw.toml` does not exist.

| App | Language | Status |
|-----|----------|--------|
| jdw-osc-router | Rust | ✅ |
| jdw-sc | Rust | ✅ |
| jdw-sequencer | Rust | ✅ |
| jdw-pycompose | Python | ✅ |
| jdw-osc-lib | Rust | N/A (builder method, no central) |
| jdw-helper-scripts | Bash | ✅ (reads via Python from central TOML) |

## Legend

| Prefix | Meaning |
|--------|---------|
| (P) | Port / address |
| (F) | Flag / boolean / mode enum |
| (S) | Size / buffer / memory |
| (D) | Duration / timeout |
| (N) | Numeric constant / start value |
| (O) | OSC address string |
| (X) | Path |
| (E) | Error / boundary (likely a bug) |

---

## jdw-osc-router ✅

All marked items ported to `config.toml`.

### src/main.rs
| Mark | Line | Value | Notes |
|------|------|-------|-------|
| DONE | 17 | `127.0.0.1:13339` | (P) Bind address → config |
| DONE | 27 | `[0u8; 333000]` | (S) Receive buffer → config |
| ☐ | 46 | `"/subscribe"`, `"/unsubscribe"` | (O) Subscription addresses |
| ☐ | 108 | `"/bundle"` | (O) Bundle routing address |

### python/manual_subscriptions.py
| Mark | Line | Value | Notes |
|------|------|-------|-------|
| DONE | 5 | `"127.0.0.1"`, `13339` | (P) Router address — config.toml |
| DONE | 8-11 | `13331`, `14441`, `17777`, `13456` | (P) Subscriber ports — config.toml |
| ☐ | 12-49 | ~25 `/subscribe` calls | (O) Subscription OSC addresses |
| DONE | 52 | `12367` | (P) jdw-sampler port (commented) |

### python/throughput_test.py
| Mark | Line | Value | Notes |
|------|------|-------|-------|
| o | 21-22 | `HOST`, `ROUTER_PORT` | (P) Test target |
| o | 37 | `65535` | (S) Recv buffer |
| ☐ | 65 | `SO_RCVBUF = 4MB` | (S) Socket buffer |
| ☐ | 152-156 | Default msg count, sweep values | (N) Test defaults |

### run.sh
| Mark | Line | Value | Notes |
|------|------|-------|-------|
| DONE | 10 | `~/mypython/bin/python` | (X) Python path — config.toml |

---

## jdw-sc

### src/config.rs
| Mark | Line | Value | Notes |
|------|------|-------|-------|
| DONE | 7 | `LevelFilter::Debug` | (F) Log level → config |
| DONE | 8 | `"127.0.0.1"` | (P) Application IP → config |
| ☐ | 9 | `"o"` | (N) SC socket name |
| ☐ | 10 | `"s"` | (N) SC server name |
| DONE | 12 | `30` | (D) SC read timeout → config |
| DONE | 15 | `13338` | (P) SC server → app → config |
| DONE | 16 | `13336` | (P) sclang in → config |
| DONE | 17 | `13337` | (P) scsynth in → config |
| DONE | 18 | `13339` | (P) Router out → config |
| DONE | 19 | `13331` | (P) jdw-sc in → config |
| DONE | 21 | `2000000` | (E) Memory → config |

### src/main.rs
| Mark | Line | Value | Notes |
|------|------|-------|-------|
| DONE | 71 | `Duration::from_secs(10)` | (D) Init wait timeout → config |
| DONE | 83 | `sample_pack_dir` from `~/sample_packs` | (X) Sample pack path → config |
| o | 88 | `"sampler.scd"` | (X) Sampler template filename |
| ☐ | 112 | `[130.81, 146.83, 196.00]` | (N) Beep frequencies |
| ☐ | 114 | `125ms` | (D) Beep spacing |

### src/sc_process_management.rs
| Mark | Line | Value | Notes |
|------|------|-------|-------|
| DONE | 33 | `"temp"` | (X) Temp dir → config |
| DONE | 47 | `"sclang"` | (X) Binary → config |
| DONE | 231 | `10ms` | (D) Poll sleep → config |

### src/osc_daemon.rs
| Mark | Line | Value | Notes |
|------|------|-------|-------|
| ☐ | 31 | `"batch-send"` | (O) Funnel bundle tag |
| DONE | 56 | `120` | (N) Default BPM → config |
| DONE | 436 | `[0u8; 333072]` | (S) Receive buffer → config |
| DONE | 377 | `10s` | (D) NRT done timeout → config |

### src/osc_model.rs
| Mark | Line | Value | Notes |
|------|------|-------|-------|
| ☐ | 62 | `4` | (N) `/note_on_timed` min args |
| ☐ | 116 | `3` | (N) `/note_on` min args |
| ☐ | 172 | `5` | (N) `/play_sample` min args |
| ☐ | Various | OSC address strings | (O) All expected message addresses |

### src/node_lookup.rs
| Mark | Line | Value | Notes |
|------|------|-------|-------|
| DONE | 21 | `100` | (N) First scsynth node ID → config |

### src/sampling.rs
| Mark | Line | Value | Notes |
|------|------|-------|-------|
| DONE | 38 | `44100 * 8.0` | (S/N) Sample buffer frames → config |
| DONE | 38 | `2` | (N) Number of channels → config |

### src/internal_osc_conversion.rs
| Mark | Line | Value | Notes |
|------|------|-------|-------|
| DONE | 39-40 | `0, 0` | (N) Group ID and placement → config |
| ☐ | 88 | `60` | (N) Seconds per minute |
| ☐ | 230 | `"sampler"` | (N) Sample synth name |

### python/ scripts (7 files)
| Mark | Line | Value | Notes |
|------|------|-------|-------|
| DONE | All | `"127.0.0.1:13331"` | (P) Target jdw-sc → python/config.py |
| ☐ | All | Various test data | (N/O) Hardcoded test message args |

### scd templates
| Mark | File | Value | Notes |
|------|------|-------|-------|
| o | nrt_record template | `2, 2, 48000` | (N) Audio format (channels, sample rate) |
| ☐ | nrt_record template | `"wav"`, `"int16"` | (F) File format |
| ☐ | start_server template | `"/init"` | (O) Handshake address |

---

## jdw-sequencer

### src/config.rs
| Mark | Line | Value | Notes |
|------|------|-------|-------|
| DONE | 7 | `LevelFilter::Info` | (F) Log level → config |
| DONE | 8 | `"127.0.0.1"` | (P) IP → config |
| DONE | 10 | `14441` | (P) Sequencer in → config |
| DONE | 13 | `13339` | (P) Router out → config |
| DONE | 16 | `14444` | (P) Outgoing socket bind → config |
| DONE | 19 | `5000` (5ms) | (D) Tick time → config |
| DONE | 22-31 | Start/reset mode enums | (F) Mode enums → config integers |
| DONE | 34 | `REAL_TIME_MODE = true` | (F) Bundle wrapping → config |
| DONE | 36 | `MIDI_SYNC = false` | (F) MIDI sync → config |

### src/main.rs
| Mark | Line | Value | Notes |
|------|------|-------|-------|
| DONE | 58 | `100` | (N) Ring buffer capacity → config |
| DONE | 74 | `120` | (N) Default BPM → config |

### src/osc_communication.rs
| Mark | Line | Value | Notes |
|------|------|-------|-------|
| DONE | 19 | `333072` | (S) Buffer size constant → removed (unused) |

### src/midi_utils.rs
| Mark | Line | Value | Notes |
|------|------|-------|-------|
| o | 12 | `60.0`, `1000000.0` | (N) Time conversion constants |
| o | 22-23 | `60`, `1000000000.0` | (N) Nanosecond conversion |

---

## jdw-osc-lib

### src/model.rs
| Mark | Line | Value | Notes |
|------|------|-------|-------|
| ☐ | 135 | `"/bundle_info"` | (O) Protocol standard |
| ☐ | 201 | `"timed_msg"` | (O) Bundle tag |
| ☐ | 208 | `"/timed_msg_info"` | (O) Info address |

### src/osc_stack.rs
| Mark | Line | Value | Notes |
|------|------|-------|-------|
| DONE | 99 | `[0u8; 333072]` | (S) Receive buffer → builder method with default |

---

## jdw-helper-scripts

| Mark | File | Value | Notes |
|------|------|-------|-------|
| DONE | All scripts | `/home/estrandv/mypython/bin/python` | (X) Python path → config.sh |
| DONE | All scripts | `/home/estrandv/programming/jdw-pycompose/run.py` | (X) pycompose path → config.sh |
| DONE | restart-background-apps.sh | `~/programming/jdw-sequencer/` | (X) Project path → config.sh |
| DONE | restart-background-apps.sh | `~/programming/jdw-osc-router/` | (X) Project path → config.sh |
| DONE | restart-collider.sh | `~/programming/jdw-sc` | (X) Project path → config.sh |
| ☐ | kill-stack.sh | process names | (N) pkill targets |

---

## jdw-pycompose

### run.py
| Mark | Line | Value | Notes |
|------|------|-------|-------|
| DONE | 7 | `BBD_ROOT` absolute song path | (X) Song directory → config |

### billboard_running.py
| Mark | Line | Value | Notes |
|------|------|-------|-------|
| DONE | 32 | `"127.0.0.1:13339"` | (P) Router target → config |
| DONE | 42 | `0.005` | (D) Inter-message delay → config |
| DONE | 71 | `0.001` | (D) Configure delay → config |
| DONE | 94 | `0.005` | (D) NRT delay → config |
| DONE | 105 | `0.005` | (D) Update delay → config |

### listener.py
| Mark | Line | Value | Notes |
|------|------|-------|-------|
| o | 14 | `("127.0.0.1", 13456)` | (P) Listener address |
| o | 23 | `5` | (N) Max wait iterations |
| o | 27 | `1s` | (D) Wait loop sleep |

### file_utilities.py
| Mark | Line | Value | Notes |
|------|------|-------|-------|
| DONE | 31 | `100` | (N) First buffer index → config |
| ☐ | 66-76 | Category matchers | (N) Keyword → category mapping |
| DONE | 90 | SynthDefs absolute path | (X) SCD file → config |
| DONE | 115 | `"~/sample_packs"` | (X) Sample directory → config |

### compile_scd.py
| Mark | Line | Value | Notes |
|------|------|-------|-------|
| DONE | 58 | common_macros absolute path | (X) Macros file → config |
| ☐ | 68 | `"scd-templating/template_synths.txt"` | (X) Relative template path |
