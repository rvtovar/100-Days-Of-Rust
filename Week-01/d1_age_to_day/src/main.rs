fn main() {
    let age = 20;
    let day = age_to_day(age);
    println!("age: {}, day: {}", age, day);
}

fn age_to_day(age:u32) -> u32{
    age * 365
}