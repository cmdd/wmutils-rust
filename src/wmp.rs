extern crate xcb;
#[macro_use]
extern crate clap;

use clap::{App, Arg};
use std::process;

pub mod util;

macro_rules! wlnerr(
    ($($arg:tt)*) => ({
        use std::io::{Write, stderr};
        writeln!(&mut stderr(), $($arg)*).ok();
    })
);

enum CursorMode {
    Absolute { x: i16, y: i16 },
    Relative { x: i16, y: i16 },
}

fn spot_cursor(c: &xcb::Connection, win: xcb::Window) {
    let qp = xcb::query_pointer(c, win);
    let qr = qp.get_reply();

    if let Ok(rr) = qr {
        unsafe {
            let r = *rr.ptr;
        
            if r.child != xcb::NONE {
                println!("{}, {}", r.win_x, r.win_y);
            } else {
                println!("{}, {}", r.root_x, r.root_y);
            }
        }
    } else {
        wlnerr!("cannot retrieve cursor position");
        process::exit(1);
    }
}

fn warp_cursor(c: &xcb::Connection, screen: &xcb::Screen, mode: CursorMode) {
    match mode {
        CursorMode::Absolute { x, y } => {
            unsafe {
                let ref scr = *screen.ptr;
                xcb::warp_pointer(c, xcb::NONE, scr.root, 0, 0, 0, 0, x, y);
            }
        },
        CursorMode::Relative { x, y } => {
            xcb::warp_pointer(c, xcb::NONE, xcb::NONE, 0, 0, 0, 0, x, y);
        },
    }
}

fn main() {
    let args = App::new("wmp")
        .about("pointer actions")
        .arg(Arg::with_name("absolute")
             .short("a")
             .takes_value(true)
             .number_of_values(2)
             .conflicts_with("relative"))
        .arg(Arg::with_name("relative")
             .short("r")
             .takes_value(true)
             .number_of_values(2)
             .conflicts_with("absolute"))
        .arg(Arg::with_name("wid"))
        .get_matches();

    let wid = args.value_of("wid");

    let connection = util::init_xcb("wmp");
    let setup = connection.get_setup();
    let screen = util::get_screen(&setup);

    let mode = if let Some(_) = args.values_of("absolute") {
        let coords = values_t!(args.values_of("absolute"), i16).unwrap_or_else(util::invalid_number);
        Some(CursorMode::Absolute { x: coords[0], y: coords[1] })
    } else if let Some(_) = args.values_of("relative") {
        let coords = values_t!(args.values_of("relative"), i16).unwrap_or_else(util::invalid_number);
        Some(CursorMode::Relative { x: coords[0], y: coords[1] })
    } else {
        None
    };

    match mode {
        Some(m) => warp_cursor(&connection, &screen, m),
        None => if let Some(wid) = wid {
            let win = util::get_window_id(wid);
            spot_cursor(&connection, win);
        } else {
            unsafe {
                let ref scr = *screen.ptr;
                let win = scr.root;
                spot_cursor(&connection, win);
            }
        }
    };

    connection.flush();
}
