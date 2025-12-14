use std::io;

fn main(){
    let mut cut = String::new();

    io::stdin().read_line(&mut cut).expect("Didn't get");

    let cut:i128 = cut.trim().parse().expect("Not a number");

    disp(cut);
}

fn disp(pr:i128){
    println!("{pr}");
}