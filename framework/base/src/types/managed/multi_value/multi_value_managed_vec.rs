use core::borrow::Borrow;

use multiversx_sc_codec::multi_types::MultiValueVec;

use crate::{
    abi::{TypeAbi, TypeAbiFrom, TypeDescriptionContainer, TypeName},
    api::ManagedTypeApi,
    codec::{
        DecodeErrorHandler, EncodeErrorHandler, TopDecodeMulti, TopDecodeMultiInput,
        TopEncodeMulti, TopEncodeMultiOutput, Vec,
    },
    types::ManagedType,
};

use crate::types::{ManagedVec, ManagedVecItem, ManagedVecRefIterator};

#[derive(Clone, Default)]
pub struct MultiValueManagedVec<M: ManagedTypeApi, T: ManagedVecItem>(ManagedVec<M, T>);

#[deprecated(
    since = "0.29.0",
    note = "Alias kept for backwards compatibility. Replace with `MultiValueManagedVec`"
)]
pub type ManagedVarArgsEager<M, T> = MultiValueManagedVec<M, T>;

#[deprecated(
    since = "0.29.0",
    note = "Alias kept for backwards compatibility. Replace with `MultiValueManagedVec`"
)]
pub type ManagedMultiResultVecEager<M, T> = MultiValueManagedVec<M, T>;

impl<M, T> From<ManagedVec<M, T>> for MultiValueManagedVec<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem,
{
    #[inline]
    fn from(managed_vec: ManagedVec<M, T>) -> Self {
        MultiValueManagedVec(managed_vec)
    }
}

impl<M, T> ManagedType<M> for MultiValueManagedVec<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem,
{
    type OwnHandle = M::ManagedBufferHandle;

    #[inline]
    unsafe fn from_handle(handle: M::ManagedBufferHandle) -> Self {
        Self(ManagedVec::from_handle(handle))
    }

    fn get_handle(&self) -> M::ManagedBufferHandle {
        self.0.get_handle()
    }

    unsafe fn forget_into_handle(self) -> Self::OwnHandle {
        self.0.forget_into_handle()
    }

    fn transmute_from_handle_ref(handle_ref: &M::ManagedBufferHandle) -> &Self {
        unsafe { core::mem::transmute(handle_ref) }
    }

    fn transmute_from_handle_ref_mut(handle_ref: &mut M::ManagedBufferHandle) -> &mut Self {
        unsafe { core::mem::transmute(handle_ref) }
    }
}

impl<M, T> MultiValueManagedVec<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem,
{
    #[inline]
    pub fn new() -> Self {
        MultiValueManagedVec(ManagedVec::new())
    }

    #[inline]
    pub fn byte_len(&self) -> usize {
        self.0.byte_len()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn get(&self, index: usize) -> T::Ref<'_> {
        self.0.get(index)
    }

    #[allow(clippy::redundant_closure)]
    pub fn slice(&self, start_index: usize, end_index: usize) -> Option<Self> {
        self.0
            .slice(start_index, end_index)
            .map(|value| Self(value))
    }

    pub fn push(&mut self, item: T) {
        self.0.push(item)
    }

    pub fn from_single_item(item: T) -> Self {
        let mut result = MultiValueManagedVec::new();
        result.push(item);
        result
    }

    pub fn overwrite_with_single_item(&mut self, item: T) {
        self.0.overwrite_with_single_item(item)
    }

    pub fn append_vec(&mut self, item: MultiValueManagedVec<M, T>) {
        self.0.append_vec(item.0)
    }

    pub fn clear(&mut self) {
        self.0.clear()
    }

    pub fn into_vec(self) -> ManagedVec<M, T> {
        self.0
    }

    #[cfg(feature = "alloc")]
    pub fn with_self_as_vec<F>(&mut self, f: F)
    where
        F: FnOnce(&mut Vec<T>),
    {
        self.0.with_self_as_vec(f)
    }

    pub fn iter(&self) -> ManagedVecRefIterator<M, T> {
        ManagedVecRefIterator::new(&self.0)
    }
}

impl<'a, M, T> IntoIterator for &'a MultiValueManagedVec<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem,
{
    type Item = T::Ref<'a>;

    type IntoIter = ManagedVecRefIterator<'a, M, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<M, T, I> From<Vec<I>> for MultiValueManagedVec<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem,
    I: Into<T>,
{
    fn from(v: Vec<I>) -> Self {
        let mut result = Self::new();
        for item in v.into_iter() {
            result.push(item.into());
        }
        result
    }
}

impl<M, T> TopEncodeMulti for &MultiValueManagedVec<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem + TopEncodeMulti,
{
    fn multi_encode_or_handle_err<O, H>(&self, output: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeMultiOutput,
        H: EncodeErrorHandler,
    {
        for elem in &self.0 {
            elem.borrow().multi_encode_or_handle_err(output, h)?;
        }
        Ok(())
    }
}

impl<M, T> TopEncodeMulti for MultiValueManagedVec<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem + TopEncodeMulti,
{
    fn multi_encode_or_handle_err<O, H>(&self, output: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeMultiOutput,
        H: EncodeErrorHandler,
    {
        (&self).multi_encode_or_handle_err(output, h)
    }
}

impl<M, T> TopDecodeMulti for MultiValueManagedVec<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem + TopDecodeMulti,
{
    fn multi_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeMultiInput,
        H: DecodeErrorHandler,
    {
        let mut result_vec: ManagedVec<M, T> = ManagedVec::new();
        while input.has_next() {
            result_vec.push(T::multi_decode_or_handle_err(input, h)?);
        }
        Ok(MultiValueManagedVec(result_vec))
    }
}

impl<M, T: TypeAbi> TypeAbiFrom<Self> for MultiValueManagedVec<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem,
{
}

impl<M, T: TypeAbi> TypeAbi for MultiValueManagedVec<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem,
{
    type Unmanaged = MultiValueVec<T::Unmanaged>;

    fn type_name() -> TypeName {
        crate::abi::type_name_variadic::<T>()
    }

    fn type_name_rust() -> TypeName {
        alloc::format!("MultiValueManagedVec<$API, {}>", T::type_name_rust())
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(accumulator: &mut TDC) {
        T::provide_type_descriptions(accumulator);
    }

    fn is_variadic() -> bool {
        true
    }
}
