use std::io;
fn main(){
    let mut input = String::new();
    println!("Enter the no. of rows");
    io::stdin().read_line(&mut input).expect("input fail");
    let n: usize = input.trim().parse().expect("only number");
    for i in 0..n {
    for _ in 0..(n-i-1){
        print!(" ");
    }
    for _ in 0..(2*i+1){
        print!("*");
    }
    println!("");
}       
    for j in 1..n{
        for _ in 0..j{
            print!(" ");
        }
        for _ in 0..(2*(n-j)-1){
            print!("*");
        }
        println!("");
    }
    
}