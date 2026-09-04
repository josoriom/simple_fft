use crate::structs::Data;

#[test]
fn reports_the_length_of_either_array() {
    let data = Data {
        real: vec![1.0, 2.0],
        imag: vec![0.0, 0.0],
    };
    assert_eq!(data.len(), 2);
}

#[test]
fn empty_data_is_empty() {
    let data = Data {
        real: vec![],
        imag: vec![],
    };
    assert!(data.is_empty());
}
