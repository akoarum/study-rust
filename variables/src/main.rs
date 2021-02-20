fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    let x = x * 6;
    println!("The value of x is: {}", x);
    let tup = (500, 6.1, "Hello");
    println!("The value of tup is: {}, {}, {}", tup.0, tup.1, tup.2);
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("The value of arr is: {}", arr[2]);
}
