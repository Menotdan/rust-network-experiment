use std::{io::{Cursor, Error, Read, Write}, net::TcpStream};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use super::{packet::Packet, packets::Packets};

pub fn read_packet(stream: &mut Box<TcpStream>, packets: &Packets) -> Result<Result<Box<dyn Packet>, ()>, Error> {
    let length = match stream.read_u32::<BigEndian>() {
        Ok(value) => value,
        Err(err) => match err.kind() {
            std::io::ErrorKind::WouldBlock => return Ok(Err(())),
            _ => {
                return Err(err);
            }
        }
    };

    if length == 0 {
        return Err(std::io::Error::new::<String>(std::io::ErrorKind::UnexpectedEof, String::from("Packet length empty.")));
    }

    let mut packet_data_vec = vec![0u8; length as usize];
    
    match stream.read_exact(&mut packet_data_vec.as_mut_slice()) {
        Err(err) => {
            return Err(err);
        },
        _ => {}
    }

    let mut packet_data = Cursor::new(packet_data_vec);
    let packet_id = match packet_data.read_u32::<BigEndian>() {
        Ok(val) => val,
        Err(err) => return Err(err),
    };

    let mut packet: Box<dyn Packet> = match packets.get_new_packet_from_id(packet_id) {
        Ok(val) => val,
        Err(_) => return Err(std::io::Error::new::<String>(std::io::ErrorKind::InvalidData, String::from("Invalid packet id.")))
    };

    let client_id = match packet_data.read_u32::<BigEndian>() {
        Ok(val) => val,
        Err(err) => return Err(err)
    };

    packet.set_client_id(client_id);
    packet.deserialize(&mut packet_data);
    return Ok(Ok(packet));
}

pub fn write_packet(stream: &mut Box<TcpStream>, packet: Box<dyn Packet>) -> Result<u32, Error> {
    let packet_data_vec = packet.serialize();
    let mut send_data_vec = Vec::<u8>::new();

    let _ = send_data_vec.write_u32::<BigEndian>((packet_data_vec.len() as u32) + 8)?;
    let _ = send_data_vec.write_u32::<BigEndian>(packet.get_id())?;
    let _ = send_data_vec.write_u32::<BigEndian>(packet.get_client_id())?;
    let _ = send_data_vec.write_all(packet_data_vec.as_slice())?;

    let _ = stream.write_all(send_data_vec.as_slice())?;
    return Ok(send_data_vec.len() as u32);
}