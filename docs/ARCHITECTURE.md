# Architecture

## Overview

JackDAW is a modular music production suite built as a set of independent
services that communicate exclusively via OSC over UDP. Each service is a
standalone Rust crate with its own repository. The `jdw` binary (`jdw-suite`
repo) provides a single entry point that can launch, manage, and communicate
with the full stack.

```
┌───────────────┐     OSC/UDP      ┌──────────────┐
│  jdw-sequencer│ ────────────────>│              │
└───────────────┘                  │              │
                                   │ jdw-osc-router│
┌───────────────┐     OSC/UDP      │              │
│   jdw-sc      │ <────────────────│              │
│ (SuperCollider│                  └──────┬───────┘
│   wrapper)    │                         │
└───────────────┘                         │ OSC/UDP
                                   ┌──────┴───────┐
                                   │ jdw-billboard│
                                   │ -ing-backend │
                                   │ (lib crate)  │
                                   └──────────────┘
```

## Principles

- **Audio latency is critical** — no Docker, no web views, no Electron
- **Modularity is a superpower** — the IPC/message bus architecture lets
  components be swapped, debugged, and restarted independently
- **Zero runtime dependencies** beyond what we bundle — no Python, no
  external databases

## Components

### jdw-osc-router

A transparent OSC message broker. Services subscribe to OSC address patterns;
the router forwards matching messages to all subscribers. This decouples
senders from receivers — the sequencer, SuperCollider, and client tools all
communicate through the router without knowing each other's addresses.

Default bind: `127.0.0.1:13339`

### jdw-sc

SuperCollider wrapper that manages `sclang` and `scsynth` processes. Receives
OSC messages from the router and translates them into SuperCollider commands
(note on/off, sample playback, synthdef loading, NRT recording). Also
forwards SC events back into the OSC bus.

### jdw-sequencer

Beat-synchronous sequencer that reads timed note events from a ring buffer
and dispatches them to the router on each tick. Supports multiple sequencing
modes (real-time, NRT). The sequencer is the clock source for the suite.

### jdw-osc-lib

Shared library crate with OSC data models, protocol helpers, and socket
utilities. Used by all Rust services.

### jdw-billboarding-backend

Rust-native composition parser. Reads mini-billboard files
(`trackname:synthname arg=val (shuttle notation)`) and converts them to OSC
messages for the suite. Contains a hand-written Shuttle Notation parser
(atomic notes, sections, alternations, repeats, args) and OSC message
builders for queue update, setup, and stop.

### jdw-suite

The single-binary facade. Provides:
- **Launch commands** — `all`, `router`, `sc`, `sequencer` — that spawn
  services in threads and coordinate startup
- **Client commands** — `send`, `stop`, `setup`, `terminate` — that send
  OSC messages to a running suite
- **OSC control listener** — a thread that listens on a configurable control
  port (default `127.0.0.1:13340`) for management commands like `/shutdown`

## IPC Protocol

All inter-process communication uses OSC (Open Sound Control) over UDP.
Messages flow through `jdw-osc-router`, which implements a publish-subscribe
pattern. The protocol is loosely typed — OSC addresses and argument positions
define message semantics.

Several crates also communicate bidirectionally with SuperCollider's `sclang`
process over TCP sockets.

## Configuration

Central configuration file at `~/.config/jdw.toml` (override with `$JDW_CONFIG`
env var). Each service reads its own section from this file via a config
module or builder pattern. Compiled-in defaults exist for every value; the
config file overrides them. Apps error/exit if the file is expected but
missing.

## Repo Organization

Each component lives in its own GitHub repository with independent versioning
and CI. The `jdw-suite` repo depends on the others via git dependencies in
`Cargo.toml`:

```
github/jdw-osc-lib              # shared lib crate
github/jdw-osc-router            # router binary + lib
github/jdw-sc                    # SuperCollider wrapper binary + lib
github/jdw-sequencer             # sequencer binary + lib
github/jdw-billboarding-backend  # composition parser lib
github/jdw-suite                 # single-binary CLI + packaging (master)
```

## Port Convention

| Service             | Port(s)                |
|---------------------|------------------------|
| OSC Router          | 13339                  |
| jdw-sc              | 13331 (in), 13336 (sclang), 13337 (scsynth) |
| Sequencer           | 14441 (in), 14444 (out)|
| Suite Control       | 13340                  |
