use std::option::Option;

fn sum(s: &[u32]) -> Option<u32> {
    let mut sum = 0;
    for item in s {
        sum = sum + item;
    }
    Some(sum)
}
fn main() {
    let a = [1, 2, 3, 4];
    let result = sum(&a);
    println!("result is {}", result.unwrap());
}
