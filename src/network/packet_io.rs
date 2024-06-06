use std::{io::{Cursor, Read, Write}, net::TcpStream};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use super::{packet::Packet, packets::Packets};

pub fn read_packet(stream: &mut Box<TcpStream>, packets: &Packets) -> Result<Result<Box<dyn Packet>, bool>, String> {
    let length = match stream.read_u32::<BigEndian>() {
        Ok(value) => value,
        Err(err) => match err.kind() {
            std::io::ErrorKind::WouldBlock => return Ok(Err(true)),
            _ => {
                let _ = stream.shutdown(std::net::Shutdown::Both);
                return Err(String::from("Failed to read packet length."));
            }
        }
    };

    if length == 0 {
        let _ = stream.shutdown(std::net::Shutdown::Both);
        return Err(String::from("Invalid Packet Length."));
    }

    let mut packet_data_vec = vec![0u8; length as usize];
    
    if stream.read_exact(&mut packet_data_vec.as_mut_slice()).is_err() {
        let _ = stream.shutdown(std::net::Shutdown::Both);
        return Err(String::from("Failed to read packet."));
    }

    let mut packet_data = Cursor::new(packet_data_vec);
    let packet_id = packet_data.read_u32::<BigEndian>().unwrap_or(u32::MAX);
    if packet_id == u32::MAX {
        return Err(String::from("Invalid Packet ID."));
    }

    let mut packet: Box<dyn Packet> = packets.get_new_packet_from_id(packet_id)?;
    let client_id = packet_data.read_u32::<BigEndian>().unwrap_or(u32::MAX);
    if client_id == u32::MAX {
        return Err(String::from("Invalid Client ID."));
    }

    packet.set_client_id(client_id);
    packet.deserialize(&mut packet_data);
    return Ok(Ok(packet));
}

pub fn write_packet(stream: &mut Box<TcpStream>, packet: Box<dyn Packet>) {
    let packet_data_vec = packet.serialize();
    let mut send_data_vec = Vec::<u8>::new();

    let _ = send_data_vec.write_u32::<BigEndian>((packet_data_vec.len() as u32) + 8);
    let _ = send_data_vec.write_u32::<BigEndian>(packet.get_id());
    let _ = send_data_vec.write_u32::<BigEndian>(packet.get_client_id());
    let _ = send_data_vec.write_all(packet_data_vec.as_slice());

    let _ = stream.write_all(send_data_vec.as_slice());
}