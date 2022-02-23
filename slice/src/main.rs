fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    s.clear();
    // sが変更されてもword=5は変更されない
    // println!("{}", s[0..word]);

    let mut s = String::from("hello world");
    let word = first_word_slice(&s); // immutableな借用

    // s.clear(); // mutableな借用が出来ない
    // println!("{}", &word); // 変更後に使用される

    let word = first_word_slice(&s); // immutableな借用
    println!("{} {}", word, word.len());

    let word = first_word_slice_mut(&mut s); // mutableな借用
    println!("{} {}", word, word.len());
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn first_word_slice_mut(s: &mut String) -> &str {
    s.push_str("!");
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
