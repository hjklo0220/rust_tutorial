fn main() {
    let x = 5;
    let x = x + 1;
    println!("value of x is: {x}");
    
    {
        let x = x * 2;
        println!("value of x is: {x}");
    }
    println!("value of x is: {x}");

    // let mut spaces = "    ";
    // spaces = spaces.len();

    let spaces = "    ";
    let spaces = spaces.len();
    
    println!("{}", spaces)
}