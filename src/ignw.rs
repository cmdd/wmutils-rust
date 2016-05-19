extern crate xcb;
extern crate clap;

use clap::{App, Arg};

pub mod util;

fn main() {
    let args = App::new("ignw")
        .about("ignore window")
        .arg(Arg::with_name("reset")
             .short("r").conflicts_with("set"))
        .arg(Arg::with_name("set")
             .short("s")
             .conflicts_with("reset"))
        .arg(Arg::with_name("wid")
             .multiple(true)
             .required(true))
        .get_matches();

    let wid: Vec<_> = args.values_of("wid").unwrap().collect();
    let wid: Vec<u32> = wid.iter().map(|win| util::get_window_id(win)).collect();

    let connection = util::init_xcb("ignw");

    if args.is_present("reset") {
        for win in wid {
            xcb::change_window_attributes(&connection, win, &[(xcb::CW_OVERRIDE_REDIRECT, 0 as u32)]);
        }
    } else if args.is_present("set") {
        for win in wid {
            xcb::change_window_attributes(&connection, win, &[(xcb::CW_OVERRIDE_REDIRECT, 1 as u32)]);
        }
    }

    connection.flush();
}
