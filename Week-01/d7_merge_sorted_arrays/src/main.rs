/*
Merge two sorted arrays into a single sorted array.
 */

fn merge_vectors(mut vec1: Vec<i32>, vec2: Vec<i32>) -> Vec<i32>{
    for i in vec2{
        vec1.push(i);
    }
    vec1.sort();
    vec1
}
fn main() {
    let nums1 = vec![1,2,3];
    let nums = vec![2,5,6];

    let result = merge_vectors(nums1, nums);
    println!("{:?}", result);
}
