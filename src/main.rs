extern crate getopts;
extern crate librespot;
extern crate ctrlc;

use std::io::{stderr, Write};
use std::process::exit;
use std::thread;

use librespot::spirc::SpircManager;
use librespot::main_helper;

fn usage(program: &str, opts: &getopts::Options) -> String {
    let brief = format!("Usage: {} [options]", program);
    format!("{}", opts.usage(&brief))
}

fn main() {
    let mut opts = getopts::Options::new();
    main_helper::add_session_arguments(&mut opts);
    main_helper::add_authentication_arguments(&mut opts);
    main_helper::add_player_arguments(&mut opts);

    let args: Vec<String> = std::env::args().collect();

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            writeln!(stderr(), "error: {}\n{}", f.to_string(), usage(&args[0], &opts)).unwrap();
            exit(1);
        }
    };

    main_helper::setup_logging(&matches);

    let session = main_helper::session_from_matches(&matches);
    let credentials = main_helper::credentials_from_matches(&session, &matches);
    session.login(credentials).unwrap();

    let player = main_helper::player_from_matches(&session, &matches);

    let spirc = SpircManager::new(session.clone(), player);
    let spirc_signal = spirc.clone();
    thread::spawn(move || spirc.run());

    ctrlc::set_handler(move || {
        spirc_signal.send_goodbye();
        exit(0);
    });

    loop {
        session.poll();
    }
}
