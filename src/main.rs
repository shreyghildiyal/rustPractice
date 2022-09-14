// mod print;
// mod vars;
// mod types;
// mod strings;


fn main() {
    // print::run();
    // strings::run();

    println!("args: {:?}", std::env::args());

    let args: Vec<String> = std::env::args().collect();

    let key1 = args[0];
    // let s: str = key1.;
    
    println!("key1: {}", key1);
    println!("key2: {}", args.next().unwrap());

    println!("args length {}", args.len());
}