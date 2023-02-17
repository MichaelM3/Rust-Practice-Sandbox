
fn main() {
    let s = String::from("Hello");
    let length = len_string(&s);
    println!("The length of the string is {length}")
}

fn len_string(message: &String) -> usize {
    message.len()
}

// let temps = [10, 20, 30, 40, 50, 60, 70];
//
// for temp in temps {
//     let result = (temp * 9/5) + 32;
//     println!("{temp} celsius in faranheit is {result}")
// }
// fn another_function(x: i32, y: i32) -> i32 {
//     x + y
// }
