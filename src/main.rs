use std::error::Error;
use std::fs::File;
use std::io::Read;

mod common;
mod messages;
mod objects;
mod tlvs;

use messages::header::CommonHeader;
use messages::keepalive::KeepAlive;
use messages::open::Open;
use messages::types::MessageType;
use objects::open::OpenObject;

fn main() -> Result<(), Box<dyn Error>> {
    let mut f = File::open("tmp_packet")?;
    let mut contents: Vec<u8> = vec![];
    f.read_to_end(&mut contents)?;

    let (remaining, common_header) = match CommonHeader::parse_common_header(&contents) {
        Ok((remaining, header)) => (remaining, header),
        Err(e) => panic!("{:?}", e),
    };

    println!("{}", common_header);
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
            let keepalive_msg: KeepAlive = common_header.into();
            println!("[+] Pcep keepalive message..");
            print!("{}", keepalive_msg);
        }
        MessageType::PCUpd => {
            println!("[+] Pcep PCupd message");
            unimplemented!();
            // match PcepUpdate::parse_update_message(remaining) {
            //     Ok((_remaining, mut pcupd)) => {
            //         pcupd.set_common_header(common_header);
            //     },
            //     Err(e) => {
            //         panic!("{:?}",e);
            //     }
            // }
        }
        _ => {
            println!("[!!]Unknown message type detected");
        }
    }

    Ok(())
}
