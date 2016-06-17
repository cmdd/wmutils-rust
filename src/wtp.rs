extern crate xcb;
#[macro_use] extern crate clap;

use clap::{App, Arg};

pub mod util;

fn main() {
    let args = App::new("wtp")
        .about("teleport window")
        .arg(Arg::with_name("x").required(true))
        .arg(Arg::with_name("y").required(true))
        .arg(Arg::with_name("w").required(true))
        .arg(Arg::with_name("h").required(true))
        .arg(Arg::with_name("wid").required(true))
        .get_matches();

    let wid = args.value_of("wid").unwrap();
    let wid = util::get_window_id(wid);

    let x = value_t!(args.value_of("x"), u32).unwrap_or_else(util::invalid_number);
    let y = value_t!(args.value_of("y"), u32).unwrap_or_else(util::invalid_number);
    let w = value_t!(args.value_of("w"), u32).unwrap_or_else(util::invalid_number);
    let h = value_t!(args.value_of("h"), u32).unwrap_or_else(util::invalid_number);

    let connection = util::init_xcb("wtp");

    let items: &[(u16, u32)] = &[
        (xcb::CONFIG_WINDOW_X as u16, x), (xcb::CONFIG_WINDOW_Y as u16, y), (xcb::CONFIG_WINDOW_WIDTH as u16, w), (xcb::CONFIG_WINDOW_HEIGHT as u16, h)];

    xcb::configure_window(&connection, wid, items);

    connection.flush();
}
