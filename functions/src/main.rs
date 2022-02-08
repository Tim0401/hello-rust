fn main() {
    println!("Hello, world!");
    // 3.3 関数
    another_function(5,6);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);

    // 式と文
    let a = 5;
    // 式の返り値が入る
    let b = {
        // 文
        let x = 3;
        // セミコロンがあると文
        x + 1;
        // 式
        x + 2
    };
    println!("The value of a is: {}", a);
    println!("The value of b is: {}", b);

    // 返り値のある関数
    println!("The value of five() is: {}", five());
    println!("The value of plus_one(1) is: {}", plus_one(1));
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
    // error セミコロンをつけると文になる
    // x + 1;
}