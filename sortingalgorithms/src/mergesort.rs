pub fn merge_sort(arr: &mut [i32]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }

    let mid = len / 2;
    merge_sort(&mut arr[0..mid]);
    merge_sort(&mut arr[mid..len]);
    let mut merged = arr.to_vec();
    merge(&arr[0..mid], &arr[mid..len], &mut merged);
    arr.copy_from_slice(&merged);
}

fn merge(left: &[i32], right: &[i32], merged: &mut [i32]) {
    let mut left_idx = 0;
    let mut right_idx = 0;
    let mut merged_idx = 0;

    while left_idx < left.len() && right_idx < right.len() {
        if left[left_idx] <= right[right_idx] {
            merged[merged_idx] = left[left_idx];
            left_idx += 1;
        } else {
            merged[merged_idx] = right[right_idx];
            right_idx += 1;
        }
        merged_idx += 1;
    }

    if left_idx < left.len() {
        merged[merged_idx..].copy_from_slice(&left[left_idx..]);
    } else {
        merged[merged_idx..].copy_from_slice(&right[right_idx..]);
    }
}