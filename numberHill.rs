use std::io;

fn main(){
    let mut input = String::new();
    println!("Enter the no. of rows");
    io::stdin().read_line(&mut input).expect("Has to be a number");
    let n: usize = input.trim().parse().expect("some error");
    // let mut space: usize = 2 * (n-1);
    for i in 1..=n{
        for j in 1..=i{
            print!("{j}");
        }
        let space: usize = 2 * (n-i);
        for _j in 1..=space {
            print!(" ");
                              }
        for j in (1..=i).rev() {
            print!("{j}");
        }
        println!("");
        // space-=2;                    
        }

}