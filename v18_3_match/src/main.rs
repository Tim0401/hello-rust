fn main() {
    let x = Some(5);
    let y = 10;

    match x {
        // 50だったよ
        Some(50) => println!("Got 50"),
        // マッチしたよ
        Some(y) => println!("Matched, y = {:?}", y),
        // 既定のケース
        _ => println!("Default case, x = {:?}", x),
    }

    // 最後にはx = {}, y = {}
    println!("at the end: x = {:?}, y = {:?}", x, y);

    let x = 1;

    match x {
        // 1か2
        1 | 2 => println!("one or two"),
        // 3
        3 => println!("three"),
        // なんでも
        _ => println!("anything"),
    }

    let x = 5;

    match x {
        // 1から5まで
        1..=5 => println!("one through five"),
        // それ以外
        _ => println!("something else"),
    }

    let x = 'c';

    match x {
        // ASCII文字前半
        'a'..='j' => println!("early ASCII letter"),
        // ASCII文字後半
        'k'..='z' => println!("late ASCII letter"),
        // それ以外
        _ => println!("something else"),
    }

    // 構造体の分配
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    let p = Point { x: 0, y: 7 };

    match p {
        // x軸上の{}
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        // y軸上の{}
        Point { x: 0, y } => println!("On the y axis at {}", y),
        // どちらの軸上でもない: ({}, {})
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }

    // enumの分配
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            // Quit列挙子には分配すべきデータがない
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move { x, y } => {
            println!(
                // x方向に{}、y方向に{}だけ動く
                "Move in the x direction {} and in the y direction {}",
                x, y
            );
        }
        // テキストメッセージ: {}
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!(
                // 色を赤{}, 緑{}, 青{}に変更
                "Change the color to red {}, green {}, and blue {}",
                r, g, b
            )
        }
    }

    // 参照の分配
    let points = vec![
        Point { x: 0, y: 0 },
        Point { x: 1, y: 5 },
        Point { x: 10, y: -3 },
    ];

    let sum_of_squares: i32 = points.iter().map(|&Point { x, y }| x * x + y * y).sum();

    // 構造体とタプルの分配
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    // 値を無視する
    foo(3, 4);

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            // 既存の値の変更を上書きできません
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }
    // 設定は{:?}です
    println!("setting is {:?}", setting_value);

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            // 何らかの数値: {}, {}, {}
            println!("Some numbers: {}, {}, {}", first, third, fifth)
        }
    }

    // 未使用の変数を無視する
    let _x = 5;
    let y = 10;

    // _sが値を束縛するのおでエラー
    // let s = Some(String::from("Hello!"));
    // if let Some(_s) = s {
    //     // 文字列が見つかりました
    //     println!("found a string");
    // }
    // println!("{:?}", s);

    // 値を束縛しない
    let s = Some(String::from("Hello!"));
    if let Some(_) = s {
        println!("found a string");
    }
    println!("{:?}", s);

    // 省略
    let origin = Point3 { x: 0, y: 0, z: 0 };
    match origin {
        Point3 { x, .. } => println!("x is {}", x),
    }

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }

    // 場所が不明瞭なのでコンパイルエラー
    // let numbers = (2, 4, 8, 16, 32);
    // match numbers {
    //     (.., second, ..) => {
    //         println!("Some numbers: {}", second)
    //     }
    // }

    // 参照を奪わずに値を生成する
    let robot_name = Some(String::from("Bors"));
    match robot_name {
        Some(ref name) => println!("Found a name: {}", name),
        None => (),
    }
    println!("robot_name is: {:?}", robot_name);

    // 条件式の追加
    let num = Some(4);
    match num {
        // 5未満です: {}
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {:?}", n),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {:?}", x, y);

    let x = 4;
    let y = false;
    match x {
        // はい
        4 | 5 | 6 if y => println!("yes"),
        // いいえ
        _ => println!("no"),
    }

    // at @演算子 値を検査しつつ、1つのパターン内で変数に保存
    enum Message {
        Hello { id: i32 },
    }
    let msg = Message::Hello { id: 5 };
    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => {
            // 範囲内のidが見つかりました: {}
            println!("Found an id in range: {}", id_variable)
        }
        Message::Hello { id: 10..=12 } => {
            // 別の範囲内のidが見つかりました
            println!("Found an id in another range")
        }
        Message::Hello { id } => {
            // それ以外のidが見つかりました
            println!("Found some other id: {}", id)
        }
    }
}

struct Point {
    x: i32,
    y: i32,
}

struct Point3 {
    x: i32,
    y: i32,
    z: i32,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn foo(_: i32, y: i32) {
    // このコードは、y引数を使うだけです: {}
    println!("This code only uses the y parameter: {}", y);
}
