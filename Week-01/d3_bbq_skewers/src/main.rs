fn main() {
    let grill = vec! [
        "--oooo-ooo--",
        "--xx--x--xx--",
        "--o---o--oo--",
        "--xx--x--ox--",
        "--xx--x--ox--"
    ];

    let (meat, veg) = count_skewers(grill);
    println!("Meat: {}, Veg: {}", meat, veg);
}


fn count_skewers(grill: Vec<&str>) -> (i32, i32) {
    let mut meat_count = 0;
    for skewer in &grill{
        if skewer.contains("x"){
            meat_count += 1;
        }
    }

    (meat_count, grill.len() as i32 - meat_count)
}