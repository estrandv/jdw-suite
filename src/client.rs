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

/// Terminate and re-launch the suite.
pub fn restart() {
    terminate();
    std::thread::sleep(std::time::Duration::from_millis(2000));
    // Spawn jdw all in the background
    let exe = std::env::current_exe().unwrap_or_else(|_| "jdw".into());
    std::process::Command::new(exe)
        .arg("all")
        .spawn()
        .expect("failed to restart suite");
    println!("Suite restarting...");
}

/// Non-real-time recording: render a composition to a WAV file.
pub fn nrt_record(file: &str) {
    let bb = match jdw_billboarding_backend::parse_billboard_file(file) {
        Ok(bb) => bb,
        Err(e) => {
            eprintln!("Error parsing {}: {}", file, e);
            std::process::exit(1);
        }
    };

    let jdw_cfg = jdw_billboarding_backend::config::JdwConfig::load(None);
    let osc_cfg = jdw_cfg.to_osc_config();

    let synthdefs = jdw_billboarding_backend::load_synthdefs(
        jdw_cfg.synthdefs_scd_path.as_deref(),
        jdw_cfg.template_synths_path.as_deref(),
        jdw_cfg.bbd_root.as_deref(),
    );

    let sample_pack_dir = jdw_cfg.sample_pack_dir.as_deref().unwrap_or("~/sample_packs");
    let samples = jdw_billboarding_backend::get_default_samples(sample_pack_dir);

    let nrt_output_dir = jdw_cfg.nrt_output_dir.as_deref().unwrap_or("~/jdw_output");
    let nrt_output_dir = nrt_output_dir.replacen("~", &std::env::var("HOME").unwrap_or_else(|_| ".".into()), 1);
    let bundles = jdw_billboarding_backend::get_nrt_record_bundles(&bb, &synthdefs, &samples, &nrt_output_dir);

    let sock = std::net::UdpSocket::bind("127.0.0.1:0").expect("Failed to bind UDP socket");

    let mut listener_port: u16 = 13456;

    for info in &bundles {
        println!("Recording track: {}", info.track_name);

        // Start listener BEFORE sending main bundle (race: jdw-sc responds fast)
        let listener = match jdw_billboarding_backend::Listener::start(listener_port) {
            Ok(l) => l,
            Err(_e) => {
                // Try next port if this one is busy
                listener_port += 1;
                jdw_billboarding_backend::Listener::start(listener_port)
                    .unwrap_or_else(|e2| {
                        eprintln!("  Failed to start listener: {}", e2);
                        std::process::exit(1);
                    })
            }
        };

        let actual_port = listener_port;
        listener_port += 1;

        // Subscribe to /nrt_record_finished on the router (before sending)
        let sub_msg = rosc::OscPacket::Message(rosc::OscMessage {
            addr: "/subscribe".to_string(),
            args: vec![
                rosc::OscType::String("/nrt_record_finished".to_string()),
                rosc::OscType::String("127.0.0.1".to_string()),
                rosc::OscType::Int(actual_port as i32),
            ],
        });
        let _ = send_osc_json(&sock, &osc_cfg.router_addr, &sub_msg);
        std::thread::sleep(std::time::Duration::from_millis(50));

        eprintln!("  preload_msgs: {} messages", info.preload_messages.len());
        for msg in &info.preload_messages {
            if let Err(e) = send_osc_json(&sock, &osc_cfg.router_addr, msg) {
                eprintln!("  preload msg send error: {}", e);
            }
            std::thread::sleep(std::time::Duration::from_millis(5));
        }

        eprintln!("  preload_bundles: {} bundles", info.preload_bundles.len());
        for bundle in &info.preload_bundles {
            if let Err(e) = send_osc_json(&sock, &osc_cfg.router_addr, bundle) {
                eprintln!("  preload bundle send error: {}", e);
            }
            std::thread::sleep(std::time::Duration::from_millis(5));
        }

        // NRT bundle is metadata-only (all timed data in preload), always fits in UDP
        let nrt_buf = match rosc::encoder::encode(&info.nrt_bundle) {
            Ok(b) => b,
            Err(e) => { eprintln!("  nrt_record encode error: {}", e); return; }
        };
        eprintln!("  nrt_record bundle size: {} bytes", nrt_buf.len());

        let nrt_target: std::net::SocketAddr = match osc_cfg.router_addr.parse() {
            Ok(a) => a,
            Err(e) => { eprintln!("  nrt_record addr parse error: {}", e); return; }
        };
        if let Err(e) = sock.send_to(&nrt_buf, nrt_target) {
            eprintln!("  nrt_record bundle send error: {}", e);
        }

        println!("  NRT bundle sent, awaiting response...");

        if listener.wait_for_nrt() {
                match listener.get_response() {
                    Some((status, filename)) => {
                        println!("  NRT complete: {} → {}", status, filename);
                    }
                    None => {
                        eprintln!("  Warning: got response but couldn't parse");
                    }
                }
            } else {
                eprintln!("  Timed out waiting for NRT completion");
            }
        // Explicitly drop listener and let port release before next track
        drop(listener);
        std::thread::sleep(std::time::Duration::from_millis(200));
    }

    println!("NRT recording finished. {} track(s) processed.", bundles.len());
}

fn send_osc_json(sock: &std::net::UdpSocket, addr: &str, packet: &rosc::OscPacket) -> Result<(), String> {
    let buf = rosc::encoder::encode(packet).map_err(|e| format!("encode: {}", e))?;
    let target: std::net::SocketAddr = addr.parse().map_err(|e| format!("addr: {}", e))?;
    sock.send_to(&buf, target).map_err(|e| format!("send: {}", e))?;
    Ok(())
}
