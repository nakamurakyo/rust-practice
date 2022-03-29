fn main() {
  // 変数は let キーワードを使用して宣言します。
  // Rust は 99% のケースで変数の型を推論できます。

  // それができない場合、変数宣言に型を追加できます。
  // 参考: https://teratail.com/questions/266017#reply-380929

  // x の型を推論
  let x = 13;
  println!("{}", x);

  // x の型を指定 ※ 勿論シャドウイングも可能
  let x: f64 = 3.14159;
  println!("{}", x);

  // 可変値は mut キーワードで表します。
  let mut y = 42;
  println!("{}", y);
  y = 13;
  println!("{}", xy;
}
