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
    options.optflag("D", "debug", "Enable debug info");
    options.optopt("p", "passphrase", "Use a passphrase", "PASSPHRASE");

    let matches = match options.parse(env::args().skip(1)){
        Ok(m) => m,
        Err(_) => return incorrect_usage()
    };

    let pass = matches.opt_str("passphrase").unwrap_or("".to_owned());

    let mut chatter = Chatter{
        working_dir: env::current_dir().unwrap(),
        flags: Flags{
            use_colour  :  matches.opt_present("color"),
            use_compress:  matches.opt_present("gzip"),
            use_checksum: !matches.opt_present("checksum"),
            debug       :  matches.opt_present("debug"),
        }
    };

    let args = matches.free;

    if chatter.flags.use_compress{
        chatter.set_colour(YellowSlashBrown).unwrap_or(());
        println!("WARN: Compression hasn't been implemented yet! See issue #1");
        chatter.reset_colour().unwrap_or(());
    }

    if args.len() == 1{
        let ip = utils::parse_addr(&args[0]).unwrap();

        println!("{}", HELLO);
        match chatter.chat_mode(ip, &pass){
            Ok(()) => {
                chatter.set_colour(Blue).unwrap_or(());
                println!("BYE!");
                chatter.reset_colour().unwrap_or(());
            },
            Err(e) => handle_error(chatter, e)
        }
    }else{
        incorrect_usage();
    }
}

fn incorrect_usage(){
    println!("Incorrect uasge!\n");
    print!("{}", USAGE);
}

fn handle_error(chatter: Chatter, err: io::Error) -> !{
    chatter.set_colour(Red).unwrap_or(());
    match err.kind(){
        _ => println!("Unexpected error occured: {:?}", err)
    }
    chatter.reset_colour().unwrap_or(());

    std::process::exit(1);
}
