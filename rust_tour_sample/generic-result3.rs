/*

Result はとてもよく使うので、Rust にはそれを扱うための強力な演算子 ? が用意されています。 以下の2つのコードは等価です。

do_something_that_might_fail()?
match do_something_that_might_fail() {
    Ok(v) => v,
    Err(e) => return Err(e),
}

*/

fn do_something_that_might_fail(i: i32) -> Result<f32, String> {
  if i == 42 {
      Ok(13.0)
  } else {
      Err(String::from("正しい値ではありません"))
  }
}

fn main() -> Result<(), String> {
  // コードが簡潔なのに注目！
  let v = do_something_that_might_fail(42)?;
  println!("発見 {}", v);
  Ok(())
}