use std::net::{SocketAddr, UdpSocket};

use rosc::encoder;
use rosc::{OscMessage, OscPacket};

use crate::config;

pub fn send(file: &str) {
    let bb = match jdw_billboarding_backend::parse_billboard_file(file) {
        Ok(bb) => bb,
        Err(e) => {
            eprintln!("Error parsing {}: {}", file, e);
            std::process::exit(1);
        }
    };

    let cfg = jdw_billboarding_backend::OscConfig::default();
    if let Err(e) = jdw_billboarding_backend::osc::send_queue_update(&bb, &cfg) {
        eprintln!("Failed to send queue update: {}", e);
        std::process::exit(1);
    }
    println!("Sent {} track(s) to the sequencer.", bb.tracks.len());
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

pub fn setup(file: &str) {
    let bb = match jdw_billboarding_backend::parse_billboard_file(file) {
        Ok(bb) => bb,
        Err(e) => {
            eprintln!("Error parsing {}: {}", file, e);
            std::process::exit(1);
        }
    };

    let cfg = jdw_billboarding_backend::OscConfig::default();
    if let Err(e) = jdw_billboarding_backend::osc::send_setup(&bb, &cfg) {
        eprintln!("Failed to send setup: {}", e);
        std::process::exit(1);
    }
    println!("Setup sent for {} track(s).", bb.tracks.len());
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
