fn find_first(arr: &[i8], key: i8) -> i32 {
    let mut left: isize = 0;
    let mut right: isize = arr.len() as isize - 1;
    let mut result: i32 = -1;

    while left <= right {
        let mid = left + (right - left) / 2;
        let val = arr[mid as usize];
        if val == key {
            result = mid as i32;
            right = mid - 1;
        } else if val < key {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    result
}

fn find_last(arr: &[i8], key: i8) -> i32 {
    let mut left: isize = 0;
    let mut right: isize = arr.len() as isize - 1;
    let mut result: i32 = -1;

    while left <= right {
        let mid = left + (right - left) / 2;
        let val = arr[mid as usize];
        if val == key {
            result = mid as i32;
            left = mid + 1;
        } else if val < key {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    result
}

fn find(arr: &[i8], key: i8) -> [i32; 2] {
    [find_first(arr, key), find_last(arr, key)]
}

fn main() {
    let arr: [i8; 6] = [5, 7, 7, 8, 8, 10];
    let key = 5;

    println!("{:?}", find(&arr, key));
}