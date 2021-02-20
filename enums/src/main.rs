#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Hawaii,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn main() {
    let mut count = 0;
    let coin = Coin::Penny;

    if let Coin::Quarter(state) = &coin {
        println!("State quater from {:?}!", state);
    } else {
        count += 1;
    }

    value_in_cents(Coin::Quarter(UsState::Hawaii));

    println!("count value is : {}", count);

    let some_u8_value = Some(3);
    if let Some(3) = some_u8_value {
        println!("Three");
    }
}
