struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}


#[derive(Debug)] // so we can inspect the state
enum UsState {
    Alabama,
    Alaska,
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
            println!("Lucky penny!");
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

// enums can have different types:
enum Message {
    Quit, // no data
    Move {x: i32, y: i32}, // struct
    Write(String), // String
    ChangeColor(i32, i32, i32), // tuple structs
}

impl Message {
    fn call(&self) {
        // method body defined here
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    // must cover Some and None cases
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();

    // Some and None variants from Option<T> can be used without Option:: prefix
    let some_number = Some(5);
    let some_char = Some('e');
    let absent_num: Option<i32> = None;
    dbg!(absent_num);

    value_in_cents(Coin::Quarter(UsState::Alabama));
    
    // ------- MATCHING WITH Option<T> ---------------------
    // Some and None cases MUST be covered!

    // ---------- CATCH-ALL PATTERNS -----------------------

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn reroll() {}

    let dice_roll = 9;
    match dice_roll {
        // this is exhaustive bc it explicitly ignores other cases
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }

    // ----------------- IF-LET -----------------------------
    // lets you handle a value that matches one pattern, and ignores the rest
    let config_max = Some(3u8);

    // long way
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    // short way
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
}

fn returnCoin(coin: Coin) -> u8 {
    let mut count: u8 = 0;
    // use "if let" for matching only one condition
    if let Coin::Quarter(state) = coin {
        return 1;
    }
    else if let Coin::Dime = coin {
        return 2;
    }
    else {
        count+1 // return an expression
    }
    // count = count + 1; // this is invalid bc its a statement
}
