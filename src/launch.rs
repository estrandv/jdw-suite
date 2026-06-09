use std::net::{SocketAddr, UdpSocket};
use std::thread;
use std::time::Duration;

use rosc::encoder;
use rosc::{OscMessage, OscPacket, OscType};

use crate::config;

pub fn run_all(quiet: bool) {
    init_logging(quiet);

    let suite_cfg = config::load();
    let router_cfg = jdw_osc_router::config::load("");
    jdw_sc::config::init("");
    jdw_sequencer::config::Config::init("");

    let router_addr = router_cfg.bind_addr().to_string();
    let router_ip = router_cfg.bind_address.clone();
    let sc_port = jdw_sc::config::Config::get().application_in_port;
    let seq_port = jdw_sequencer::config::Config::get().application_in_port;

    let ctrl_addr = format!("{}:{}", suite_cfg.control_address, suite_cfg.control_port);
    thread::Builder::new()
        .name("control-listener".to_string())
        .spawn(move || {
            let sock = UdpSocket::bind(&ctrl_addr)
                .unwrap_or_else(|e| panic!("Failed to bind control socket on {}: {}", ctrl_addr, e));
            log::info!("Suite control listening on {}", ctrl_addr);
            let mut buf = vec![0u8; 65536];
            loop {
                match sock.recv_from(&mut buf) {
                    Ok((size, _)) => {
                        if let Ok((_, packet)) = rosc::decoder::decode_udp(&buf[..size]) {
                            if let OscPacket::Message(msg) = packet {
                                match msg.addr.as_str() {
                                    "/shutdown" => {
                                        log::info!("Shutdown requested via control channel");
                                        let pid = std::process::id() as i32;
                                        let _ = std::process::Command::new("kill")
                                            .arg("-INT")
                                            .arg(pid.to_string())
                                            .status();
                                        thread::sleep(Duration::from_secs(1));
                                        std::process::exit(0);
                                    }
                                    addr => {
                                        log::warn!("Unknown control message: {}", addr);
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        log::error!("Control listener error: {}", e);
                        break;
                    }
                }
            }
        })
        .expect("failed to spawn control-listener thread");

    thread::Builder::new()
        .name("osc-router".to_string())
        .spawn(move || {
            jdw_osc_router::run("", quiet);
        })
        .expect("failed to spawn osc-router thread");

    thread::sleep(Duration::from_millis(200));

    subscribe(
        &router_addr,
        &router_ip,
        &[
            ("/note_on_timed", sc_port),
            ("/note_on", sc_port),
            ("/bundle", sc_port),
            ("/c_set", sc_port),
            ("/play_sample", sc_port),
            ("/note_modify", sc_port),
            ("/read_scd", sc_port),
            ("/create_synthdef", sc_port),
            ("/load_sample", sc_port),
            ("/free_notes", sc_port),
            ("/clear_nrt", sc_port),
            ("/nrt_record_from_file", sc_port),
            ("/set_bpm", sc_port),
            ("/jdw_sc_event_trigger", sc_port),
            ("/bundle", seq_port),
            ("/nrt_record_finished", suite_cfg.nrt_listener_port_base as i32),
            ("/set_bpm", seq_port),
            ("/hard_stop", seq_port),
            ("/wipe_on_finish", seq_port),
        ],
    );
    log::info!("Router subscriptions registered.");

    thread::Builder::new()
        .name("sequencer".to_string())
        .spawn(move || {
            jdw_sequencer::run("", quiet);
        })
        .expect("failed to spawn sequencer thread");

    jdw_sc::run("", quiet);
}

pub fn run_router(quiet: bool) {
    init_logging(quiet);
    jdw_osc_router::run("", quiet);
}

pub fn run_sc(quiet: bool) {
    init_logging(quiet);
    jdw_sc::run("", quiet);
}

pub fn run_sequencer(quiet: bool) {
    init_logging(quiet);
    jdw_sequencer::run("", quiet);
}

fn init_logging(quiet: bool) {
    let _ = simple_logger::SimpleLogger::new()
        .with_level(if quiet {
            log::LevelFilter::Error
        } else {
            log::LevelFilter::Info
        })
        .init();

    // On panic/crash, remind about orphan cleanup. Subprocess children
    // (sclang, scsynth) are killed on drop during unwind.
    std::panic::set_hook(Box::new(|info| {
        log::error!("jdw crashed: {}", info);
        eprintln!("If sclang/scsynth are still running, run: pkill scsynth sclang");
    }));
}

fn subscribe(router_addr: &str, ip: &str, entries: &[(&str, i32)]) {
    let sock = UdpSocket::bind("127.0.0.1:0").expect("subscribe: failed to bind ephemeral socket");
    let target: SocketAddr = router_addr.parse().expect("subscribe: invalid router addr");

    for (osc_addr, port) in entries {
        let msg = OscPacket::Message(OscMessage {
            addr: "/subscribe".to_string(),
            args: vec![
                OscType::String(osc_addr.to_string()),
                OscType::String(ip.to_string()),
                OscType::Int(*port),
            ],
        });
        let buf = encoder::encode(&msg).unwrap();
        sock.send_to(&buf, target).unwrap();
    }
}
