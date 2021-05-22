pub fn run() {
    // tableau de données de différents types
    let person1: (&str, &str, i8) = ("Sacha", "Longueuil", 28);
    let person2 = ("Josiane", "Saint-Honoré", 27);

    println!("{} is from {} et is {} years old.", person1.0, person1.1, person1.2 );
    println!("{} is from {} et is {} years old.", person2.0, person2.1, person2.2 );
}
