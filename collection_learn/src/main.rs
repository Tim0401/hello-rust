fn main() {
    // 初期化方法
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    {
        let v = vec![1, 2, 3, 4];
        // vで作業をする
    } // <- vはここでスコープを抜け、解放される

    // アクセス方法
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        //                      "3つ目の要素は{}です"
        Some(third) => println!("The third element is {}", third),
        //               "3つ目の要素はありません。"
        None => println!("There is no third element."),
    }

    // error
    // let does_not_exist = &v[100];
    // ok
    let does_not_exist = v.get(100);

    // 参照エラー
    let mut v = vec![1, 2, 3, 4, 5];
    /*
    let first = &v[0];
    v.push(6);
    println!("The first element is: {}", first);
    */

    // ループ
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
}
