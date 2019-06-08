// Variable are immutable by default.

pub fn run() {
    let name = "Ranjeet";
    let mut age = 29;
    // name = "ranjeet"; // Not allowed
    age = 30;
    println!("My name is {} and I am {}", name, age);

    // Define Constant
    const ID: i32 = 001;
    println!("ID : {}", ID);

    // Assing multiple vars
    let (my_name, my_age) = ("Ranjeet", 30);
    println!("{} is {} ", my_name, my_age);
}