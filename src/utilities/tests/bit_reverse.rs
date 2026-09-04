use crate::utilities::bit_reverse::{bit_reverse, count_bits, reverse_bits};

#[test]
fn counts_the_bits_of_a_power_of_two() {
    assert_eq!(count_bits(1), 0);
    assert_eq!(count_bits(2), 1);
    assert_eq!(count_bits(8), 3);
    assert_eq!(count_bits(1024), 10);
}

#[test]
#[should_panic(expected = "Size must be a power of two.")]
fn rejects_a_size_that_is_not_a_power_of_two() {
    count_bits(6);
}

#[test]
fn reads_three_bit_indices_backwards() {
    let reversed: Vec<usize> = (0..8).map(|index| reverse_bits(index, 3)).collect();
    assert_eq!(reversed, vec![0, 4, 2, 6, 1, 5, 3, 7]);
}

#[test]
fn reorders_eight_parts() {
    let mut values: Vec<f64> = (0..8).map(|index| index as f64).collect();
    bit_reverse(&mut values);
    assert_eq!(values, vec![0.0, 4.0, 2.0, 6.0, 1.0, 5.0, 3.0, 7.0]);
}

#[test]
fn reordering_twice_gives_back_the_original() {
    let original: Vec<f64> = (0..8).map(|index| index as f64).collect();
    let mut values = original.clone();
    bit_reverse(&mut values);
    bit_reverse(&mut values);
    assert_eq!(values, original);
}

#[test]
fn short_inputs_are_left_alone() {
    let mut one = vec![9.0];
    bit_reverse(&mut one);
    assert_eq!(one, vec![9.0]);
}

#[test]
fn tiled_sizes_land_where_the_reversed_index_says() {
    for bit_count in 10..16u32 {
        let size = 1usize << bit_count;
        let mut values: Vec<f64> = (0..size).map(|index| index as f64).collect();
        bit_reverse(&mut values);
        for (index, part) in values.iter().enumerate() {
            let expected = reverse_bits(index, bit_count) as f64;
            assert_eq!(*part, expected, "size {} at {}", size, index);
        }
    }
}

#[test]
fn tiled_sizes_come_back_after_two_passes() {
    for bit_count in 10..14u32 {
        let size = 1usize << bit_count;
        let original: Vec<f64> = (0..size).map(|index| index as f64).collect();
        let mut values = original.clone();
        bit_reverse(&mut values);
        bit_reverse(&mut values);
        assert_eq!(values, original, "size {}", size);
    }
}
