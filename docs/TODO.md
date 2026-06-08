# TODO

## Features

- [x] **Full billboard parser** — complete. All `.bbd` features: sections,
      headers, tracks, effects, commands, DEFAULT, filters, macros, shuttle
      notation, arg inheritance, operators. Parse → OSC pipeline working.
- [x] **Port Python compose logic to Rust** — complete. Sample loading,
      synthdef loading, effects/drones create, command translation, queue
      update, silence drones. Verified against Python for arena.bbd.
- [x] **NRT recording** — complete. `jdw nrt <file>` renders each track to WAV.
      Ported Python's Score class, extend_groups, preload batching,
      /nrt_record_info, /nrt_record_finished listener. See
      `jdw-billboarding-backend/docs/NRT_OVERVIEW.md`.
- [x] **Release pipeline** — CI builds Linux + macOS on tag push, attaches zips
      to GitHub Release. `scripts/release.sh` for local tagging.
- [ ] **`jdw all` idempotency** — kill existing scsynth/sclang before re-launch
- [ ] **Effect modulation during update** — send `/note_modify` for existing
      effects during `--update` (modifies running effects, not recreate them)
- [ ] **Tray icon daemon** — `jdw daemon` as a supervisor that spawns, watches,
      and restarts services, with system tray integration
- [ ] **Librarization** — both crates already have `[lib]` + `[[bin]]` targets
      with thin `main.rs` wrappers, but their library APIs are not clean:
      - **jdw-osc-router** (~1-2hrs): `run()` is a single blocking function.
        Add a non-blocking `poll()` variant.
      - **jdw-sequencer** (~3-5d): `run()` is a 231-line monolith. Break into
        composable pieces (e.g. `create_pipeline()`, `start_loop()`).
- [ ] **Packaging** — platform-specific distribution:
      - macOS: `.app` bundle, sign + notarize, Homebrew tap
      - Windows: MSI installer or winget
      - Linux: AppImage or `.deb`/`.rpm`
- [ ] **Auto-updater** — native per-platform or `self_update` crate
- [ ] **OSC control port extensions** — add `/status`, `/restart-sc`,
      `/restart-sequencer` commands to the control listener
- [ ] **GUI** — graphical interface for suite monitoring and control
- [ ] **Plugin format support** — VST3, CLAP host (long-term)
- [ ] **New input devices** — MIDI, OSC controller input as standalone services
- [ ] **Swap SuperCollider backend** — replace with VST3 host or other audio
      engine without touching sequencer or router
- [ ] **Vendor SuperCollider binaries** — bundle per-platform SC binaries so
      users don't need a system install

## Bugs

### jdw-osc-router

- [ ] `subscriber_data` is a `Vec` — O(n) scan per message. Should be
      `HashMap<String, Vec<SocketAddr>>` for O(1) lookup.

### jdw-sc

- [ ] `regex_search_node_ids` compiles `Regex` on every call. Cache compiled
      regexes.
- [ ] `regex_clear_node_ids` — same regex recompilation issue.

### jdw-sequencer

- [ ] `sequencing_daemon.rs:131` — uncertain `.clone()` on tick result.
- [ ] Multiple `Utc::now()` syscalls per tick — consolidate.

### Cross-cutting

- [ ] **BigDecimal in hot paths** — switching to integer nanoseconds (`u64`)
      would eliminate heap allocation (~100x cheaper).
- [ ] `jdw-osc-lib` `get_string_at` clones `OscType` on every access.
- [x] **time 0.3.47 MSRV** — was an issue when on stable Rust <1.88. Using
      nightly, this is no longer a problem.

## Code-Level TODOs

Scattered across repos in source files. Collected here for visibility.

### jdw-sc

| File | Line | Note |
|---|---|---|
| `nrt_record.rs` | 13 | Cheating — supposed to do full processor routine for NRT |
| `sampling.rs` | 101 | Error if duplicate buffer numbers |
| `sampling.rs` | 119 | More error handling for buffer operations |
| `osc_daemon.rs` | 88 | Keyboard handling could be its own service |
| `osc_daemon.rs` | 150 | Adapt new OSC conversion when everything is converted |
| `osc_daemon.rs` | 295 | Legacy internal OSC conversion, works but messy |
| `osc_daemon.rs` | 380 | NRT recording is synchronous, blocks everything else |
| `osc_daemon.rs` | 447 | Buffer sizes should match struct declarations |

### jdw-sequencer

| File | Line | Note |
|---|---|---|
| `master_sequencer.rs` | 187 | Thread safety concern in overshoot handling |
| `master_sequencer.rs` | 201 | Experimental start-reset — possible overshoot bug |
| `master_sequencer.rs` | 215 | `capacity()` returns wrong value (HashMap allocation, not entries) |
| `master_sequencer.rs` | 256 | More start mode tests needed |
| `master_sequencer.rs` | 311 | Debug output to paste into test |
| `bundle_model.rs` | 14 | Bundle model usage docs: receive → mark → clear |
| `sequencing_daemon.rs` | 38 | Composite payload note (end_beat for queue) |
| `sequencing_daemon.rs` | 108 | Stop request handling |
| `sequencing_daemon.rs` | 113 | State machine needed? |
| `sequencing_daemon.rs` | 131 | Unnecessary `.clone()` on tick result |

### jdw-billboarding-backend

| File | Line | Note |
|---|---|---|
| `listener.rs` | 184 | Hack: finding port from listener without proper API |

### jdw-osc-lib

| File | Line | Note |
|---|---|---|
| `osc_stack.rs` | 112 | Buffer sizes should match struct declarations |
