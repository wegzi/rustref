pub fn run() {
    let person: (&str, &str, i8) = ("Brad", "Mass", 8);
    println!("{} is from {} and is {}", person.0, person.1, person.2)
}
