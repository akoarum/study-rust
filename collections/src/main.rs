use std::collections::HashMap;

// fn main() {
//     enum SpreadsheetCell {
//         Int(i32),
//         Float(f64),
//         Text(String),
//     }

//     let row = vec![
//         SpreadsheetCell::Int(3),
//         SpreadsheetCell::Float(10.111),
//         SpreadsheetCell::Text(String::from("blue on blue")),
//     ];

//     let mut s1 = String::from("Hello");
//     let s2 = String::from("World");
//     let s3 = format!("{} {}", s1, s2);
//     println!("{}", s3);

//     let s = "憂鬱";
//     for i in s.bytes() {
//         println!("{}", i);
//     }

//     let mut scores = HashMap::new();
//     scores.insert(String::from("Marines"), 10);
//     scores.entry(String::from("Tigers")).or_insert(1);
//     scores.entry(String::from("Marines")).or_insert(10);
//     scores.insert(String::from("Tigers"), 0);
//     scores.insert(String::from("Marines"), 10);
//     scores.insert(String::from("Tigers"), 1);
//     scores.insert(String::from("Marines"), 3);
//     scores.insert(String::from("Tigers"), 2);
//     scores.insert(String::from("Marines"), 33);
//     scores.insert(String::from("Tigers"), 4);

//     println!("{:?}", scores);

//     let field_name = String::from("Favorite color");
//     let field_value = String::from("Blue");

//     let mut map = HashMap::new();
//     map.insert(field_name, field_value);

//     let text = "hello world wonderful world";
//     let mut map = HashMap::new();

//     for word in text.split_whitespace() {
//         let count = map.entry(word).or_insert(0);
//         *count += 1;
//     }

//     println!("{:?}", map);
// }

fn main() {
    let int_list = vec![1, 2, 2, 1, 5, 9, 12, 5, 98, 2, 34, 7, 40];
    let average = get_average(&int_list);
    let median = get_median(&int_list);
    let mode = get_mode(&int_list);
    println!("{}", average);
    println!("{}", median);
    println!("{}", mode);
}

fn get_average(list: &Vec<i32>) -> i32 {
    let mut total: i32 = 0;
    for i in list {
        total += i;
    }
    total / list.len() as i32
}

fn get_median(list: &Vec<i32>) -> i32 {
    let mut l = list.clone();
    let length = l.len();
    l.sort();

    if length % 2 == 0 {
        return (l[length / 2] + l[length / 2 + 1]) / 2;
    }

    l[(length - 1) / 2 + 1] as i32
}

fn get_mode(list: &Vec<i32>) -> i32 {
    let mut l = list.clone();
    let mut map = HashMap::new();
    let mut max = 0;
    let mut compare_int = 0;

    l.sort();

    for i in l {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

    for (key, value) in map {
        if value > compare_int {
            compare_int = value;
            max = key;
        }
    }

    max
}
