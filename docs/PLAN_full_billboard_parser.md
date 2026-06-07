# Plan: Full Billboard Parser

**Goal:** Replace the flat mini-billboard parser with a full parser that
handles the complete Billboard Notation spec (per tree-sitter-jdw-billboarding).

## Data Model

New types (in `jdw-billboarding-backend/src/billboard.rs`):

```rust
pub struct Billboard {
    pub sections: Vec<SynthSection>,
    pub filters: Vec<Vec<String>>,    // first unbroken chain
    pub commands: Vec<BillboardCommand>,
    pub default_args: HashMap<String, f64>,
}

pub struct SynthSection {
    pub header: SynthHeader,
    pub tracks: Vec<TrackDefinition>,
    pub effects: Vec<EffectDefinition>,
}

pub struct SynthHeader {
    pub instrument: String,
    pub is_drone: bool,        // DR_ prefix
    pub is_sampler: bool,      // SP_ prefix
    pub is_selected: bool,     // *@
    pub group: Option<String>,
    pub default_args: HashMap<String, f64>,
    pub pad_config: Vec<(u32, u32)>,  // sampler only: index:sample_id
}

pub struct TrackDefinition {
    pub content: String,                // raw shuttle notation
    pub group_override: Option<String>,
    pub arg_overrides: HashMap<String, (char, f64)>,  // operator+value
    pub index: usize,                   // position in section
    pub enabled: bool,
}

pub struct EffectDefinition {
    pub effect_type: String,
    pub id: String,
    pub args: HashMap<String, f64>,
}

pub struct BillboardCommand {
    pub address: String,
    pub context: CommandContext,
    pub args: Vec<String>,
}

pub enum CommandContext { All, Update, Queue }
```

## Stages

### ✓ Stage 1 — Classifier (done — 253c267)
- Implemented `classify_content()` → `LineType` enum
- `join_continuations()` — backslash-newline line joining
- `split_inline_comment()` — inline `#` comment extraction
- `classify_source()` — full pipeline: join → split → classify
- 26 tests, all passing

### ✓ Stage 2 — Section Grouper (done — 75c2128)
- `SectionGroup` struct — header + tracks + effects + comments
- `GroupedBillboard` struct — filters, default_statement, commands, sections, orphans
- `group_sections()` — walks classified lines, groups tracks/effects under headers
- Handles filter chain break rules, orphan detection, multi-section
- 12 new tests = 48 total

### ✓ Stage 3 — Low-Level Parsers (done — a584f0f)
- `SynthHeaderData`, `TrackMeta`, `EffectData`, `CommandData`, `FilterData` structs
- `parse_arg_list()` — handles `key=val`, operators (`+`, `-`, `*`), and
  implicit `<name><number>` splitting (e.g. `amp0.5` → name=`amp`, val=`0.5`)
- `parse_synth_header()` — `[@|*@][SP_|DR_]name[:group] [args] [pad_config]`
- `parse_track_metadata()` — `<group[;arg1,arg2]>`
- `parse_effect()` — `€type:id [args]`
- `parse_command()` — `[COMMAND_TYPE] /address [args...]`
- `parse_filter()` — `>>> name1 name2 ...`
- Convenience: `parse_all_headers()`, `parse_tracks_metadata()`,
  `parse_section_effects()`, `parse_all_commands()`, `parse_all_filters()`
- 22 new tests = 70 total

### Stage 2 — Section Grouper
- Walk classified lines to:
  - Collect first unbroken chain of group filters
  - Find the last `DEFAULT` statement
  - Collect all commands
  - Group tracks + effects under their synth header into `SynthSection` vec
- Commented tracks must still increment index
- Empty lines ignored, comments preserved for indexing
- Write tests for section grouping

### Stage 3 — Low-Level Parsers
- Parse synth header line: `[@|*@][SP_|DR_]instrument[:group] [args] [pad_config]`
- Parse track metadata: `<group[;arg1=val1,arg2=val2]>`
- Parse effect definition: `€type:id [arg1=val1,arg2=val2]`
- Parse command: `[COMMAND_TYPE] /address arg1 arg2`
- Parse DEFAULT: `DEFAULT arg1=val1,arg2=val2`
- Parse group filter: `>>> name1 name2 ...`
- Write tests for each sub-parser

### Stage 4 — Billboard Construction
- Combine Stages 1-3 into `parse(source: &str) -> Result<Billboard, String>`
- Resolve argument inheritance chain: DEFAULT → SynthHeader → Track → Element
- Apply argument override operators (`+`, `-`, `*`, `=`)
- Create `Billboard` object with sections, filters, commands, defaults
- Write integration tests against real .bbd file examples (from the spec)

### Stage 5 — OSC Conversion
- Update `osc.rs` to handle the full `Billboard` type:
  - Section-aware queue updates (send per-section bundles)
  - Effect creation OSC messages
  - Command execution (`/set_bpm`, `/set_scale`, `/transpose`)
  - Drone section setup (force amp=0, create drone effects per track)
  - Sampler pad configuration
- Keep backward compat with mini-billboard for transition
- Write tests

### Stage 6 — Integration
- Update `jdw-suite/src/client.rs` to use new OSC functions
- Wire commands into `jdw send` and `jdw setup`
- Test end-to-end with a real .bbd file against the running suite
- Update `jdw-suite/docs/TODO.md` — mark feature complete

## Session Notes

- **Drop location marker:** Always start a session by reading this file and
  `src/billboard.rs` to re-establish context. The next un-staged item is
  where you left off.
- **Commit after each stage** even if imperfect — this is designed for
  incremental progress.
- **Each stage produces compilable+testable code** before moving on. Never
  leave a stage half-finished.
