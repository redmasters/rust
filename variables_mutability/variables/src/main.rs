fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!("{THREE_HOURS_IN_SECONDS}");

    let y = 7;
    let y = y + 1;

    {
        let y = x * 2;
        print!("The value of y in inner scope is: {x}");
    }

    println!("The value of x is: {x}")
}
