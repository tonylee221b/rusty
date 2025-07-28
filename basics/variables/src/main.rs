// fn main() {
//     let mut x = 5;
//     println!("The value of x is: {x}");
//     x = 6;
//     println!("The value of x is: {x}");
// }

fn main() {
    let x = 5;
    let x = x + 1; // << shadowing

    {
        let x = x * 2; // << shadowing
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}")
}
