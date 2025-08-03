use crate::compress::{compress, FieldElement, State};
use core::fmt;
use digest::{
    HashMarker, Output,
    block_api::{
        AlgorithmName, Block, BlockSizeUser, Buffer, BufferKindUser, Eager, FixedOutputCore,
        OutputSizeUser, Reset, UpdateCore,
    },
    crypto_common::hazmat::{DeserializeStateError, SerializableState, SerializedState},
    typenum::{U24, U32, U64, Unsigned},
};

const STATE_LEN: usize = 3;
const S0: State = [
    FieldElement::new(0),
    FieldElement::new(0),
    FieldElement::new(0),
];

/// Core Poseidon hasher state.
#[derive(Clone)]
pub struct PoseidonCore {
    block_len: u64,
    state: State,
}

impl HashMarker for PoseidonCore {}

impl BlockSizeUser for PoseidonCore {
    type BlockSize = U64;
}

impl BufferKindUser for PoseidonCore {
    type BufferKind = Eager;
}

impl OutputSizeUser for PoseidonCore {
    type OutputSize = U24;
}

impl UpdateCore for PoseidonCore {
    #[inline]
    fn update_blocks(&mut self, blocks: &[Block<Self>]) {
        self.block_len += blocks.len() as u64;
        for block in blocks {
            compress(&mut self.state, block.as_ref());
        }
    }
}

impl FixedOutputCore for PoseidonCore {
    #[inline]
    fn finalize_fixed_core(&mut self, buffer: &mut Buffer<Self>, out: &mut Output<Self>) {
        let bs = Self::BlockSize::U64;
        let pos = buffer.get_pos() as u64;
        let bit_len = 8 * (pos + bs * self.block_len);

        // Apply padding
        buffer.len64_padding_le(bit_len, |b| compress(&mut self.state, b.as_ref()));

        // Convert state to output bytes
        for (chunk, elem) in out.chunks_exact_mut(8).zip(self.state.iter()) {
            chunk.copy_from_slice(&elem.value().to_le_bytes());
        }
    }
}

impl Default for PoseidonCore {
    #[inline]
    fn default() -> Self {
        Self {
            block_len: 0,
            state: S0,
        }
    }
}

impl Reset for PoseidonCore {
    #[inline]
    fn reset(&mut self) {
        *self = Default::default();
    }
}

impl SerializableState for PoseidonCore {
    type SerializedStateSize = U32;

    fn serialize(&self) -> SerializedState<Self> {
        let mut serialized_state = SerializedState::<Self>::default();

        for (elem, chunk) in self.state.iter().zip(serialized_state.chunks_exact_mut(8)) {
            chunk.copy_from_slice(&elem.value().to_le_bytes());
        }

        serialized_state[24..].copy_from_slice(&self.block_len.to_le_bytes());

        serialized_state
    }

    fn deserialize(
        serialized_state: &SerializedState<Self>,
    ) -> Result<Self, DeserializeStateError> {
        let mut state = [FieldElement::new(0); STATE_LEN];

        for (elem, chunk) in state.iter_mut().zip(serialized_state.chunks_exact(8)) {
            let value = u64::from_le_bytes(chunk.try_into().unwrap());
            *elem = FieldElement::new(value);
        }

        let block_len = u64::from_le_bytes(
            serialized_state[24..32].try_into().unwrap(),
        );

        Ok(Self {
            block_len,
            state,
        })
    }
}

impl AlgorithmName for PoseidonCore {
    #[inline]
    fn write_alg_name(f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Poseidon")
    }
}

impl fmt::Debug for PoseidonCore {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("PoseidonCore { ... }")
    }
}

impl Drop for PoseidonCore {
    #[inline]
    fn drop(&mut self) {
        #[cfg(feature = "zeroize")]
        digest::zeroize::Zeroize::zeroize(self);
    }
}

#[cfg(feature = "zeroize")]
impl digest::zeroize::ZeroizeOnDrop for PoseidonCore {} 