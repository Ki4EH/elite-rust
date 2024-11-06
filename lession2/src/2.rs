use std::io;
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let _n: i32 = input.trim().parse().unwrap();
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let sequence: Vec<i32> = input.split_whitespace().map(|x| x.parse().unwrap()).collect(); 
    let mut max_sum = sequence[0]; 
    let mut current_sum = 0;
    
    let mut max_with_flip = sequence[0]; 
    let mut current_with_flip = 0;
    
    for num in sequence {
        current_sum = current_sum.max(0) + num;
        max_sum = max_sum.max(current_sum);
        
        current_with_flip = (current_with_flip + num.max(-num)).max(current_sum);
        max_with_flip = max_with_flip.max(current_with_flip);
    }
    println!("{}", max_with_flip.max(max_sum))
}