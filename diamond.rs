use std::io;
fn main(){
    let mut input = String::new();
    println("Enter the no. of rows");
    io::stdin().read_line(&mut input).expect("input fail");
    let n: usize = input.trim().parse().expect("only number");
    for i in 0..n {
        
    }
}