use openbrush::traits::Storage;

#[derive(Default, Debug)]
#[openbrush::storage_item]
pub struct Data {
    value: u8,
}

#[openbrush::trait_definition]
pub trait KvStore: Storage<Data>  {
    #[ink(message)]
    fn get_value(&self) -> u8 {
        self.data().value
    }

    #[ink(message)]
    fn set_value(&mut self, value: u8) {
        self.data().value = value;
    }
}
