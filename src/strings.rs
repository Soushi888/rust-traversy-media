pub fn run() {
    let mut hello = String::from("Hello ");

    println!("{}", hello);

    hello.push('W'); // push one char
    hello.push_str("orld !"); // push a string

    println!("{}", hello);

    println!("length : {}", hello.len()); // Get length

    println!("Capacity: {}", hello.capacity()); // Capacity in bytes

    println!("Is empty: {}", hello.is_empty()); // Check if empty

    println!("Does contain the word 'World' : {}", hello.contains("World")); // Contain

    println!("{}", hello.replace("World", "You")); // Replace

    // Iterate through string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
    println!("{}", s);

    // Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());
}
