use openbrush::traits::Storage;

pub use crate::traits::kv_store;
pub use crate::traits::kv_store::*;

#[derive(Default, Debug)]
#[openbrush::storage_item]
pub struct Data {
    pub value: u8,
}

pub trait KvStoreImpl: Storage<Data> + Sized {
    fn get_value(&self) -> u8 {
        self.data().value
    }

    fn set_value(&mut self, value: u8) {
        self.data().value = value;
    }
}