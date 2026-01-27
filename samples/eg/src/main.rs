use std::io;

struct Person{
    name: String
}

fn main(){
    let mut person = Person{ name:String::new()};
    person.name = get_str();
    println!("{}", person.name);
}

fn get_str() -> String{
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("not a string");
    return user_input;
}