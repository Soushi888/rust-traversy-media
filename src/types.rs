pub fn run() {
    let _y: i64 = 32; // Explicite type

    // Find max size
    println!("{}", i8::MAX);
    println!("{}", i16::MAX);
    println!("{}", i32::MAX);
    println!("{}", i64::MAX);

    // Boolean
    let is_active: bool = true;

    // Get boolean from expression
    let is_greater: bool = 10 > 5;

    // Unique character
    let a1 = 'a';
    let face = '\u{058D}'; // Unicode

    println!("{:?}", (_y, is_active, is_greater, a1, face));
}
