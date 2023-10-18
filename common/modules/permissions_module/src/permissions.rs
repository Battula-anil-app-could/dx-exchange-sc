use bitflags::bitflags;
use dharitri_sc::{
    abi::TypeAbi,
    codec::{DecodeError, TopDecode, TopEncode},
};
bitflags! {
    pub struct Permissions: u32 {
        const NONE = 0;
        const OWNER = 1;
        const ADMIN = 2;
        const PAUSE = 4;
    }
}

impl TopEncode for Permissions {
    fn top_encode<O>(&self, output: O) -> Result<(), dharitri_sc::codec::EncodeError>
    where
        O: dharitri_sc::codec::TopEncodeOutput,
    {
        u32::top_encode(&self.bits(), output)
    }
}

impl TopDecode for Permissions {
    fn top_decode<I>(input: I) -> Result<Self, dharitri_sc::codec::DecodeError>
    where
        I: dharitri_sc::codec::TopDecodeInput,
    {
        let bits = u32::top_decode(input)?;
        Permissions::from_bits(bits).ok_or(DecodeError::INVALID_VALUE)
    }
}

impl TypeAbi for Permissions {
    fn type_name() -> dharitri_sc::abi::TypeName {
        core::any::type_name::<u32>().into()
    }
}
