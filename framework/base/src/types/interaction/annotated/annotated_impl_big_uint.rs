use crate::{
    proxy_imports::ManagedRef,
    types::{heap::Address, BigUint, ManagedAddress, ManagedBuffer},
};

use super::{AnnotatedValue, TxEnv};

impl<Env> AnnotatedValue<Env, BigUint<Env::Api>> for BigUint<Env::Api>
where
    Env: TxEnv,
{
    fn annotation(&self, _env: &Env) -> ManagedBuffer<Env::Api> {
        self.to_display()
    }

    fn to_value(&self, _env: &Env) -> BigUint<Env::Api> {
        self.clone()
    }

    fn into_value(self, _env: &Env) -> BigUint<Env::Api> {
        self
    }

    fn with_value_ref<F, R>(&self, env: &Env, f: F) -> R
    where
        F: FnOnce(&BigUint<Env::Api>) -> R,
    {
        f(self)
    }
}

impl<Env> AnnotatedValue<Env, BigUint<Env::Api>> for &BigUint<Env::Api>
where
    Env: TxEnv,
{
    fn annotation(&self, _env: &Env) -> ManagedBuffer<Env::Api> {
        self.to_display()
    }

    fn to_value(&self, _env: &Env) -> BigUint<Env::Api> {
        (*self).clone()
    }

    fn into_value(self, _env: &Env) -> BigUint<Env::Api> {
        self.clone()
    }

    fn with_value_ref<F, R>(&self, env: &Env, f: F) -> R
    where
        F: FnOnce(&BigUint<Env::Api>) -> R,
    {
        f(self)
    }
}

impl<'a, Env> AnnotatedValue<Env, BigUint<Env::Api>> for ManagedRef<'a, Env::Api, BigUint<Env::Api>>
where
    Env: TxEnv,
{
    fn annotation(&self, _env: &Env) -> ManagedBuffer<Env::Api> {
        self.to_display()
    }

    fn to_value(&self, _env: &Env) -> BigUint<Env::Api> {
        (*self).clone_value()
    }

    fn into_value(self, _env: &Env) -> BigUint<Env::Api> {
        self.clone_value()
    }

    fn with_value_ref<F, R>(&self, env: &Env, f: F) -> R
    where
        F: FnOnce(&BigUint<Env::Api>) -> R,
    {
        f(self)
    }
}

impl<Env> AnnotatedValue<Env, BigUint<Env::Api>> for u64
where
    Env: TxEnv,
{
    fn annotation(&self, env: &Env) -> ManagedBuffer<Env::Api> {
        self.to_value(env).to_display()
    }

    fn to_value(&self, _env: &Env) -> BigUint<Env::Api> {
        BigUint::from(*self)
    }
}

impl<Env> AnnotatedValue<Env, BigUint<Env::Api>> for ()
where
    Env: TxEnv,
{
    fn annotation(&self, env: &Env) -> ManagedBuffer<Env::Api> {
        ManagedBuffer::from("0")
    }

    fn to_value(&self, _env: &Env) -> BigUint<Env::Api> {
        BigUint::zero()
    }
}
