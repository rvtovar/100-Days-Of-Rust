fn progress(miles: &Vec<i32>) -> i32{
    if miles.len() < 2{
        return 0;
    }
    let mut progress_days = 0;
    let mut total_miles = 0;

    for i in 1..miles.len(){
        if miles[i] > miles[i - 1]{
            progress_days += 1;
        }


    }
    progress_days


}

fn main() {
    let miles = vec![3, 4, 5, 6, 7, 8, 9];
    let result = progress(&miles);
    println!("Result: {}", result);

    let new_miles = vec![6, 5, 4, 3, 2, 9];
    let new_result = progress(&new_miles);
    println!("Result: {}", new_result);
}
