fn main() {
  let a = 13u8;
  let b = 7u32;

  // u8 と u32 を混ぜるとエラーになります。
  let c = a as u32 + b;
  println!("{}", c);

  // Rust は as キーワードで明示的に型変換できる。
  let t = true;
  println!("{}", t as u8);
}
