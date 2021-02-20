fn main() {
    let x = plus(10, 5);
    println!("The value of x : {}", x);
}

fn plus(x: i32, y: i32) -> i32 {
    x + y
}
