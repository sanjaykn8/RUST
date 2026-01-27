fn main() {
    const ROW:usize = 3;
    const COL:usize = 4;

    let mut arr:[[i8; ROW]; COL] = [[1; ROW]; COL];
    println!("{:?}", arr);

    arr[0][0] = 5;
    println!("{:?}", arr);
}
