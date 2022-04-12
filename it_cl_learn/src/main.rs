use it_cl_learn::generate_workout;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);

    let x = vec![1, 2, 3];

    // move
    // let equal_to_x = move |z| z == x;
    // borrow
    let equal_to_x = |z| z == x;

    // ここでは、xを使用できません: {:?}
    println!("can't use x here: {:?}", x);

    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));
}
