use std::{io::{Cursor, Read}, net::TcpStream};

use byteorder::{BigEndian, ReadBytesExt};

use super::{packet::Packet, packets::Packets};

pub fn read_packet(stream: &mut Box<TcpStream>, packets: &Packets) -> Result<Box<dyn Packet>, String> {
    let length = stream.read_u32::<BigEndian>().unwrap_or(0);
    if length == 0 {
        let _ = stream.shutdown(std::net::Shutdown::Both);
        return Err(String::from("Invalid Packet Length."));
    }

    let mut packet_data_vec = Vec::<u8>::with_capacity(length as usize);
    
    if stream.read_exact(&mut packet_data_vec.as_mut_slice()).is_err() {
        let _ = stream.shutdown(std::net::Shutdown::Both);
        return Err(String::from("Failed to read packet."));
    }

    let mut packet_data = Cursor::new(packet_data_vec);
    let packet_id = packet_data.read_u32::<BigEndian>().unwrap_or(u32::MAX);
    if packet_id == u32::MAX {
        return Err(String::from("Invalid Packet Length."));
    }

    let mut packet: Box<dyn Packet> = packets.get_new_packet_from_id(packet_id)?;
    let client_id = packet_data.read_u32::<BigEndian>().unwrap_or(u32::MAX);
    if client_id == u32::MAX {
        return Err(String::from("Invalid Client ID."));
    }

    packet.set_client_id(client_id);
    packet.deserialize(&mut packet_data);
    return Ok(packet);
}
