use std::net::SocketAddr;
use std::io::{Result as IOResult, Read, Write, BufRead, BufReader, stdout, stdin};

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
    pub fn reset_colour(&self) -> IOResult<()>{
        if self.flags.use_colour{
            stdout().write_all(b"\x1b[0m")
        }else{
            Ok(())
        }
    }

    pub fn set_colour(&self, c: Colour) -> IOResult<()>{
        if self.flags.use_colour{
            stdout().write_all(format!("\x1b[{}m", c as u8).as_bytes())
        }else{
            Ok(())
        }
    }

    pub fn chat_mode(&self, _ip: SocketAddr, pass: &str) -> IOResult<()>{
        let mut buf = BufReader::new(stdin());

        'chat: loop{
            try!(self.set_colour(Green));
            print!(" Δ ");
            try!(stdout().flush());

            let mut msg = String::new();
            try!(buf.read_line(&mut msg));
            let msg = msg.trim();

            try!(self.reset_colour());

            if msg.len() == 0{
                continue 'chat
            }

            if let Some('§') = msg.chars().nth(0){
                match msg{
                    "§bye"|"§quit" => {
                        break 'chat
                    },
                    _ => continue 'chat
                }
            }

            try!(self.send(msg, pass))
        }

        Ok(())
    }

    pub fn send(&self, msg: &str, pass: &str) -> IOResult<()>{
        let mut dl = DeltaL::new();
        if pass != ""{
            dl.set_passphrase(pass);
        }
        try!(self.set_colour(Cyan));

        let mut buf = Vec::new();

        println!("--- begins encrypted data ---");
        try!(dl.encode(&mut msg.as_bytes(), &mut buf, self.flags.use_checksum));

        for i in 0.. {
            let t = i * 12;

            if t >= buf.len(){
                break
            }

            let size = ::std::cmp::min(12, buf.len() - t);

            println!("[{:02X}] ({:02}) [{}]", i, size, ::utils::hex_string(&buf[t..t+size]));

        }
        println!("--- end of encrypted data ---");
        self.reset_colour()
    }
}
