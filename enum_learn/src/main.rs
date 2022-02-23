fn main() {
    // enum IpAddr {
    //     V4,
    //     V6,
    // }
    // enum IpAddr {
    //     V4(String),
    //     V6(String),
    // }
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    impl Message {
        fn call(&self) {
            // method body would be defined here
            // メソッド本体はここに定義される
        }
    }
    let m = Message::Write(String::from("hello"));
    m.call();
    println!("{:?}", m);

    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // let sum = x + y; // error: type mismatch

    // match

    #[derive(Debug)] // すぐに州を点検できるように
    enum UsState {
        Alabama,
        Alaska,
        // ... などなど
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }
    let money = Coin::Penny;

    fn value_in_cents(coin: Coin) -> u32 {
        match coin {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            }
        }
    }
    println!("{:?}", value_in_cents(money));
    println!("{:?}", value_in_cents(Coin::Quarter(UsState::Alaska)));

    // option<T>
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?}", six);
    println!("{:?}", none);

    // default
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => println!("_"),
    }

    // if let
    let some_u8_value = Some(3u8);
    // 以下の2つの文は同じ意味
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }
    if let Some(3) = some_u8_value {
        println!("three");
    }
}
