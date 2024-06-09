use std::net::TcpStream;

use crate::core::game_state::{Client, GameStateData};
use super::{packets::new_client_packet::NewClientPacket, serialization::Serialization};

pub trait Packet: Serialization {
    fn get_id(&self) -> u32;
    fn get_client_id(&self) -> u32;
    fn operate(&self, packet_client: &mut Client, game_state: &mut GameStateData) -> Result<PacketType, ()>;
    fn set_client_id(&mut self, id: u32);
    fn get_new(&self) -> PacketType;
}

pub enum PacketType {
    NewClientPacket(Box<NewClientPacket>),
    GenericPacket(Box<dyn Packet>)
}

impl PacketType {
    pub fn set_client_id(&mut self, id: u32) {
        match self {
            Self::NewClientPacket(packet) => {
                packet.set_client_id(id);
            },
            Self::GenericPacket(packet) => {
                packet.set_client_id(id);
            }
        }
    }

    pub fn get_client_id(&mut self) -> u32 {
        match self {
            Self::NewClientPacket(packet) => {
                return packet.get_client_id();
            },
            Self::GenericPacket(packet) => {
                return packet.get_client_id();
            }
        }
    }
}

impl Serialization for PacketType {
    fn serialize(&self) -> Vec<u8> {
        match self {
            Self::NewClientPacket(packet) => {
                return packet.serialize();
            },
            Self::GenericPacket(packet) => {
                return packet.serialize();
            }
        }
    }

    fn deserialize(&mut self, data: &mut std::io::Cursor<Vec<u8>>) -> () {
        match self {
            Self::NewClientPacket(packet) => {
                packet.deserialize(data);
            },
            Self::GenericPacket(packet) => {
                packet.deserialize(data);
            }
        }
    }
}