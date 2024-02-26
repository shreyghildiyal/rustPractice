mod sorting;
use std::io;

use crate::sorting::bubble_sort;
use crate::sorting::merge_sort;

fn main() {
    // sorting_tests();

    input_tests();
}

fn input_tests() {
    // todo!();

    println!("enter array length");

    let arr_len: i16 = read_i16();
    // io::stdin().read_line(&mut arrLen);

    println!("please enter {} numbers ", arr_len);

    let mut arr: Vec<i16> = Vec::new();

    for i in 0..arr_len {
        println!("please enter number at position {} ", i);
        arr.push(read_i16());
    }

    println!("original arr {:?}", arr);

    let mut bubble_arr = arr.clone();
    bubble_sort::sort_arr_bubble(&mut bubble_arr);
    println!("bubble sorted {:?}", bubble_arr);

    let merge_arr = merge_sort::sorted_vec(&mut arr);
    println!("merge sorted {:?}", merge_arr);
}

fn read_i16() -> i16 {
    let mut num_str = String::new();
    io::stdin()
        .read_line(&mut num_str)
        .expect("Error reading user input");

    let num: i16 = num_str.trim().parse().expect("Invalid input");

    return num;
}

#[test]
fn test_merge_sort_already_sorted() {
    let mut input = vec![1, 2, 3, 4, 5];
    let expected_output = vec![1, 2, 3, 4, 5];
    let output = merge_sort::sorted_vec(&mut input);

    assert_eq!(
        output.len(),
        expected_output.len(),
        "Testing whether the returned vector has the same length"
    );

    for i in 0..expected_output.len() {
        assert_eq!(
            output[i], expected_output[i],
            "tested whether the val at index {} matches",
            i
        );
    }
}

#[test]
fn test_merge_sort_reverse_sorted() {
    let mut input = vec![5, 4, 3, 2, 1];
    let expected_output = vec![1, 2, 3, 4, 5];
    let output = merge_sort::sorted_vec(&mut input);

    assert_eq!(
        output.len(),
        expected_output.len(),
        "Testing whether the returned vector has the same length"
    );

    for i in 0..expected_output.len() {
        assert_eq!(
            output[i], expected_output[i],
            "tested whether the val at index {} matches",
            i
        );
    }
}
