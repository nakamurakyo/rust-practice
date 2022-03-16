fn main() {
    println!("Hello, world!");

    // x の型を推論
    let x = 13;
    println!("{}", x);

    // x の型を指定 ※ 勿論シャドウイングも可能
    let x: f64 = 3.14159;
    println!("{}", x);

    // mut というキーワードで可変変数に。
    let mut y = 42;
    println!("{}", y);
    y = 13;
    println!("{}", y);
}
