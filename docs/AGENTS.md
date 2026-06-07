# AGENTS.md — jdw-suite

## Typical Workflow

1. `jdw all` — start router + sequencer + jdw-sc
2. `jdw setup <song.bbd>` — load synthdefs, samples, set config
3. `jdw send <song.bbd>` — queue notes live
4. `jdw stop` — stop playback
5. `jdw terminate` — shut down suite

## Port Convention

| Port  | Service         |
|-------|-----------------|
| 13339 | OSC Router      |
| 13331 | jdw-sc          |
| 14441 | Sequencer       |
| 13340 | Suite control   |

## Config Lookup Order

Compiled-in defaults → `~/.config/jdw.toml` → per-app `config` module init.
Central config is required; apps error/exit if `~/.config/jdw.toml` does not
exist (except suite control which gracefully defaults).
