extern crate delta_l;
extern crate getopts;

use std::net::SocketAddr;
use std::env;

use getopts::Options;

use delta_l::DeltaL;

const _PORT: u16 = 15327;
const HELLO: &'static str = "Welcome to gochatde an encrypted terminal chat client using delta-l encryption.";

fn main() {
    let mut options = Options::new();

    options.optflag("c", "color", "use colour");
    options.optflag("", "gzip", "use compression");
    options.optflag("", "checksum", "use checksum");

    let matches = options.parse(env::args().skip(1)).unwrap();
    let args = matches.free;

    if args.len() >= 1{
        let ip: SocketAddr = args[0].parse().unwrap();

        println!("{}", HELLO);
        if args.len() == 1{
            chat_mode(ip, "")
        }else{
            chat_mode(ip, &args[1])
        }
    }
}

use std::io::BufReader;
use std::io::stdin;

fn chat_mode(_ip: SocketAddr, pass: &str){
    let mut buf = BufReader::new(stdin());

    'chat: loop{
        Green.set();
        print!(" Δ ");
        std::io::stdout().flush().unwrap();

        let mut msg = String::new();
        buf.read_line(&mut msg).unwrap();
        let msg = msg.trim();

        Colour::reset();

        if msg.len() == 0{
            continue 'chat
        }

        if msg.chars().nth(0).unwrap() == '§'{
            match msg{
                "§bye"|"§quit" => {
                    YellowSlashBrown.set();
                    println!("BYE!");

                    break 'chat
                },
                _ => continue 'chat
            }
        }

        send(msg, pass)
    }
}

fn send(msg: &str, pass: &str){
    let mut dl = DeltaL::new();
    if pass != ""{
        dl.set_passphrase(pass);
    }
    Cyan.set();

    let mut buf = Vec::new();

    println!("--- begins encrypted data ---");
    dl.encode(&mut msg.as_bytes(), &mut buf, true).unwrap();

    for i in 0.. {
        let t = i * 12;

        if t >= buf.len(){
            break
        }

        let size = buf.len() - t;

        let size = if size > 12 {12}else{size};

        println!("[{:02X}] ({:02}) [{}]", i, size, hex_string(&buf[t..t+size]));

    }
    println!("--- end of encrypted data ---");
    Colour::reset();
}

fn hex_string(b: &[u8]) -> String{
    let mut ret_string = String::new();

    let n;

    if b[0] == 0xCE && b[1] == 0x94 && b[2] == 0x4C && b[3] == 0xA {
        ret_string.push_str(" CHECKSUM:  ");
        n = 5
    }else{
        n = 1
    }

    ret_string.push_str(&format!("{:02X}", b[n-1]));

    for v in &b[n..]{
        ret_string.push_str(&format!(" {:02X}", v))
    }

    ret_string
}

use std::io::{Read, Write, BufRead};
use std::io::stdout;

use Colour::*;

#[derive(Debug, Copy, Clone)]
pub enum Colour {
    Black = 30,
    Red = 31,
    Green = 32,
    // Looks orange on Windows
    YellowSlashBrown = 33,
    Blue = 34,
    Magenta = 35,
    Cyan = 36,
    White = 37
}

impl Colour{
    fn reset() {
        stdout().write_all(b"\x1b[0m").unwrap()
    }

    fn set(&self) {
        stdout().write_all(format!("\x1b[{}m", *self as u8).as_bytes()).unwrap()
    }
}
