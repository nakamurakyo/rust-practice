/*

ベクタ型
最も有用なジェネリック型のいくつかはコレクション型です。 
ベクタは構造体 Vec で表される可変サイズのリストです。

マクロ vec! を使えば、手動で構築するよりも簡単にベクタを生成できます。

Vec にはメソッド iter() があります。 これによってベクタからイテレータを生成すれば、
ベクタを簡単に for ループに入れることができます。

メモリに関する詳細：

- Vec は構造体ですが、内部的にはヒープ上の固定リストへの参照を含んでいます。

- ベクタはデフォルトの容量で始まります。 容量よりも多くの項目が追加された場合、
  ヒープ上により大きな容量の固定リストを生成して、データを再割り当てします。
*/

fn main() {
  // 型を明示的に指定
  let mut i32_vec = Vec::<i32>::new(); // turbofish <3
  i32_vec.push(1);
  i32_vec.push(2);
  i32_vec.push(3);

  // もっと賢く、型を自動的に推論
  let mut float_vec = Vec::new();
  float_vec.push(1.3);
  float_vec.push(2.3);
  float_vec.push(3.4);

  // きれいなマクロ！
  let string_vec = vec![String::from("Hello"), String::from("World")];

  for word in string_vec.iter() {
      println!("{}", word);
  }
}
