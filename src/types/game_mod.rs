use crate::network::packet::Packet;

pub struct GameModData {
    packets: Vec<Box<dyn Packet>>,
}

pub trait GameMod {
    fn get_mod_data(&self) -> GameModData;
    fn get_packet_by_id(&self, id: u32) -> dyn Packet;
    fn init(&self) -> bool;
}