// use std::collections::HashMap;
// fn main() {
//     let socks = "AAABBB";
//     let pairs = sock_pairs(socks);
//     println!("Pairs: {}", pairs);
//
//     let new_socks = "CABBACCC";
//     let new_pairs = sock_pairs(new_socks);
//     println!("Pairs: {}", new_pairs);
// }
//
// fn sock_pairs(socks: &str) -> i32{
//     let mut counts = HashMap::new();
//     if socks.len() == 0{
//         return 0;
//     }
//     for c in socks.chars(){
//         *counts.entry(c).or_insert(0) += 1;
//     }
//     let mut pairs = 0;
//     for (key, value) in counts.iter(){
//         pairs += value / 2;
//     }
//     pairs
// }

use std::collections::HashMap;

fn main() {
    let socks = ["AAABBB", "CABBACCC"];
    for sock in socks.iter() {
        println!("Pairs: {}", sock_pairs(sock));
    }
}

fn sock_pairs(socks: &str) -> i32 {
    let counts = socks.chars().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    });
    counts.values().map(|&v| v / 2).sum()
}