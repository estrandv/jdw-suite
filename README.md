# jdw — modular music production suite

`jdw` is a single-binary facade over the JackDAW microservice ecosystem. It
launches, manages, and communicates with a suite of real-time music services
connected via OSC/UDP messaging.

## Usage

```
jdw [OPTIONS] [COMMAND]
```

Commands default to `all` when omitted.

### Launch commands (block the terminal)

| Command      | Action                                        |
|--------------|-----------------------------------------------|
| `jdw all`    | Launch router + sequencer + SC wrapper        |
| `jdw router` | Launch only the OSC message router             |
| `jdw sc`     | Launch only the SuperCollider wrapper          |
| `jdw seq`    | Launch only the beat-synchronous sequencer     |

Press Ctrl-C to stop. `jdw all` coordinates startup order and registers
router subscriptions automatically.

### Client commands (run from another terminal)

| Command              | Action                                          |
|----------------------|-------------------------------------------------|
| `jdw send <file>`    | Queue a composition for playback                 |
| `jdw stop`           | Stop all playback                                |
| `jdw setup <file>`   | Load synthdefs and samples for a composition     |
| `jdw nrt <file>`     | Non-real-time render each track to WAV           |
| `jdw terminate`      | Shut down the running suite via OSC control port |

### Options

| Flag          | Effect                     |
|---------------|----------------------------|
| `-q` / `--quiet` | Suppress info logs, errors only |

## Configuration

Central config at `~/.config/jdw.toml` (override with `$JDW_CONFIG` env var).

```toml
[suite]
control_port = 13340
control_address = "127.0.0.1"
```

## Dependencies

- **SuperCollider** (`sclang` + `scsynth`) must be on `$PATH`
- No Python required (all composition parsing is Rust-native)

## Build

```sh
cargo build --release
./target/release/jdw --help
```

## Repository

This is the master repo for the JackDAW project. Individual service crates:
- [jdw-osc-router](https://github.com/estrandv/jdw-osc-router)
- [jdw-sc](https://github.com/estrandv/jdw-sc)
- [jdw-sequencer](https://github.com/estrandv/jdw-sequencer)
- [jdw-osc-lib](https://github.com/estrandv/jdw-osc-lib)
- [jdw-billboarding-backend](https://github.com/estrandv/jdw-billboarding-backend)
