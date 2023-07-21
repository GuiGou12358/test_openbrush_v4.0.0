#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(KvStore)]
#[openbrush::contract]
pub mod test_contract {

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
            let contract = MyContract::new();
            contract.set_value(12);
            assert_eq!(12, contract.get_value());
        }

    }

}
