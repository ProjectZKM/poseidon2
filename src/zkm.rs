use crate::{RATE, WIDTH};

extern "C" {
    fn syscall_poseidon2_permute(w: *mut u32);
}

/// Executes the Poseidon2 permutation on the given state.
///
/// ### Safety
///
/// The caller must ensure that `state` is valid pointer to data that is aligned along a four
/// byte boundary.
///
/// As this module is only active the with `target = zkvm`, this is always true.
#[inline]
fn permute(state: &mut [u32; 16]) {
    unsafe {
        syscall_poseidon2_permute(state.as_mut_ptr());
    }
}

#[inline]
pub(crate) fn poseidon2_sponge(input: &[u32]) -> Vec<u8> {
    let mut state = [0u32; WIDTH];
    let mut chunks = input.chunks_exact(RATE);

    for chunk in &mut chunks {
        state[..RATE].clone_from_slice(chunk);
        permute(&mut state);
    }

    let remainder = chunks.remainder();
    if !remainder.is_empty() {
        state[..remainder.len()].clone_from_slice(remainder);
        permute(&mut state);
    }

    state
        .into_iter()
        .map(|x| x.to_le_bytes())
        .flatten()
        .collect()
}
