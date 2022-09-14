pub fn run() {
    println!("Hello World from print.rs");

    println!("a {} b", 1);

    println!("{} > {} ?", 1, 2);

    println!("{0} and {0} are the same", 2);

    println!("{name} of the {t}", name="Shrey", t=2);

    println!("Binary: {:b} Hex: {:x}, Octal: {:o}", 11, 10, 10);

    println!("{:?}", (true, 100, "hello"));
}