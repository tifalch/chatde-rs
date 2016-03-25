use std::net::SocketAddr;
use std::net::AddrParseError;

#[inline]
pub fn parse_addr(s: &str) -> Result<SocketAddr, AddrParseError>{
    const PORT: u16 = 15327;

    format!("{}{}", s, if !s.contains(':'){
        format!(":{}", PORT)
    }else{
        "".to_owned()
    }).parse()
}

pub fn hex_string(b: &[u8]) -> String{
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
