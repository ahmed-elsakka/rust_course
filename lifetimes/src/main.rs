struct Book<'a>{
    title: &'a str
}

fn main() {
    let x = 5;
    let r = &x;
    println!("r: {}", r);

    let s1 = String::from("hello");
    let s2 = String::from("world!");
    let result = longest(&s1, &s2);
    println!("{}", result);

    let title = String::from("Rust Programming");
    let book = Book { title: &title };

    println!("{}", book.title);
}

fn get_ref() -> i32 {
    5
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn first_word(s: &str) -> & str {
    &s[..1]
}
