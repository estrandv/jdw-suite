use std::net::{SocketAddr, UdpSocket};

use rosc::encoder;
use rosc::{OscMessage, OscPacket, OscType};

use crate::config;

/// Play a confirmation beep via the `blip` synth.
/// `amp` controls volume (1.0 for setup, 0.1 for update).
fn beep(amp: f32) {
    let cfg = jdw_billboarding_backend::OscConfig::default();
    let sock = match UdpSocket::bind("127.0.0.1:0") {
        Ok(s) => s,
        Err(_) => return,
    };
    let target: SocketAddr = match cfg.router_addr.parse() {
        Ok(a) => a,
        Err(_) => return,
    };
    let msg = OscPacket::Message(OscMessage {
        addr: "/note_on_timed".to_string(),
        args: vec![
            OscType::String("blip".to_string()),
            OscType::String(format!("beep-{}", std::process::id())),
            OscType::String("0.25".to_string()),
            OscType::Int(0),
            OscType::String("freq".to_string()),
            OscType::Float(875.0),
            OscType::String("relT".to_string()),
            OscType::Float(0.2),
            OscType::String("amp".to_string()),
            OscType::Float(amp),
        ],
    });
    let buf = encoder::encode(&msg).unwrap();
    let _ = sock.send_to(&buf, target);
}

pub fn play(file: &str) {
    let bb = match jdw_billboarding_backend::parse_billboard_file(file) {
        Ok(bb) => bb,
        Err(e) => {
            eprintln!("Error parsing {}: {}", file, e);
            std::process::exit(1);
        }
    };

    let cfg = jdw_billboarding_backend::OscConfig::default();
    if let Err(e) = jdw_billboarding_backend::osc::send_full_queue_update(&bb, &cfg) {
        eprintln!("Failed to send queue update: {}", e);
        std::process::exit(1);
    }
    let track_count: usize = bb.sections.iter().map(|s| s.tracks.len()).sum();
    println!("Sent {} track(s) to the sequencer.", track_count);
}

pub fn setup(file: &str) {
    let bb = match jdw_billboarding_backend::parse_billboard_file(file) {
        Ok(bb) => bb,
        Err(e) => {
            eprintln!("Error parsing {}: {}", file, e);
            std::process::exit(1);
        }
    };

    let jdw_cfg = jdw_billboarding_backend::config::JdwConfig::load(None);
    let osc_cfg = jdw_cfg.to_osc_config();

    // Load all known SynthDefs from config paths
    let synthdefs = jdw_billboarding_backend::load_synthdefs(
        jdw_cfg.synthdefs_scd_path.as_deref(),
        jdw_cfg.template_synths_path.as_deref(),
        jdw_cfg.bbd_root.as_deref(),
    );

    // Load samples from sample pack directory (before synthdefs, matching Python order)
    let sample_pack_dir = jdw_cfg.sample_pack_dir.as_deref().unwrap_or("~/sample_packs");
    let samples = jdw_billboarding_backend::get_default_samples(sample_pack_dir);
    if !samples.is_empty() {
        if let Err(e) = jdw_billboarding_backend::osc::send_samples(&samples, &osc_cfg) {
            eprintln!("Failed to send samples: {}", e);
            std::process::exit(1);
        }
    }

    if let Err(e) = jdw_billboarding_backend::osc::send_full_setup(&synthdefs, &osc_cfg) {
        eprintln!("Failed to send setup: {}", e);
        std::process::exit(1);
    }
    if let Err(e) = jdw_billboarding_backend::osc::send_effects_clear(&osc_cfg) {
        eprintln!("Failed to clear effects: {}", e);
        std::process::exit(1);
    }
    // Commands (routers) must precede effects/drones — SC bus order is strict
    if let Err(e) = jdw_billboarding_backend::osc::send_full_commands(&bb, &osc_cfg) {
        eprintln!("Failed to send commands: {}", e);
        std::process::exit(1);
    }
    if let Err(e) = jdw_billboarding_backend::osc::send_effects_create(&bb, &osc_cfg) {
        eprintln!("Failed to create effects: {}", e);
        std::process::exit(1);
    }
    if let Err(e) = jdw_billboarding_backend::osc::send_drones_create(&bb, &osc_cfg) {
        eprintln!("Failed to create drones: {}", e);
        std::process::exit(1);
    }

    let synthdef_count = synthdefs.len();
    let synth_count = bb.sections.len();
    println!(
        "Setup sent: {} sample(s), {} SynthDef(s), {} synth section(s) in billboard.",
        samples.len(), synthdef_count, synth_count
    );

    // Confirmation beep
    beep(1.0);
}

