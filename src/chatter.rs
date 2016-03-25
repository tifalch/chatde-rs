use std::net::SocketAddr;
use std::io::{Read, Write, BufRead, BufReader, stdout, stdin};

use delta_l::DeltaL;

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
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

use self::Colour::*;

pub struct Flags{
    pub use_colour: bool,
    pub use_compress: bool,
    pub use_checksum: bool
}

pub struct Chatter{
    pub flags: Flags
}

impl Chatter{
    pub fn reset_colour(&self) {
        if self.flags.use_colour{
            stdout().write_all(b"\x1b[0m").unwrap()
        }
    }

    pub fn set_colour(&self, c: Colour) {
        if self.flags.use_colour{
            stdout().write_all(format!("\x1b[{}m", c as u8).as_bytes()).unwrap()
        }
    }

    pub fn chat_mode(&self, _ip: SocketAddr, pass: &str){
        let mut buf = BufReader::new(stdin());

        'chat: loop{
            self.set_colour(Green);
            print!(" Δ ");
            stdout().flush().unwrap();

            let mut msg = String::new();
            buf.read_line(&mut msg).unwrap();
            let msg = msg.trim();

            self.reset_colour();

            if msg.len() == 0{
                continue 'chat
            }

            if msg.chars().nth(0).unwrap() == '§'{
                match msg{
                    "§bye"|"§quit" => {
                        self.set_colour(YellowSlashBrown);
                        println!("BYE!");

                        break 'chat
                    },
                    _ => continue 'chat
                }
            }

            self.send(msg, pass)
        }
    }

    pub fn send(&self, msg: &str, pass: &str){
        let mut dl = DeltaL::new();
        if pass != ""{
            dl.set_passphrase(pass);
        }
        self.set_colour(Cyan);

        let mut buf = Vec::new();

        println!("--- begins encrypted data ---");
        dl.encode(&mut msg.as_bytes(), &mut buf, self.flags.use_checksum).unwrap();

        for i in 0.. {
            let t = i * 12;

            if t >= buf.len(){
                break
            }

            let size = ::std::cmp::min(12, buf.len() - t);

            println!("[{:02X}] ({:02}) [{}]", i, size, ::utils::hex_string(&buf[t..t+size]));

        }
        println!("--- end of encrypted data ---");
        self.reset_colour();
    }
}
