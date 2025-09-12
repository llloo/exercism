pub fn find(array: &[i32], key: i32) -> Option<usize> {
    if array.len() == 0 || key > *array.last().unwrap() || key < *array.first().unwrap() {
        return None;
    }

    let mut i: usize = 0;
    let mut j: usize = array.len() - 1;
    let mut mid: usize = 0;

    while i <= j {
        mid = (i + j) / 2;
        if array[mid] == key {
            return Some(mid);
        } else if array[mid] < key {
            i = mid + 1;
        } else {
            j = mid - 1;
        }
    }
    None
}
