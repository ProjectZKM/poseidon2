use crate::{RATE, WIDTH, FIELD_SIZE};

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
#[inline(always)]
fn permute(state: &mut [u32; 16]) {
    unsafe {
        syscall_poseidon2_permute(state.as_mut_ptr());
    }
}

#[inline(always)]
pub(crate) fn poseidon2_sponge(padded_input: &[u8], state: &mut [u32; WIDTH]) {
    let chunk_count = padded_input.len() / FIELD_SIZE;
    let mut i = 0;

    while i + RATE <= chunk_count {
        for j in 0..RATE {
            let byte_offset = (i + j) * FIELD_SIZE;
            state[j] = bytes_to_u32_fast(&padded_input[byte_offset..byte_offset + FIELD_SIZE]);
        }
        permute(state);

        i += RATE;
    }

    if i < chunk_count {
        let remaining = chunk_count - i;

        for j in 0..remaining {
            let byte_offset = (i + j) * FIELD_SIZE;
            state[j] = bytes_to_u32_fast(&padded_input[byte_offset..byte_offset + FIELD_SIZE]);
        }
        permute(state);
    }
}

#[inline(always)]
fn bytes_to_u32_fast(bytes: &[u8]) -> u32 {
    match bytes.len() {
        3 => (bytes[0] as u32) | ((bytes[1] as u32) << 8) | ((bytes[2] as u32) << 16),
        2 => (bytes[0] as u32) | ((bytes[1] as u32) << 8),
        1 => bytes[0] as u32,
        _ => 0,
    }
}
