pub fn run() {
    let immutable_string = "immutable";

    let mut mutable_string = String::from("mutable");

    println!("Word: {} Length: {}", immutable_string, immutable_string.len());

    println!("Word: {} Length: {}", mutable_string, mutable_string.len());

    mutable_string.push_str(" World");

    println!("Word: {} Length: {}", mutable_string, mutable_string.len());

    mutable_string.push('!');

    println!("Word: {} Length: {}", mutable_string, mutable_string.len());

    
}