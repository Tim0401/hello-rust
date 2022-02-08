fn main() {
    // 3.1
    // mutable
    let mut x = 5;
    println!("The value of x is: {}", x);     // xの値は{}です
    x = 6;
    println!("The value of x is: {}", x);

    // shadowing
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of spaces is: {}", spaces);

    // 3.2
    // 型注釈
    let _guess: u32 = "42".parse().expect("Not a number!");    // 数字ではありません！
    // error!
    // let _guess = "42".parse().expect("Not a number!"); 

    // データ型
    // スカラー型
    // 数値
    let _n = b'a';
    let _n = 32_000;
    let _n = 0b1111_0000;
    // 真偽
    let _b = true;
    let _b = false;
    // 実数
    let _f = 2.0; 
    let _f: f32 = 2.0; 
    // 文字
    let _c = 'z';
    let _z = 'ℤ';
    let _heart_eyed_cat = '😻'; 

    // 複合型
    // タプル
    let tup = (500, 6.4, 1);
    let (_x, y, _z) = tup;
    println!("The value of y is: {}", y);
    println!("The value of tup.2 is: {}", tup.2);

    // 配列
    let a = [1, 2, 3, 4, 5];
    let _index = 10;
    // error
    // let element = a[index];
    let element = a[2];
    println!("The value of element is: {}", element);
}
