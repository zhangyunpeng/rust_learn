pub fn sort(arr: &mut [i32]) {
    sort_helper(arr, 0, arr.len() -1);
}

fn sort_helper(arr: &mut [i32], start: usize, end: usize) {
    if start < end {
        let povit_index = povit(arr, start, end);
        if povit_index > 0 {
            sort_helper(arr, start, povit_index - 1);
        }
        sort_helper(arr, povit_index + 1, end);
    }
}

fn povit(arr: &mut [i32], start: usize, end: usize) -> usize {
    let povit = start;
    let (mut left, mut right) = (start, end);
    while left < right {
        while left < right && arr[right] >= arr[povit] {
            right -= 1;
        }
        while left < right && arr[left] <= arr[povit] {
            left += 1;
        }
        if left != right {
            arr.swap(left, right);
        }
    }
    if left != povit {
        arr.swap(left, povit);
    }
    povit
}