multiversx_sc::imports!();
multiversx_sc::derive_imports!();

/// Module that calls another contract to read the content of a SetMapper remotely
#[multiversx_sc::module]
pub trait StorageMapperGetAtAddress {
   
   #[storage_mapper("empty_set_mapper")]
   fn empty_set_mapper(&self) -> SetMapper<u32>;

   #[storage_mapper("contract_address")]
   fn contract_address(&self) -> SingleValueMapper<ManagedAddress>;

   #[endpoint]
   fn set_contract_address(&self, address: ManagedAddress) {
    self.contract_address().set(address)
   }
}
