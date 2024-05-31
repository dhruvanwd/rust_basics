// fn main() {
//     println!("Hello world");
//     let a = {a:10};
//     let b = {b:15};
//     println!("{}", add(a, b));
// }

// fn add(num1: i32, num2: i32) -> i32 {
//     return num1 + num2;
// }



fn main() {
    println!("Hello world");
    let a = (10,);
    let b = (15,);
    println!("{}", add(a, b));
}

fn add((num1,): (i32,), (num2,): (i32,)) -> i32 {
    num1 + num2
}
