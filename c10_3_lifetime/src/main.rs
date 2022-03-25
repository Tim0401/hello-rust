fn main() {
    {
        let r;
        {
            let x = 5;
            r = &x;
            println!("r: {}", r);
        }
        // ここではxのライフタイムがないので参照できない
        // println!("r: {}", r);
    }

    {
        let string1 = String::from("abcd");
        let string2 = "xyz";

        let result = longest(string1.as_str(), string2);
        // 最長の文字列は、{}です
        println!("The longest string is {}", result);
    }

    {
        // 長い文字列は長い
        let string1 = String::from("long string is long");
        // （訳注：この言葉自体に深い意味はない。下の"xyz"より長いということだけが重要）
        let result;
        {
            let string2 = String::from("xyz");
            result = longest(string1.as_str(), string2.as_str());
            // 一番長い文字列は{}
            println!("The longest string is {}", result);
            longest_with_an_announcement(&string1, &string2, "ann");
        }
        // 無効な参照の可能性があるので許可されない
        // println!("The longest string is {}", result);
    }

    {
        // 僕をイシュマエルとお呼び。何年か前・・・
        let novel = String::from("Call me Ishmael. Some years ago...");
        //                                                  "'.'が見つかりませんでした"
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");
        let i = ImportantExcerpt {
            part: first_sentence,
        };
        i.announce_and_return_part("a");
    }
    let s: &'static str = "I have a static lifetime.";
}

// 短い方のライフタイムを用いて返却する
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        //       "お知らせします: {}"
        println!("Attention please: {}", announcement);
        self.part
    }
}

use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    //       "アナウンス！ {}"
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
