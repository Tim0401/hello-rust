fn main() {
    // match

    // if let
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        // あなたのお気に入りの色、{}を背景色に使用します
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        // 火曜日は緑の日！
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            // 紫を背景色に使用します
            println!("Using purple as the background color");
        } else {
            // オレンジを背景色に使用します
            println!("Using orange as the background color");
        }
    } else {
        // 青を背景色に使用します
        println!("Using blue as the background color");
    }

    // while let
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    // for
    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    // tupple
    let (x, y, z) = (1, 2, 3);
    // error
    // let (x, y) = (1, 2, 3);

    // function
    let point = (3, 5);
    print_coordinates(&point);

    // 論駁可能はNG
    // let Some(x) = some_option_value;

    // 論駁可能OK
    let some_option_value: Option<&str> = None;
    if let Some(x) = some_option_value {
        println!("{}", x);
    }

    if let x = 5 {
        println!("{}", x);
    };
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    // 現在の位置: ({}, {})
    println!("Current location: ({}, {})", x, y);
}
