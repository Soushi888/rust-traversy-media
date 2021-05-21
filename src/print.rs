pub fn run() {
    // Print to console
    println!("Hello from print.rs file !");

    // Basic formatting
    println!("{} have {} cookies", "Sacha", 5);

    // Positional arguments
    println!("{0} is from {1} and {} likes to {2}", "Sacha", "New-Caledonia", "code");

    // Named arguments
    println!("{name} likes to play {activity}.", name = "Sacha", activity = "music");

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    // Basic math
    println!("10 + 10 = {}", 10 + 10);
}
