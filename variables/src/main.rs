const TWO: u32 = 1 + 1;
fn main() {
   
    // // let mut x =5;
    // // println!("The value of x is: {x}");
    // // x = 6;
    // // println!("The value of x is: {x}");

    //  println!("{TWO}");
    //  let x = 5;

    // let x = x + 1;

    // {
    //     let x = x * 2;
    //     println!("The value of x in the inner scope is: {x}");
    // }

    // println!("The value of x is: {x}");

    //     let mut spaces = "   ";
    // spaces = spaces.len();
    println!("Kello, world!");
    another_function(5);
    another_function2(5, 'h');
}

fn another_function(x: i32){
    println!("The value of x is: {x}");
}

fn another_function2(x: i32, y: char) {
    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
}
