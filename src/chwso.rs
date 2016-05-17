extern crate xcb;
extern crate clap;

use clap::{App, Arg, Format};

pub mod util;

// TODO: Should flags use overrides_with_all?
fn main() {
    let args = App::new("chwso")
        .about("change window stacking order")
        .arg(Arg::with_name("above")
             .short("r")
             .conflicts_with_all(&["below", "opposite"]))
        .arg(Arg::with_name("below")
             .short("l")
             .conflicts_with_all(&["above", "opposite"]))
        .arg(Arg::with_name("opposite")
             .short("i")
             .conflicts_with_all(&["above", "below"]))
        .arg(Arg::with_name("wid").required(true))
        .get_matches();

    let wid = args.value_of("wid").unwrap();
    let wid = util::get_window_id(wid);

    let connection = util::init_xcb("chwso");

    if args.is_present("above") {
        xcb::configure_window(&connection, wid, &[(xcb::CONFIG_WINDOW_STACK_MODE as u16, xcb::STACK_MODE_ABOVE as u32)]);
    } else if args.is_present("below") {
        xcb::configure_window(&connection, wid, &[(xcb::CONFIG_WINDOW_STACK_MODE as u16, xcb::STACK_MODE_BELOW as u32)]);
    } else if args.is_present("opposite") {
        xcb::configure_window(&connection, wid, &[(xcb::CONFIG_WINDOW_STACK_MODE as u16, xcb::STACK_MODE_OPPOSITE as u32)]);
    } else {
        // TODO: Maybe just print usage?
        //       Or use missing_required_argument() of Error?
        let err = clap::Error {
            message: format!("{} Must provide a flag\n\n{}\n\nFor more information try {}", Format::Error("error:"), args.usage(), Format::Good("--help")),
            kind: clap::ErrorKind::MissingRequiredArgument,
            info: None
        };
        err.exit();
    }

    connection.flush();
}
