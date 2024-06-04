pub fn selection_sort(arr: &mut [i32]) {
    let len = arr.len();
    for i in 0..len - 1 {
        let mut smallest = i;
        for j in (i + 1)..len {
            if arr[j] < arr[smallest] {
                smallest = j;
            }
        }
        arr.swap(smallest, i);
    }
}