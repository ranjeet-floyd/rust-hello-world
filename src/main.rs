
mod file;
mod print;

fn main() {
    println!("Hello, world!");
    print::run();
    println!("{} practice {}", "Ranjeet", "Rust");

    // Placeholder
    println!(
        "{name} practice {language}",
        name = "Ranjeet",
        language = "Rust"
    );

    println!("{:?}", (12, true, "Hello"));

    // from file module
    file::run();

}
