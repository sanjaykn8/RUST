fn sort(mut arr: [u8; 5]) -> [u8; 5] {
    for i in 0..5{
        let mut min_index = i;
        let mut j = i + 1;

        while j < arr.len() {
            if arr[j] < arr[min_index] {
                min_index = j;
            }
            j += 1;
        }

        let temp = arr[min_index];
        arr[min_index] = arr[i];
        arr[i] = temp;
    }

    arr
}

fn main() {
    let arr: [u8; 5] = [3, 2, 5, 1, 4];
    println!("{:?}", sort(arr));
}
