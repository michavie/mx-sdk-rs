use elrond_codec::TopEncodeMulti;

use crate::{
    api::CallTypeApi,
    types::{BigUint, ManagedAddress, ManagedVec},
};

use super::{
    contract_call_full::ContractCallFull, contract_call_no_payment::ContractCallNoPayment,
    ContractCallTrait,
};

#[must_use]
pub struct ContractCallWithEgld<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
{
    pub(super) basic: ContractCallNoPayment<SA, OriginalResult>,
    pub(super) egld_payment: BigUint<SA>,
}

impl<SA, OriginalResult> ContractCallTrait<SA> for ContractCallWithEgld<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
    OriginalResult: TopEncodeMulti,
{
    type OriginalResult = OriginalResult;

    fn into_contract_call_full(self) -> ContractCallFull<SA, OriginalResult> {
        ContractCallFull {
            basic: self.basic,
            egld_payment: self.egld_payment,
            payments: ManagedVec::new(),
        }
    }

    #[inline]
    fn get_mut_basic(&mut self) -> &mut ContractCallNoPayment<SA, OriginalResult> {
        &mut self.basic
    }
}

impl<SA, OriginalResult> ContractCallWithEgld<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
{
    pub fn proxy_new(
        to: ManagedAddress<SA>,
        endpoint_name: &'static str,
        egld_payment: BigUint<SA>,
    ) -> Self {
        ContractCallWithEgld {
            basic: ContractCallNoPayment::proxy_new(to, endpoint_name),
            egld_payment,
        }
    }

    pub fn proxy_arg<T: TopEncodeMulti>(&mut self, endpoint_arg: &T) {
        super::contract_call_common::proxy_arg(&mut self.basic.arg_buffer, endpoint_arg)
    }
}
