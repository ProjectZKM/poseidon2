use crate::{RATE, WIDTH};
use p3_field::{FieldAlgebra, PrimeField32};
use p3_koala_bear::KoalaBear;
use p3_symmetric::Permutation;
use zkm_primitives::poseidon2_init;

#[inline]
pub(crate) fn poseidon2_sponge(input: &[u32]) -> Vec<u8> {
    let input = input
        .iter()
        .map(|&x| KoalaBear::from_canonical_u32(x))
        .collect::<Vec<_>>();
    let mut state = [KoalaBear::ZERO; WIDTH];
    let mut chunks = input.chunks_exact(RATE);

    let hasher = poseidon2_init();
    for chunk in &mut chunks {
        state[..RATE].clone_from_slice(chunk);
        hasher.permute_mut(&mut state);
    }

    let remainder = chunks.remainder();
    if !remainder.is_empty() {
        state[..remainder.len()].clone_from_slice(remainder);
        hasher.permute_mut(&mut state);
    }

    state
        .into_iter()
        .map(|x| x.as_canonical_u32().to_le_bytes())
        .flatten()
        .collect()
}
