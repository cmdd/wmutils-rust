extern crate xcb;
#[macro_use] extern crate clap;

use clap::{App, Arg};

pub mod util;

fn set_border(c: &xcb::Connection, win: xcb::Window, width: Option<&str>, color: Option<&str>) {
    if let Some(x) = width {
        let x = x.parse::<u32>().unwrap_or_else(util::invalid_number);
        xcb::configure_window(c, win, &[(xcb::CONFIG_WINDOW_BORDER_WIDTH as u16, x)]);
    }

    if let Some(y) = color {
        let y = y.parse::<u32>().unwrap_or_else(util::invalid_number);
        xcb::change_window_attributes(c, win, &[(xcb::CW_BORDER_PIXEL as u32, y)]);
    }
}

fn main() {
    let args = App::new("chwb")
        .about("change window border")
        .arg(Arg::with_name("size")
             .short("s")
             .min_values(1)
             .max_values(2))
        .arg(Arg::with_name("color")
             .short("c")
             .min_values(1)
             .max_values(2))
        .arg(Arg::with_name("wid")
             .multiple(true)
             .required(true))
        .get_matches();

    let wid: Vec<_> = args.values_of("wid").unwrap().collect();
    let wid: Vec<u32> = wid.iter().map(|win| util::get_window_id(win)).collect();

    let size = args.value_of("size");
    let color = args.value_of("color");

    let connection = util::init_xcb("chwb");

    for win in wid {
        set_border(&connection, win, size, color);
    }

    connection.flush();
}
