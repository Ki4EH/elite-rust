use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let arr: Vec<i32> = input.trim().split_whitespace().map(|s| s.parse().unwrap()).collect();
    let mut current_sum = arr[0];
    let mut max_sum = arr[0];

    for &x in arr.iter().skip(1) {
        current_sum = current_sum.max(current_sum + x);
        max_sum = max_sum.max(current_sum);
    }

    println!("{}", max_sum);
}