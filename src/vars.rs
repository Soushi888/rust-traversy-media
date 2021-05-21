pub fn run() {
    let name = "Sacha"; // Immutable variable
    let mut age = 0; // Mutable variable
    age = 2021 - 1993;
    println!("My name is {} and I am {}.", name, age);

    const ID: i32 = 001; // Constant variable
    println!("ID: {}", ID);

    let (my_name, my_age) = ("Sacha", 28); // Multiple assignment
    println!("{} is {}.", my_name, my_age);
}
