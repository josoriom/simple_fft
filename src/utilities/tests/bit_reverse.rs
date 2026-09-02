use crate::utilities::bit_reverse::{bit_reverse, count_bits, reverse_bits};
use crate::Complex;

fn points(values: &[f64]) -> Vec<Complex> {
    values.iter().map(|value| Complex::new(*value, 0.0)).collect()
}

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
fn reorders_eight_points() {
    let mut data = points(&[0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0]);
    bit_reverse(&mut data);
    let order: Vec<f64> = data.iter().map(|point| point.real).collect();
    assert_eq!(order, vec![0.0, 4.0, 2.0, 6.0, 1.0, 5.0, 3.0, 7.0]);
}

#[test]
fn reordering_twice_gives_back_the_original() {
    let original = points(&[0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0]);
    let mut data = original.clone();
    bit_reverse(&mut data);
    bit_reverse(&mut data);
    assert_eq!(data, original);
}

#[test]
fn short_inputs_are_left_alone() {
    let mut one = points(&[9.0]);
    bit_reverse(&mut one);
    assert_eq!(one, points(&[9.0]));
}

#[test]
fn tiled_sizes_land_where_the_reversed_index_says() {
    for bit_count in 10..16u32 {
        let size = 1usize << bit_count;
        let mut data: Vec<Complex> = (0..size).map(|i| Complex::new(i as f64, 0.0)).collect();
        bit_reverse(&mut data);
        for index in 0..size {
            let expected = reverse_bits(index, bit_count) as f64;
            assert_eq!(data[index].real, expected, "size {} at {}", size, index);
        }
    }
}

#[test]
fn tiled_sizes_come_back_after_two_passes() {
    for bit_count in 10..14u32 {
        let size = 1usize << bit_count;
        let original: Vec<Complex> = (0..size).map(|i| Complex::new(i as f64, -1.0)).collect();
        let mut data = original.clone();
        bit_reverse(&mut data);
        bit_reverse(&mut data);
        assert_eq!(data, original, "size {}", size);
    }
}
