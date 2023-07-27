use openbrush::traits::Storage;

#[derive(Default, Debug)]
#[openbrush::storage_item]
pub struct Data {
    value: u8,
}

pub trait KvStoreImpl: Storage<Data> + Sized {
    fn get_value(&self) -> u8 {
        self.data().value
    }

    fn set_value(&mut self, value: u8) {
        self.data().value = value;
    }
}