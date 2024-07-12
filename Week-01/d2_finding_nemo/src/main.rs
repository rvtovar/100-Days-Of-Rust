fn main() {
    println!("{}", find_nemo("I am finding Nemo !"));
    println!("{}", find_nemo("I am finding NemoD !"));
}

fn find_nemo(sent: &str) -> String{
    let words: Vec<&str> = sent.split_whitespace().collect();
    for (i, word) in words.iter().enumerate(){
        if word == &"Nemo"{
            return format!("I found Nemo at {}!", i+1);
        }
    }
    return "I can't find Nemo :(".to_string();
}