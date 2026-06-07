# TODO

## Features

- [ ] **NRT recording** — `jdw nrt <file> <output>` to trigger non-real-time
      rendering via the SuperCollider wrapper
- [ ] **Full billboard parser** — support full billboard format (not just
      mini-billboard subset) with all composition features.
      See [PLAN_full_billboard_parser.md](PLAN_full_billboard_parser.md) for
      staged breakdown.
- [ ] **Port Python compose logic to Rust** — eliminate Python dependency
      entirely by porting `jdw-pycompose` composition logic to Rust
- [ ] **Tray icon daemon** — `jdw daemon` as a supervisor that spawns, watches,
      and restarts services, with system tray integration
- [ ] **Librarization** — both crates already have `[lib]` + `[[bin]]` targets
      with thin `main.rs` wrappers, but their library APIs are not clean:
      - **jdw-osc-router** (~1-2hrs): `run()` is a single blocking function.
        Add a non-blocking `poll()` variant that processes one packet at a
        time (so consumers can integrate into their own event loop). Replace
        `unwrap()` panics with `Result` returns.
      - **jdw-sequencer** (~3-5d): `run()` is a 231-line monolith intertwining
        config init, logger setup, ringbuffer creation, OSC handler
        registration, and thread spawning. Break into composable pieces
        (e.g. `create_pipeline()`, `start_loop()`). Decouple from the global
        config singleton (`OnceLock`). Expose internal sequencer state via
        getters. Move logging init to `main.rs`. Remove nightly feature gates
        (`result_flattening` is stable since 1.42; `proc_macro_hygiene` and
        `decl_macro` need evaluation).
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
      `HashMap<String, Vec<SocketAddr>>` indexed by `osc_address` for O(1)
      lookup if subscriber count grows into the hundreds.

### jdw-sc

- [ ] `regex_search_node_ids` compiles `Regex` on every call (node_lookup.rs).
      Called from `/note_modify` and `/free_notes` hot paths. An LRU cache of
      compiled regexes would avoid repeated DFA compilation (10-100µs each).
- [ ] `regex_clear_node_ids` — same regex recompilation issue as above.

### jdw-sequencer

- [ ] `sequencing_daemon.rs:131` — uncertain `.clone()` on tick result.
      Investigate whether the borrow issue can be resolved to remove the clone
      of the entire collected `Vec`.
- [ ] Multiple `Utc::now()` syscalls per tick (3+ calls per tick in
      `sequencing_daemon.rs`). Consolidate — `SystemTime::now()` suffices for
      elapsed-time math without chrono.

### Cross-cutting

- [ ] **BigDecimal in hot paths** — used throughout for beat arithmetic, OSC
      time tags, gate time conversions. Every operation involves heap
      allocation and arbitrary-precision arithmetic. Switching to integer
      nanoseconds (`u64`) would eliminate allocation entirely and is ~100x
      cheaper per operation. Large refactor touching every crate.
- [ ] `jdw-osc-lib` `get_string_at` clones `OscType` on every access
      (`model.rs:81`). `some.clone().string()` clones the entire `OscType`
      enum (incl. inner `String`) because `rosc::OscType::string()` takes
      `self` by value. A manual `match` against `OscType::String(s)` would
      halve the allocation. Same pattern for `get_int_at`/`get_float_at`
      (cheaper since int/float are `Copy`, but still unnecessary enum-wrapper
      overhead).
