use std::mem::size_of_val;

// Vectors - Resizable arrays
pub fn run() {
    let mut numbers: Vec<i8> = vec![1, 2, 3, 4, 5];

    // Reassign value
    numbers[1] = 20;
    numbers[4] = 50;

    // Add value on the vector
    numbers.push(6);

    println!("Vector : {:?}", numbers);

    // Pop off last value
    numbers.pop();

    println!("Vector : {:?}", numbers);

    // Get single val
    println!("First number : {}", numbers[0]);

    // Get array length
    println!("Vector length : {}", numbers.len());

    // Vectors are stack allocated
    println!("Vector occupies {} bytes", size_of_val(&numbers));

    // Get slice
    let slice: &[i8] = &numbers[1..3];
    println!("Slice 2 à 4 : {:?}", slice);

    // Loop through vector values
    for x in numbers.iter() {
        println!("{}", x);
    }

    // Loop and mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("{:?}", numbers);
}
