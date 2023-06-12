use std::error::Error;
use std::fs::File;
use std::io::Read;

mod classes;
mod common;
mod messages;
mod objects;
mod tlvs;
mod types;

use messages::{CommonHeader, KeepAlive, MessageType, Open};
use objects::OpenObject;

fn main() -> Result<(), Box<dyn Error>> {
    let mut f = File::open("tmp_packet")?;
    let mut contents: Vec<u8> = vec![];
    f.read_to_end(&mut contents)?;

    let (remaining, common_header) = match CommonHeader::parse_common_header(&contents) {
        Ok((remaining, header)) => (remaining, header),
        Err(e) => panic!("{:?}", e),
    };

    match common_header.message_type {
        MessageType::Open => {
            println!("[+] Pcep Open message..");
            match OpenObject::parse_open_object(remaining) {
                Ok((_remaining, open_object)) => {
                    let open_msg = Open::new(common_header, open_object);
                    print!("{}", open_msg);
                }
                Err(e) => panic!("{:?}", e),
            }
        }
        MessageType::Keepalive => {
            assert_eq!(remaining.len(), 0, "[!!] Malformed Keepalive message");
            let keepalive_msg: KeepAlive = common_header.into();
            println!("[+] Pcep keepalive message..");
            print!("{}", keepalive_msg);
        }
        _ => {
            println!("[!!]Unknown message type detected");
        }
    }

    Ok(())
}
