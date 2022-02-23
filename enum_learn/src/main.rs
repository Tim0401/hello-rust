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
}
