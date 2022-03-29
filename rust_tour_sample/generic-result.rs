/*

Rustには Result と呼ばれるジェネリックな列挙型が組み込まれており、失敗する可能性のある値を返せます。 
これは言語がエラーを処理する際の慣用的な方法です。

enum Result<T, E> {
    Ok(T),
    Err(E),
}
このジェネリック型はカンマで区切られた複数のパラメータ化された型を持つことに注意してください。

この列挙型はとても一般的なもので、Ok と Err を使えばどこでもインスタンスを生成できます。
*/

fn do_something_that_might_fail(i:i32) -> Result<f32,String> {
  if i == 42 {
      Ok(13.0)
  } else {
      Err(String::from("正しい値ではありません"))
  }
}

fn main() {
  let result = do_something_that_might_fail(12);

  // match は Result をエレガントに分解して、
  // すべてのケースが処理されることを保証できます！
  match result {
      Ok(v) => println!("発見 {}", v),
      Err(e) => println!("Error: {}",e),
  }
}
