use std::io;
fn main(){
    let mut input = String::new();
    println!("Enter the no. of rows:");
    io::stdin().read_line(&mut input).expect("failed");
    let number: usize = input.trim().parse().expect("please type a number");
    let ch = 'A';
    for i in 0..number {
        for j in 0..=i {
            print!("{}",(ch as u8 + j as u8) as char);
        }
        println!("");
    }
}
