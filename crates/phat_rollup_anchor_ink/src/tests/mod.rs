#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[phat_rollup_anchor_ink_macro::implementation(KvStore)]
#[openbrush::contract]
pub mod test_contract {

    use crate::impls::{kv_store, kv_store::KvStoreImpl};
    use crate::traits::kv_store::KvStore;
    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct MyContract {
        #[storage_field]
        kv_store: kv_store::Data,
    }

    impl MyContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut instance = Self::default();
            instance
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn test_get_no_value() {
            let mut contract = MyContract::new();
            KvStore::set_value(&mut contract, 12);
            assert_eq!(12, KvStore::get_value(&contract));
        }
    }
}
