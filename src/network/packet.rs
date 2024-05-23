use super::serialization::Serialization;

pub trait Packet: Serialization {
    fn get_id(&self) -> u32;
    fn get_mod_id(&self) -> u32;
    fn get_client_id(&self) -> u32;
    fn operate(&self) -> bool;
    fn set_client_id(&mut self, id: u32) -> ();
}