/// Send commands (synthdefs + effects + scale, etc.) for a billboard.
/// This is equivalent to Python `--update` (configure + quiet beep).
pub fn update(file: &str) {
    let bb = match jdw_billboarding_backend::parse_billboard_file(file) {
        Ok(bb) => bb,
        Err(e) => {
            eprintln!("Error parsing {}: {}", file, e);
            std::process::exit(1);
        }
    };

    let cfg = jdw_billboarding_backend::OscConfig::default();

    // Re-send synthdefs during update (matches Python's configure which includes synthdefs)
    let jdw_cfg = jdw_billboarding_backend::config::JdwConfig::load(None);
    let synthdefs = jdw_billboarding_backend::load_synthdefs(
        jdw_cfg.synthdefs_scd_path.as_deref(),
        jdw_cfg.template_synths_path.as_deref(),
        jdw_cfg.bbd_root.as_deref(),
    );
    if let Err(e) = jdw_billboarding_backend::osc::send_full_setup(&synthdefs, &cfg) {
        eprintln!("Failed to send synthdefs during update: {}", e);
        std::process::exit(1);
    }

    if let Err(e) = jdw_billboarding_backend::osc::send_effects_clear(&cfg) {
        eprintln!("Failed to clear effects: {}", e);
        std::process::exit(1);
    }
    // Commands (routers) must precede effects/drones — SC bus order is strict
    if let Err(e) = jdw_billboarding_backend::osc::send_full_commands(&bb, &cfg) {
        eprintln!("Failed to send commands: {}", e);
        std::process::exit(1);
    }
    if let Err(e) = jdw_billboarding_backend::osc::send_effects_create(&bb, &cfg) {
        eprintln!("Failed to create effects: {}", e);
        std::process::exit(1);
    }
    if let Err(e) = jdw_billboarding_backend::osc::send_drones_create(&bb, &cfg) {
        eprintln!("Failed to create drones: {}", e);
        std::process::exit(1);
    }

    println!("Update sent. Commands configured for {}.", file);
    beep(0.1);
}

pub fn stop() {
    let cfg = jdw_billboarding_backend::OscConfig::default();
    match jdw_billboarding_backend::osc::send_stop(&cfg) {
        Ok(()) => println!("Stop signal sent."),
        Err(e) => {
            eprintln!("Failed to send stop: {}", e);
            std::process::exit(1);
        }
    }
}

pub fn quiet(file: &str) {
    let cfg = jdw_billboarding_backend::OscConfig::default();

    // Stop playback first
    if let Err(e) = jdw_billboarding_backend::osc::send_stop(&cfg) {
        eprintln!("Failed to send stop: {}", e);
        std::process::exit(1);
    }

    // Parse billboard to identify and silence drones
    let bb = match jdw_billboarding_backend::parse_billboard_file(file) {
        Ok(bb) => bb,
        Err(e) => {
            eprintln!("Error parsing {}: {}", file, e);
            std::process::exit(1);
        }
    };

    if let Err(e) = jdw_billboarding_backend::osc::send_silence_drones(&bb, &cfg) {
        eprintln!("Failed to silence drones: {}", e);
        std::process::exit(1);
    }

    println!("Quiet completed.");
}

pub fn terminate() {
    let suite_cfg = config::load();
    let addr = format!("{}:{}", suite_cfg.control_address, suite_cfg.control_port);

    let sock = match UdpSocket::bind("127.0.0.1:0") {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to open socket: {}", e);
            std::process::exit(1);
        }
    };

    let target: SocketAddr = match addr.parse() {
        Ok(a) => a,
        Err(_) => {
            eprintln!("Invalid control address: {}", addr);
            std::process::exit(1);
        }
    };

    let msg = OscPacket::Message(OscMessage {
        addr: "/shutdown".to_string(),
        args: vec![],
    });

    let buf = encoder::encode(&msg).unwrap();
    match sock.send_to(&buf, target) {
        Ok(_) => {
            println!("Shutdown signal sent to suite on {}.", addr);
            println!("(If the suite was not running, this is a no-op.)");
        }
        Err(e) => {
            eprintln!("Failed to send shutdown: {}", e);
            std::process::exit(1);
        }
    }
}
