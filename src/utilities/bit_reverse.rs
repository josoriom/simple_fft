const TILE_BITS: u32 = 5;

/// Applies the bit reversal permutation in place to one array of parts.
pub(crate) fn bit_reverse(values: &mut [f64]) {
    if values.len() < 2 {
        return;
    }
    let bit_count = count_bits(values.len());
    if bit_count < 2 * TILE_BITS + 4 {
        swap_one_by_one(values, bit_count);
    } else {
        swap_by_tiles(values, bit_count);
    }
}

/// Number of bits an index needs, log2(size). Panics unless size is a power of two.
#[inline]
pub(crate) fn count_bits(size: usize) -> u32 {
    assert!(size.is_power_of_two(), "Size must be a power of two.");
    size.trailing_zeros()
}

/// Reads the bits of index backwards: with 3 bits, 001 becomes 100.
#[inline]
pub(crate) fn reverse_bits(index: usize, bit_count: u32) -> usize {
    index.reverse_bits() >> (usize::BITS - bit_count)
}

fn swap_one_by_one(values: &mut [f64], bit_count: u32) {
    for index in 0..values.len() {
        let target = reverse_bits(index, bit_count);
        if index < target {
            values.swap(index, target);
        }
    }
}

/// Splits an index into a high tile, a middle, and a low tile, then moves whole tiles
/// through a small buffer that stays in cache. Reads and writes then run along the
/// array in short runs instead of jumping across it once per point.
fn swap_by_tiles(values: &mut [f64], bit_count: u32) {
    let middle_bits = bit_count - 2 * TILE_BITS;
    let tile = 1usize << TILE_BITS;
    let mut buffer = vec![0.0f64; tile * tile * 2];
    let (front, back) = buffer.split_at_mut(tile * tile);

    for middle in 0..(1usize << middle_bits) {
        let reversed_middle = reverse_bits(middle, middle_bits);
        if reversed_middle < middle {
            continue;
        }
        gather(values, front, middle, middle_bits);
        gather(values, back, reversed_middle, middle_bits);
        scatter(values, front, reversed_middle, middle_bits);
        scatter(values, back, middle, middle_bits);
    }
}

fn gather(values: &[f64], buffer: &mut [f64], middle: usize, middle_bits: u32) {
    let tile = 1usize << TILE_BITS;
    let high_shift = middle_bits + TILE_BITS;
    for high in 0..tile {
        let from = (high << high_shift) | (middle << TILE_BITS);
        let into = high << TILE_BITS;
        buffer[into..into + tile].copy_from_slice(&values[from..from + tile]);
    }
}

fn scatter(values: &mut [f64], buffer: &[f64], middle: usize, middle_bits: u32) {
    let tile = 1usize << TILE_BITS;
    let high_shift = middle_bits + TILE_BITS;
    for low in 0..tile {
        let start = (reverse_bits(low, TILE_BITS) << high_shift) | (middle << TILE_BITS);
        for high in 0..tile {
            values[start | reverse_bits(high, TILE_BITS)] = buffer[(high << TILE_BITS) | low];
        }
    }
}
