use std::fs::File;

fn main() {
    // panic!("crash and burn"); //クラッシュして炎上

    // match
    // let f = File::open("hello.txt");

    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => {
    //         // ファイルを開く際に問題がありました
    //         // panic!("There was a problem opening the file: {:?}", error)
    //         println!("There was a problem opening the file: {:?}", error)
    //     }
    // };

    // unwrap
    // let f = File::open("hello.txt").unwrap();

    // expect
    // hello.txtを開くのに失敗しました
    // let f = File::open("hello.txt").expect("Failed to open hello.txt");

    let err = read_username_from_file();
    println!("{:?}", err);
    let err = read_username_from_file_q();
    println!("{:?}", err);
    let err = read_username_from_file_q2();
    println!("{:?}", err);
}

use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_q() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_q2() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
