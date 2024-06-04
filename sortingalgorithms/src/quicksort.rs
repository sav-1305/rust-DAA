pub fn quick_sort(arr: &mut [i32]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }
    quick_sort_recursive(arr, 0, len - 1);
}

fn quick_sort_recursive(arr: &mut [i32], low: usize, high: usize) {
    if low < high {
        let p = partition(arr, low, high);
        if p > 0 {
            quick_sort_recursive(arr, low, p - 1);
        }
        quick_sort_recursive(arr, p + 1, high);
    }
}

fn partition(arr: &mut [i32], low: usize, high: usize) -> usize {
    let pivot = high;
    let mut i = low;
    for j in low..high {
        if arr[j] < arr[pivot] {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, high);
    i
}