fn main() {
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str()関数は、リテラルをStringに付け加える

    println!("{}", s); // これは`hello, world!`と出力する

    // move
    let s1 = String::from("hello");
    let s2 = s1;

    // s1は使えない
    //println!("{}, world!", s1);
    println!("{}, world!", s2);

    let s1 = String::from("hello");
    let s2 = s1.clone();

    // cloneでdeepCopyすると使える
    println!("s1 {}, world!", s1);
    println!("s2 {}, world!", s2);

    // 所有権と関数
    let s = String::from("hello");  // sがスコープに入る
    takes_ownership(s);             // sの値が関数にムーブされ...
                                    // ... ここではもう有効ではない
    // println!("s {}, world!", s);

    let x = 5;                      // xがスコープに入る
    makes_copy(x);    
}


fn takes_ownership(some_string: String) { // some_stringがスコープに入る。
    println!("{}", some_string);
} // ここでsome_stringがスコープを抜け、`drop`が呼ばれる。後ろ盾してたメモリが解放される。
  // 

fn makes_copy(some_integer: i32) { // some_integerがスコープに入る
    println!("{}", some_integer);
} // ここでsome_integerがスコープを抜ける。何も特別なことはない。