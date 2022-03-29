/*
  他の言語では、値が存在しないことを表すためにキーワード null が用いられます。 
  これは変数やフィールドの操作に失敗する可能性があることを意味するため、
  プログラミング言語に困難をもたらします。

  Rust には null はありませんが、値がないことを表現することの重要性を無視しているわけではありません。 
  既に知っているツールを使った素朴な表現を考えてみましょう。

  1つ以上の値を None によって代替するパターンは、null がない Rust ではとても一般的です。
  ジェネリック型はこの問題を解決するのに役立ちます。

  Rustには Option と呼ばれるジェネリックな列挙型が組み込まれており、
  null を使わずに null 許容な値を表現できます。

  enum Option<T> {
      None,
      Some(T),
  }
  この列挙型はとても一般的なもので、Some と None を使えばどこでもインスタンスを生成できます。
*/

// 部分的に定義された構造体型
struct BagOfHolding<T> {
  // パラメータ T を渡すことが可能
  item: Option<T>,
}

fn main() {
  // 注意: i32 が入るバッグに、何も入っていません！
  // None からは型が決められないため、型を指定する必要があります。
  let i32_bag = BagOfHolding::<i32> { item: None };

  if i32_bag.item.is_none() {
      println!("バッグには何もない！")
  } else {
      println!("バッグには何かある！")
  }

  let i32_bag = BagOfHolding::<i32> { item: Some(42) };

  if i32_bag.item.is_some() {
      println!("バッグには何かある！")
  } else {
      println!("バッグには何もない！")
  }

  // match は Option をエレガントに分解して、
  // すべてのケースが処理されることを保証できます！
  match i32_bag.item {
      Some(v) => println!("バッグに {} を発見！", v),
      None => println!("何も見付からなかった"),
  }
}

