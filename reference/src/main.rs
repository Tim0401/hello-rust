// 参照と借用
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    // '{}'の長さは、{}です
    println!("The length of '{}' is {}.", s1, len);

    // mutableな参照と借用
    let mut s = String::from("hello");
    change(&mut s);
    println!("s {}", s);

    // mutableな参照と借用は1つしか持てない
    let mut s = String::from("hello");

    let r1 = &mut s;
    //let r2 = &mut s;
    //println!("{}, {}", r1, r2);

    // scopeを用いたmutableな参照と借用
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
    } // r1はここでスコープを抜けるので、問題なく新しい参照を作ることができる

    let r2 = &mut s;

    // immutableとmutableな参照と借用を組み合わせる
    let mut s = String::from("hello");
    let r1 = &s; // 問題なし
    let r2 = &s; // 問題なし

    // let r3 = &mut s; // 大問題！ r1,r2を使う場合にコンパイルエラーになる
    println!("s {} {}", r1, r2);

    // 宙に浮いた参照
    // let reference_to_nothing = dangle();
    let reference_to_nothing = no_dangle();
    println!("reference_to_nothing {}", reference_to_nothing);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
//     // sはスコープから抜けたら消されるはず
// }

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
