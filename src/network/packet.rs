use super::serialization::Serialization;

pub trait Packet: Serialization {
    fn get_id(&self) -> u32;
    fn get_mod_id(&self) -> u32;
}