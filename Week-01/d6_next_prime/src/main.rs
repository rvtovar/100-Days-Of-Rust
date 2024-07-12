
fn is_prime(n: u32) -> bool {
    (2..n).all(|i| n % i != 0)
}

fn next_prime(m: u32) -> u32 {
    let mut num = m;
    while !is_prime(num){
        num += 1;
    }
    num
}
fn main() {
    let n = 24;
    println!("Is {} a prime: {}", n, is_prime(n));
    println!("Next prime after {} is {}", n, next_prime(n));
}
