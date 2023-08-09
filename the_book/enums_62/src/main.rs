fn main() {
    let penny: u8 = value_in_cents(Coin::Penny);
    println!("penny is {}", penny);

    let nickel: u8 = value_in_cents(Coin::Nickel);
    println!("nickel is {}", nickel);

    let dime: u8 = value_in_cents(Coin::Dime);
    println!("dime is {}", dime);

    let quarter: u8 = value_in_cents(Coin::Quarter);
    println!("quarter is {}", quarter);

    let quarter_state: UsState = value_usquarter(UsStateQuarter::Quarter(UsState::Massachusetts));
    println!("quarter state is {:?}", quarter_state);
}


fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => get_nickel_value(),
        Coin::Dime => {
            print_got_dime();
            10
        },
        Coin::Quarter => {
            let value: u8 = get_quarter_value();
            value
        },
    }
}

fn value_usquarter(quarter: UsStateQuarter) -> UsState {
     match quarter {
         UsStateQuarter::Quarter(state) => state,
     }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

enum UsStateQuarter {
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Massachusetts,
    Alaska,
    Hawaii,
    Texas,
    Connecticut,
}

fn get_nickel_value() -> u8 {
    return 5; 
}

fn print_got_dime() {
    println!("got dime!");
}

fn get_quarter_value() -> u8 {
    return 25;
}

