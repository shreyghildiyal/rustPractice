pub fn sort_arr_bubble<T: Ord>(src_arr: &mut Vec<T>)
where
    T: Copy,
{
    for i in 0..src_arr.len() {
        for j in (i + 1)..src_arr.len() {
            // println!("i={}, j={}", i, j);

            if src_arr[j] < src_arr[i] {
                let temp = src_arr[i];
                src_arr[i] = src_arr[j];
                src_arr[j] = temp;
            }

            // let mut temp i32 = src_arr.get(i)
        }
    }
}

// pub fn sort_arr_bubble(src_arr: &mut Vec<i16>) {
//     for i in 0..src_arr.len() {
//         for j in (i + 1)..src_arr.len() {
//             // println!("i={}, j={}", i, j);

//             if src_arr[j] < src_arr[i] {
//                 let temp = src_arr[i];
//                 src_arr[i] = src_arr[j];
//                 src_arr[j] = temp;
//             }

//             // let mut temp i32 = src_arr.get(i)
//         }
//     }
// }
