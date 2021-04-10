pub fn run(){
    //Print to console
    println!("Hellp fron print.rs file");

    //Basic Formatting
    println!("{} is from {}", "Fabrício", "Caxias");

    //Positional Arguments
    println!(
        "{0} is from {1} and {0} likets to {2}",
        "Fabrício",
        "Caxias",
        "Code"
    );

    //Names Arguments
    println!(
        "{name} likes to play {activity}",
        name = "Fabrício",
        activity = "Videogames"
    );

    //Placeholder traits
    println!(
        "Binary: {:b}, Hex: {:x}, Octal: {:o}",
        10,
        10,
        10
    );

    //Placeholder for debug trait with touple
    println!("{:?}", (12, true, "hello"));

    //Basic Math
    println!("10 + 10 = {}", 10 + 10);
}