extern crate delta_l;
extern crate getopts;

pub mod utils;
mod chatter;

use chatter::{Chatter, Flags};
pub use chatter::Colour::*;

use std::env;

use getopts::Options;

const HELLO: &'static str = "Welcome to gochatde an encrypted terminal chat client using delta-l encryption.";
const USAGE: &'static str = r"
USAGE:
    chatde-rs [OPTIONS] ip[:PORT] [passphrase]
";

fn main() {
    let mut options = Options::new();

    options.optflag("", "color", "Use colour.");
    options.optflag("z", "gzip", "Use compression.");
    options.optflag("c", "checksum", "Don't use checksum.");

    let matches = options.parse(env::args().skip(1)).unwrap();

    let use_colour   =  matches.opt_present("color");
    let use_compress =  matches.opt_present("gzip");
    let use_checksum = !matches.opt_present("checksum");

    let args = matches.free;

    let chatter = Chatter{flags: Flags{
        use_colour: use_colour,
        use_compress: use_compress,
        use_checksum: use_checksum
    }};

    if args.len() >= 1{
        let ip = utils::parse_addr(&args[0]).unwrap();

        println!("{}", HELLO);
        if args.len() == 1{
            chatter.chat_mode(ip, "")
        }else{
            chatter.chat_mode(ip, &args[1])
        }
    }else{
        println!("{}", USAGE);
    }
}
