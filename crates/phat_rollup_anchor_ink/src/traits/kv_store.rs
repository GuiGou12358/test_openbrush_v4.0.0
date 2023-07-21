
#[openbrush::trait_definition]
pub trait KvStore {
    #[ink(message)]
    fn get_value(&self) -> u8;

    #[ink(message)]
    fn set_value(&mut self, value: u8);
}