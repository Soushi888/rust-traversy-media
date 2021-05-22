pub fn run() {
    let age: i8 = 10;
    let check_id: bool = false;
    let knows_person: bool = true;

    // If/Else
    if age >= 21 && check_id || knows_person {
        println!("Bartender : What would you like to drink ?");
    } else if age < 21 && check_id {
        println!("Bartender : Get out !");
    } else {
        println!("Bartender : Show your Id !")
    }

    // Shorthand If
    let is_major: bool = age >= 21;

    println!("Is major ? {}", is_major);
}
