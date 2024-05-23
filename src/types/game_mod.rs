use crate::network::packet::Packet;

pub struct GameModData {
    name: String,
    packets: Vec<Box<dyn Packet>>
}

impl GameModData {
    // pub fn get_packet_by_id(&self, id: u32) -> dyn Packet {
    //     for
    // }
}

pub trait GameMod {
    fn get_mod_data(&self) -> GameModData;
    fn get_packet_by_id(&self, id: u32) -> dyn Packet;
    fn init(&self) -> bool;
}