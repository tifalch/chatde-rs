use std::net::SocketAddr;
use std::fs::File;
use std::io::{Result as IOResult, Read, Write, BufRead, BufReader, stdout, stdin};
use std::path::PathBuf;
use std::fs;

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
    pub use_checksum: bool,
    pub debug: bool,
}

pub struct Chatter{
    pub flags: Flags,
    pub working_dir: PathBuf,
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

    pub fn chat_mode(&mut self, _ip: SocketAddr, pass: &str) -> IOResult<()>{
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
                let cmd = msg[2..].split(' ').collect::<Vec<_>>();

                match cmd[0]{
                    "bye"|"quit" => {
                        break 'chat
                    },
                    "ls" => {
                        try!(self.set_colour(White));
                        for entry in try!(fs::read_dir(&self.working_dir)){
                            let entry = try!(entry);

                            println!("{}", entry.file_name().to_string_lossy())
                        }
                        try!(self.reset_colour());
                    },
                    "cd" if cmd.len() > 1 => {
                        let mut temp = self.working_dir.clone();
                        temp.push(cmd[1]);

                        if temp.is_dir() && temp.exists(){
                            self.working_dir = try!(fs::canonicalize(temp));
                        }else{
                            try!(self.set_colour(Red));
                            println!("Folder doesn't exist");
                            try!(self.reset_colour());
                        }

                    },
                    "cd"|"pwd" => {
                        try!(self.set_colour(White));
                        println!("{}", self.working_dir.display());
                        try!(self.reset_colour());
                    },
                    "file" => {
                        if cmd.len() > 1 {
                            let file_path = self.working_dir.join(cmd[1]);

                            if file_path.exists() && file_path.is_file() {
                                let mut file = try!(File::open(file_path));
                                let mut buf = Vec::new();
                                try!(file.read_to_end(&mut buf));

                                try!(self.send(&*buf, pass));
                                continue 'chat
                            }else{
                                try!(self.set_colour(Red));
                                println!("No such file!");
                                try!(self.reset_colour());
                            }
                        }else{
                            try!(self.set_colour(Red));
                            println!("Please specify a file to send");
                            try!(self.reset_colour());
                        }
                    },
                    _ => {
                        try!(self.set_colour(Red));
                        println!("Unknown command!");
                        try!(self.reset_colour());
                    }
                }

                continue 'chat
            }

            try!(self.send(msg.as_bytes(), pass))
        }

        Ok(())
    }

    pub fn send(&self, mut bytes: &[u8], pass: &str) -> IOResult<()>{
        let mut dl = DeltaL::new();
        if pass != ""{
            dl.set_passphrase(pass);
        }

        let mut buf = Vec::new();

        try!(dl.encode(&mut bytes, &mut buf, self.flags.use_checksum));

        if self.flags.debug {
            try!(self.set_colour(Cyan));

            println!("--- begins encrypted data ---");

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
        }else{Ok(())}
    }
}
