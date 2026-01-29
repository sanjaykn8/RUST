fn sort(mut arr: [u8; 5]) -> [u8; 5] {
    for i in 1..arr.len() {
        let key = arr[i];
        let mut j = i;

        while j > 0 && arr[j - 1] > key {
            arr[j] = arr[j - 1];
            j -= 1;
        }

        arr[j] = key;
    }
    arr
}

fn main() {
    let arr: [u8; 5] = [3, 2, 5, 1, 4];
    println!("{:?}", sort(arr));
}
