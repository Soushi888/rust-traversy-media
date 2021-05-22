use std::mem::size_of_val;

// Arrays - Fixed list where elements are the same data types
pub fn run() {
    let mut numbers: [i8; 5] = [1, 2, 3, 4, 5];
    numbers[2] = 20;

    println!("Array : {:?}", numbers);

    // Get single val
    println!("First number : {}", numbers[0]);

    // Get array length
    println!("Array length : {}", numbers.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", size_of_val(&numbers));

    // Get slice
    let slice: &[i8] = &numbers[1..3];
    println!("{:?}", slice);
}
