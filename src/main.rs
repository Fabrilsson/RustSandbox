mod print;
mod vars;
mod types;

fn main() {
    print::run();

    println!("");
    println!("");

    vars::run();

    println!("");
    println!("");

    types::run();

    println!("Hello, world!");
}
