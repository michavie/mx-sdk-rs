use super::BoxedBytes;
use crate::{
    abi::TypeAbi,
    api::{Handle, ManagedTypeApi},
    types::{ManagedBuffer, ManagedType},
};
use alloc::string::String;
use elrond_codec::*;

/// Specialized type for handling token identifiers.
/// It wraps a BoxedBytes with the full ASCII name of the token.
/// EGLD is stored as an empty name.
///
/// Not yet implemented, but we might add additional restrictions when deserializing as argument.
#[derive(Clone, Debug)]
pub struct TokenIdentifier<M: ManagedTypeApi> {
    buffer: ManagedBuffer<M>,
}

impl<M: ManagedTypeApi> ManagedType<M> for TokenIdentifier<M> {
    #[inline]
    fn from_raw_handle(handle: Handle) -> Self {
        TokenIdentifier {
            buffer: ManagedBuffer::from_raw_handle(handle),
        }
    }

    #[doc(hidden)]
    fn get_raw_handle(&self) -> Handle {
        self.buffer.get_raw_handle()
    }
}

impl<M: ManagedTypeApi> TokenIdentifier<M> {
    /// This special representation is interpreted as the EGLD token.
    #[allow(clippy::needless_borrow)] // clippy is wrog here, there is no other way
    pub const EGLD_REPRESENTATION: &'static [u8; 4] = &b"EGLD";

    #[inline]
    pub fn from_esdt_bytes<B: Into<ManagedBuffer<M>>>(bytes: B) -> Self {
        TokenIdentifier {
            buffer: bytes.into(),
        }
    }

    /// New instance of the special EGLD token representation.
    #[inline]
    pub fn egld() -> Self {
        TokenIdentifier {
            buffer: ManagedBuffer::new(),
        }
    }

    #[inline]
    pub fn is_egld(&self) -> bool {
        self.is_empty()
    }

    #[inline]
    pub fn is_esdt(&self) -> bool {
        !self.is_egld()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    #[inline]
    pub fn into_managed_buffer(self) -> ManagedBuffer<M> {
        self.buffer
    }

    #[inline]
    pub fn as_managed_buffer(&self) -> &ManagedBuffer<M> {
        &self.buffer
    }

    #[inline]
    pub fn to_esdt_identifier(&self) -> BoxedBytes {
        self.buffer.to_boxed_bytes()
    }

    #[inline]
    pub fn as_name(&self) -> BoxedBytes {
        if self.is_egld() {
            BoxedBytes::from(&Self::EGLD_REPRESENTATION[..])
        } else {
            self.buffer.to_boxed_bytes()
        }
    }

    pub fn is_valid_esdt_identifier(&self) -> bool {
        M::instance().validate_token_identifier(self.buffer.handle)
    }
    /// Converts `"EGLD"` to `""`.
    /// Does nothing for the other values.
    fn normalize(&mut self) {
        if self.buffer == Self::EGLD_REPRESENTATION {
            self.buffer.overwrite(&[]);
        }
    }
}

impl<M: ManagedTypeApi> From<ManagedBuffer<M>> for TokenIdentifier<M> {
    #[inline]
    fn from(buffer: ManagedBuffer<M>) -> Self {
        let mut token_identifier = TokenIdentifier { buffer };
        token_identifier.normalize();
        token_identifier
    }
}

impl<M: ManagedTypeApi> From<&[u8]> for TokenIdentifier<M> {
    fn from(bytes: &[u8]) -> Self {
        if bytes == Self::EGLD_REPRESENTATION {
            TokenIdentifier::egld()
        } else {
            TokenIdentifier {
                buffer: ManagedBuffer::new_from_bytes(bytes),
            }
        }
    }
}

impl<M: ManagedTypeApi> PartialEq for TokenIdentifier<M> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.buffer == other.buffer
    }
}

impl<M: ManagedTypeApi> Eq for TokenIdentifier<M> {}

impl<M: ManagedTypeApi> NestedEncode for TokenIdentifier<M> {
    #[inline]
    fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
        if self.is_empty() {
            (&Self::EGLD_REPRESENTATION[..]).dep_encode(dest)
        } else {
            self.buffer.dep_encode(dest)
        }
    }
}

impl<M: ManagedTypeApi> TopEncode for TokenIdentifier<M> {
    #[inline]
    fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
        if self.is_empty() {
            (&Self::EGLD_REPRESENTATION[..]).top_encode(output)
        } else {
            self.buffer.top_encode(output)
        }
    }
}

impl<M: ManagedTypeApi> NestedDecode for TokenIdentifier<M> {
    fn dep_decode<I: NestedDecodeInput>(input: &mut I) -> Result<Self, DecodeError> {
        Ok(TokenIdentifier::from(ManagedBuffer::dep_decode(input)?))
    }

    fn dep_decode_or_exit<I: NestedDecodeInput, ExitCtx: Clone>(
        input: &mut I,
        c: ExitCtx,
        exit: fn(ExitCtx, DecodeError) -> !,
    ) -> Self {
        TokenIdentifier::from(ManagedBuffer::dep_decode_or_exit(input, c, exit))
    }
}

impl<M: ManagedTypeApi> TopDecode for TokenIdentifier<M> {
    fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
        Ok(TokenIdentifier::from(ManagedBuffer::top_decode(input)?))
    }

    fn top_decode_or_exit<I: TopDecodeInput, ExitCtx: Clone>(
        input: I,
        c: ExitCtx,
        exit: fn(ExitCtx, DecodeError) -> !,
    ) -> Self {
        TokenIdentifier::from(ManagedBuffer::top_decode_or_exit(input, c, exit))
    }
}

impl<M: ManagedTypeApi> TypeAbi for TokenIdentifier<M> {
    fn type_name() -> String {
        "TokenIdentifier".into()
    }
}
