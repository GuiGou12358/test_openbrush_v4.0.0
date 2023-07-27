use quote::{
    format_ident,
    quote,
};
use std::collections::HashMap;
use syn::Block;

pub type IsDefault = bool;
pub type OverridenFnMap = HashMap<String, Vec<(String, (Box<Block>, Vec<syn::Attribute>, IsDefault))>>;

pub struct ImplArgs<'a> {
    pub map: &'a OverridenFnMap,
    pub items: &'a mut Vec<syn::Item>,
    pub imports: &'a mut HashMap<&'a str, syn::ItemUse>,
    pub overriden_traits: &'a mut HashMap<&'a str, syn::Item>,
    pub storage_struct_name: String,
}

impl<'a> ImplArgs<'a> {
    pub fn new(
        map: &'a OverridenFnMap,
        items: &'a mut Vec<syn::Item>,
        imports: &'a mut HashMap<&'a str, syn::ItemUse>,
        overriden_traits: &'a mut HashMap<&'a str, syn::Item>,
        storage_struct_name: String,
    ) -> Self {
        Self {
            map,
            items,
            imports,
            overriden_traits,
            storage_struct_name,
        }
    }

    fn contract_name(&self) -> proc_macro2::Ident {
        format_ident!("{}", self.storage_struct_name)
    }

    fn vec_import(&mut self) {
        let vec_import = syn::parse2::<syn::ItemUse>(quote!(
            use ink::prelude::vec::Vec;
        ))
            .expect("Should parse");
        self.imports.insert("vec", vec_import);
    }
}

pub(crate) fn impl_kv_store(impl_args: &mut ImplArgs) {
    let storage_struct_name = impl_args.contract_name();

    let kv_store_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl KvStoreImpl for #storage_struct_name {}
    ))
        .expect("Should parse");

    let mut kv_store = syn::parse2::<syn::ItemImpl>(quote!(
        impl KvStore for #storage_struct_name {

            #[ink(message)]
            fn get_value(&self) -> u8 {
                KvStoreImpl::get_value(self)
            }

            #[ink(message)]
            fn set_value(&mut self, value: u8) {
                KvStoreImpl::set_value(self, value)
            }
        }
    ))
        .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
        use crate::traits::kv_store::*;
    ))
        .expect("Should parse");
    impl_args.imports.insert("KvStore", import);

    override_functions("KvStore", &mut kv_store, impl_args.map);

    //impl_args.items.push(syn::Item::Impl(internal_impl));
    //impl_args.items.push(syn::Item::Impl(internal));
    impl_args.items.push(syn::Item::Impl(kv_store_impl));
    impl_args.items.push(syn::Item::Impl(kv_store));
}
/*
pub(crate) fn impl_upgradeable(impl_args: &mut ImplArgs) {
    let storage_struct_name = impl_args.contract_name();
    let upgradeable_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl UpgradeableImpl for #storage_struct_name {}
    ))
        .expect("Should parse");

    let mut upgradeable = syn::parse2::<syn::ItemImpl>(quote!(
        impl Upgradeable for #storage_struct_name {
            #[ink(message)]
            fn set_code_hash(&mut self, new_code_hash: Hash)  -> Result<(),UpgradeableError>  {
                upgradeable::UpgradeableImpl::set_code_hash(self,new_code_hash)
            }
        }
    ))
        .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
        use openbrush::contracts::upgradeable::*;
    ))
        .expect("Should parse");
    impl_args.imports.insert("Upgradeable", import);

    override_functions("Upgradeable", &mut upgradeable, impl_args.map);

    impl_args.items.push(syn::Item::Impl(upgradeable));
    impl_args.items.push(syn::Item::Impl(upgradeable_impl));
}

 */

fn override_functions(trait_name: &str, implementation: &mut syn::ItemImpl, map: &OverridenFnMap) {
    if let Some(overrides) = map.get(trait_name) {
        // we will find which fns we wanna override
        for (fn_name, (fn_code, attributes, is_default)) in overrides {
            for item in implementation.items.iter_mut() {
                if let syn::ImplItem::Method(method) = item {
                    if &method.sig.ident.to_string() == fn_name {
                        if !is_default {
                            method.block = *fn_code.clone();
                        }
                        method.attrs.append(&mut attributes.to_vec());
                    }
                }
            }
        }
    }
}