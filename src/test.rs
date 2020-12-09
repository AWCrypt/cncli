use std::path::PathBuf;

use cardano_ouroboros_network::mux;
use cardano_ouroboros_network::mux::Cmd;
use log::info;
use rug::{Float, Rational};
use rug::float::Round;
use rug::ops::MulAssignRound;

use nodeclient::leaderlog::is_overlay_slot;

use super::*;

#[test]
fn test_is_overlay_slot() {
    match var("RUST_LOG") {
        Ok(_) => {}
        Err(_) => {
            // set a default logging level of info if unset.
            set_var("RUST_LOG", "info");
        }
    }
    pretty_env_logger::init_timed();

    let first_slot_of_epoch = 15724800_i64;
    let mut current_slot = 16128499_i64;
    // let d = Float::with_val(120, 0.32);
    let mut d = Float::with_val(24, Float::parse("0.32").unwrap());
    d.mul_assign_round(100, Round::Nearest);
    let r: Rational = Rational::from((d.to_integer().unwrap(), 100));

    assert_eq!(is_overlay_slot(&first_slot_of_epoch, &current_slot, &r), false);

    // AD test
    current_slot = 15920150_i64;
    assert_eq!(is_overlay_slot(&first_slot_of_epoch, &current_slot, &r), true);
}

#[test]
fn test_ping() {
    let host = "north-america.relays-new.cardano-testnet.iohkdev.io".to_string();
    let port = 3001;
    let network_magic = 1097911063;
    let mut stdout: Vec<u8> = Vec::new();

    mux::start(&mut stdout, Cmd::Ping, &PathBuf::new(), &host, port, network_magic, &String::new(), &PathBuf::new(), &String::new(), &String::new());

    assert_eq!(&std::str::from_utf8(&stdout).unwrap()[..99], "{\n \"status\": \"ok\",\n \"host\": \"north-america.relays-new.cardano-testnet.iohkdev.io\",\n \"port\": 3001,\n ");
}

#[test]
fn test_rational() {
    match var("RUST_LOG") {
        Ok(_) => {}
        Err(_) => {
            // set a default logging level of info if unset.
            set_var("RUST_LOG", "info");
        }
    }
    pretty_env_logger::init_timed();

    let active_stake = 20374829620952_u64;
    let total_active_stake = 20094293921511214_u64;
    info!("activeStake: {:?}, totalActiveStake: {:?}", &active_stake, &total_active_stake);
    let sigma = Rational::from((active_stake, total_active_stake));
    info!("sigma: {:?}", sigma);
}