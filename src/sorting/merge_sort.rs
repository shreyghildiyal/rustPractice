pub fn sorted_vec<T: Ord>(arr: &mut Vec<T>) -> Vec<T>
where
    T: Copy,
{
    // let sorted_arr = sorted_arr(&mut arr[0..]);
    let sorted_vec: Vec<T> = sorted_arr(&mut arr[0..]).to_vec();
    return sorted_vec;
}

fn sorted_arr<T: Ord>(arr: &mut [T]) -> Vec<T>
where
    T: Copy,
{
    if arr.len() == 1 {
        arr.to_vec()
    } else if arr.len() == 2 {
        let mut arr_copy = arr.to_vec();
        if arr_copy[0] > arr_copy[1] {
            let temp = arr_copy[0];
            arr_copy[0] = arr_copy[1];
            arr_copy[1] = temp;
        }
        arr_copy
    } else {
        let mid = arr.len() / 2;
        let arr_len = arr.len();

        let left_arr = sorted_arr(&mut arr[0..mid]);
        let right_arr = sorted_arr(&mut arr[mid..arr_len]);

        let mut merged_arr = vec![];

        let mut left_index = 0;
        let mut right_index = 0;

        while left_index < left_arr.len() && right_index < right_arr.len() {
            if left_arr[left_index] <= right_arr[right_index] {
                merged_arr.push(left_arr[left_index]);
                left_index = left_index + 1;
            } else {
                merged_arr.push(right_arr[right_index]);
                right_index = right_index + 1;
            }
        }

        while left_index < left_arr.len() {
            merged_arr.push(left_arr[left_index]);
            left_index = left_index + 1;
        }

        while right_index < right_arr.len() {
            merged_arr.push(right_arr[right_index]);
            right_index = right_index + 1;
        }

        return merged_arr;
    }
}
