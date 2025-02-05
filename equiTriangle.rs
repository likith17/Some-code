
use std::io;
fn main(){
let mut input = String::new();
println!("enter number");    
io::stdin().read_line(&mut input).expect("input fail");
let n: usize = match input.trim().parse(){
    Ok(num) => num,
    Err(_) => {
        println!("ENter number!");
        return;
    }
};
for i in 0..n {
for _ in 0..(n-i-1) {
    print!(" ");
    
    
}

for _ in 0..(2*i+1) {
    print!("*");
}
println!("");
}
}

