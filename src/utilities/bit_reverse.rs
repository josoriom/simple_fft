use crate::Complex;

/// Applies the bit reversal permutation  in-place
#[inline]
pub(crate) fn bit_reverse(data: &mut [Complex]) {
    if data.len() < 2 {
        return;
    }
    let bit_count = count_bits(data.len());
    for index in 0..data.len() {
        let target = reverse_bits(index, bit_count);
        if index < target {
            data.swap(index, target);
        }
    }
}

/// Number of bits an index needs to be log2(size). Panics unless size is a power of two.
#[inline]
pub(crate) fn count_bits(size: usize) -> u32 {
    assert!(size.is_power_of_two(), "Size must be a power of two.");
    size.trailing_zeros()
}

/// Reads the bits of index backwards!
#[inline]
pub(crate) fn reverse_bits(index: usize, bits: u32) -> usize {
    index.reverse_bits() >> (usize::BITS - bits)
}
