use generics::{self, NewsArticle, Tweet, Summary};

fn get_largest<T: PartialOrd + Copy>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn get_longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn get_longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let numbers = vec![34, 50, 25, 100, 65];
    let result = get_largest(&numbers);

    println!("The largest number is {}", result);
    assert_eq!(result, &100);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = get_largest(&char_list);
    println!("The largest char is {}", result);
    assert_eq!(result, &'y');

    let tweet = Tweet {
        username: String::from("marines"),
        content: String::from("yes, this is tweet."),
        reply: false,
        retweet: false,
    };

    println!("{}", tweet.summarize());

    let string1 = String::from("abcd");
    {
        let string2 = "xyz";
        let result = get_longest(string1.as_str(), string2);
        println!("The longest is {}", result);
    }
}
