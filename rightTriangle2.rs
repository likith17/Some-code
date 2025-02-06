use std::io;

fn main(){
    let mut input = String::new();
    println!("Enter the no. of rows:");
    io::stdin().read_line(&mut input).expect("Error");
    let num: usize = input.trim().parse().expect("NUmber!");
    let mut ch = 'A';
    for i in 0..num{
        for _j in 0..=i{
            print!("{}",ch );
        }
        println!("");
        ch = (ch as u8 + 1) as char;
    }

}