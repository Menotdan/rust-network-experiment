use std::net::TcpStream;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use crate::{core::game_state::{Client, GameStateData}, network::{packet::{Packet, PacketType}, serialization::Serialization}};

#[derive(Default)]
pub struct NewClientPacket {
    client_id: u32,
    pub meow: u32,
}

impl Serialization for NewClientPacket {
    fn serialize(&self) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::new();
        let _ = out.write_u32::<BigEndian>(self.meow);

        return out;
    }

    fn deserialize(&mut self, data: &mut std::io::Cursor<Vec<u8>>) -> () {
        self.meow = data.read_u32::<BigEndian>().unwrap();
    }
}

impl Packet for NewClientPacket {
    fn get_id(&self) -> u32 {
        return 0;
    }
    
    fn get_client_id(&self) -> u32 {
        return self.client_id;
    }

    fn set_client_id(&mut self, id: u32) -> () {
        self.client_id = id;
    }
    
    fn operate(&self, _packet_client: &mut Client, _game_state: &mut GameStateData) -> Result<PacketType, ()> {
        return Err(());
    }

    fn get_new(&self) -> PacketType {
        return PacketType::NewClientPacket(Box::new(NewClientPacket::default()));
    }
}