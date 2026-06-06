#![feature(result_flattening, proc_macro_hygiene, decl_macro)]

use std::net::{SocketAddr, UdpSocket};
use std::thread;
use std::time::Duration;

use rosc::encoder;
use rosc::{OscMessage, OscPacket, OscType};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let quiet = args.iter().any(|a| a == "-q" || a == "--quiet");

    // Initialise logging once for the whole process.
    // Individual crate run() calls attempt the same init and silently ignore
    // the "already initialised" error, so this is safe.
    simple_logger::SimpleLogger::new()
        .with_level(if quiet {
            log::LevelFilter::Error
        } else {
            log::LevelFilter::Info
        })
        .init()
        .unwrap();

    // Load each crate's config up front (central ~/.config/jdw.toml, no local
    // override) so we can read ports without hardcoding them. Each crate's
    // run() will call init() again; OnceLock silently ignores the second set.
    let router_cfg = jdw_osc_router::config::load("");
    jdw_sc::config::init("");
    jdw_sequencer::config::Config::init("");

    let router_addr = router_cfg.bind_addr().to_string();
    let router_ip   = router_cfg.bind_address.clone();
    let sc_port     = jdw_sc::config::Config::get().application_in_port;
    let seq_port    = jdw_sequencer::config::Config::get().application_in_port;

    // Each run() blocks its thread indefinitely.
    // Router and sequencer get background threads; SC blocks the main thread
    // (it also owns the Ctrl-C handler that terminates the process).

    let router_quiet = quiet;
    thread::Builder::new()
        .name("osc-router".to_string())
        .spawn(move || {
            jdw_osc_router::run("", router_quiet);
        })
        .expect("failed to spawn osc-router thread");

    // Give the router a moment to bind its UDP socket before sending subscriptions.
    thread::sleep(Duration::from_millis(200));

    // Wire jdw-sc and jdw-sequencer as router subscribers.
    // This is the suite's responsibility: it is the only place that knows which
    // applications exist, which ports they listen on, and how they should be
    // connected. The router itself is a dumb message bus with no knowledge of
    // the wider ecosystem.
    subscribe(
        &router_addr,
        &router_ip,
        &[
            // jdw-sc — receives note, sample, synthdef, and control messages
            ("/note_on_timed",        sc_port),
            ("/note_on",              sc_port),
            ("/bundle",               sc_port),
            ("/c_set",                sc_port),
            ("/play_sample",          sc_port),
            ("/note_modify",          sc_port),
            ("/read_scd",             sc_port),
            ("/create_synthdef",      sc_port),
            ("/load_sample",          sc_port),
            ("/free_notes",           sc_port),
            ("/clear_nrt",            sc_port),
            ("/set_bpm",              sc_port),
            ("/jdw_sc_event_trigger", sc_port),
            // jdw-sequencer — receives playback control and note queue messages
            ("/bundle",               seq_port),
            ("/set_bpm",              seq_port),
            ("/hard_stop",            seq_port),
            ("/wipe_on_finish",       seq_port),
        ],
    );
    log::info!("Router subscriptions registered.");

    let sequencer_quiet = quiet;
    thread::Builder::new()
        .name("sequencer".to_string())
        .spawn(move || {
            jdw_sequencer::run("", sequencer_quiet);
        })
        .expect("failed to spawn sequencer thread");

    // jdw-sc run() blocks here and installs the Ctrl-C handler.
    jdw_sc::run("", quiet);
}

/// Send `/subscribe` messages to the router, registering (osc_address, port) pairs
/// as subscribers. Each matching incoming message will be forwarded by the router
/// to `ip:port`.
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
