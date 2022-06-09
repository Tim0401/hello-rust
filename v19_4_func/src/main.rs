fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: &dyn Fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(&add_one, 5);
    // 答えは{}
    println!("The answer is: {}", answer);

    let answer = do_twice(&returns_closure(), 5);
    // 答えは{}
    println!("The answer is: {}", answer);
}

fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
