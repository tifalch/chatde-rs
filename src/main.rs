extern crate delta_l;
extern crate getopts;

pub mod utils;
mod chatter;

use chatter::{Chatter, Flags};
pub use chatter::Colour::*;

use std::env;
use std::io;

use getopts::Options;

const HELLO: &'static str = "Welcome to gochatde an encrypted terminal chat client using delta-l encryption.";
const USAGE: &'static str = r"
USAGE:
    chatde-rs [OPTIONS] ip[:PORT]
";

fn main() {
    let mut options = Options::new();

    options.optflag("", "color", "Use colour.");
    options.optflag("z", "gzip", "Use compression.");
    options.optflag("c", "checksum", "Don't use checksum.");
    options.optopt("p", "passphrase", "Use a passphrase", "PASSPHRASE");

    let matches = options.parse(env::args().skip(1)).unwrap();

    let use_colour   =  matches.opt_present("color");
    let use_compress =  matches.opt_present("gzip");
    let use_checksum = !matches.opt_present("checksum");
    let pass = matches.opt_str("passphrase").unwrap_or("".to_owned());

    let args = matches.free;

    let chatter = Chatter{flags: Flags{
        use_colour: use_colour,
        use_compress: use_compress,
        use_checksum: use_checksum
    }};

    if args.len() == 1{
        let ip = utils::parse_addr(&args[0]).unwrap();

        println!("{}", HELLO);
        match chatter.chat_mode(ip, &pass){
            Ok(()) => {
                chatter.set_colour(YellowSlashBrown).unwrap_or(());
                println!("BYE!");
                chatter.reset_colour().unwrap_or(());
            },
            Err(e) => return handle_error(chatter, e)
        }
    }else{
        chatter.set_colour(Red).unwrap_or(());
        println!("Incorrect uasge!\n");
        print!("{}", USAGE);
        chatter.reset_colour().unwrap_or(());
    }
}

fn handle_error(chatter: Chatter, err: io::Error){
    chatter.set_colour(Red).unwrap_or(());
    match err.kind(){
        _ => println!("Unexpected error occured: {:?}", err)
    }
    chatter.reset_colour().unwrap_or(());
}
