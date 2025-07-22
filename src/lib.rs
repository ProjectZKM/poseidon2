cfg_if::cfg_if! {
    if #[cfg(target_os = "zkvm")] {
        mod zkm;
        use zkm::poseidon2_sponge;
    } else {
        mod permute;
        use permute::poseidon2_sponge;
    }
}

const WIDTH: usize = 16; // Number of field elements in the Poseidon2 state
const RATE: usize = 8; // Number of field elements processed per round
const DEFAULT_OUT_FIELD_LEN: usize = 8; // Number of field elements in the output
const FIELD_SIZE: usize = 3; // Number of bytes can be safely converted to field element

// Perform the Poseidon2 hash on the given input
/// The default poseidon2 implementation uses a fixed output length of 32 bytes.
pub fn poseidon2(input: &[u8]) -> [u8; 32] {
    // The output length is fixed to 32 bytes
    let result = poseidon2_impl(input, DEFAULT_OUT_FIELD_LEN * 4);

    result.try_into().unwrap()
}

pub fn poseidon2_128(input: &[u8]) -> [u8; 16] {
    // The output length is fixed to 16 bytes
    let result = poseidon2_impl(input, 16);

    result.try_into().unwrap()
}

#[inline]
pub fn poseidon2_impl(input: &[u8], out_byte_len: usize) -> Vec<u8> {
    let field_input = padding(input);
    let result = poseidon2_sponge(&field_input);

    result[..out_byte_len].to_vec()
}

#[inline]
fn padding(input: &[u8]) -> Vec<u32> {
    let l = input.len();
    let mut padded_input = input.to_vec();
    let new_size = (l + FIELD_SIZE) / FIELD_SIZE * FIELD_SIZE;
    padded_input.resize(new_size, 0);

    // Pad the input to a multiple of 3 bytes
    // Pad 1*01
    if l % FIELD_SIZE == FIELD_SIZE - 1 {
        padded_input[l] = 0b10000001;
    } else {
        padded_input[l] = 1;
        padded_input[new_size - 1] = 0b10000000;
    }

    bytes_to_field_elements(&padded_input)
}

// each 3 bytes of input can be safely converted to a field element
#[inline]
fn bytes_to_field_elements(bytes: &[u8]) -> Vec<u32> {
    bytes
        .chunks(3)
        .map(|chunk| {
            let mut value = 0u32;
            for (i, &byte) in chunk.iter().enumerate() {
                value |= (byte as u32) << (i * 8);
            }
            value
        })
        .collect()
}
