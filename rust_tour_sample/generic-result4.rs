/*

Option/Result を使って作業するのは、ちょっとしたコードを書くのには厄介です。 
Option と Result の両方には unwrap と呼ばれる関数があり、
手っ取り早く値を取得するのには便利です。 unwrap は以下のことを行います。

Option/Result 内の値を取得します。
列挙型が None/Err の場合、panic! します。
以下の2つのコードは等価です。

my_option.unwrap()

match my_option {
    Some(v) => v,
    None => panic!("Rust によって生成されたエラーメッセージ！"),
}

同様に:

my_result.unwrap()
match my_result {
    Ok(v) => v,
    Err(e) => panic!("Rust によって生成されたエラーメッセージ！"),
}

良い Rust 使い（Rustacean）であるためには、可能な限り適切に match を使用してください。
*/

fn do_something_that_might_fail(i: i32) -> Result<f32, String> {
  if i == 42 {
      Ok(13.0)
  } else {
      Err(String::from("正しい値ではありません"))
  }
}

fn main() -> Result<(), String> {
  // 簡潔ですが、値が存在することを仮定しており、
  // すぐにダメになる可能性があります。
  let v = do_something_that_might_fail(42).unwrap();
  println!("発見 {}", v);
  
  // パニックするでしょう！
  let v = do_something_that_might_fail(1).unwrap();
  println!("発見 {}", v);
  
  Ok(())
}