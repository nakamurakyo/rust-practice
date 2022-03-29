/*
  struct はフィールドの集合です。

  フィールド とはデータ構造とキーワードを紐付ける値です。
  その値はプリミティブ型かデータ構造を指定可能です。

  その定義はメモリ上で隣合うデータの配置をコンパイラに伝える設計図の様なもの

  コードの中で 構造体 を インスタンス化 する際に、プログラムはフィールドデータをメモリ上で隣り合うように作成します。

  全てのフィールドの値を指定してインスタンス化をする際：
    構造体名 {...}.

  構造体のフィールドは演算子 . で取り出すことができます。

  例に示したコードのメモリ状況について：
  - ダブルクオートに囲まれたテキスト(例: "Ferris")は読み取り専用データであるため、 データメモリ に入ります。
  - 関数の呼び出し String::from では構造体 String を作成し、
    この構造体と SeaCreature のフィールドを隣り合う形で スタック に入れられます。 　
    フィールドの値は変更可能であり、メモリ上では以下の様に変更されます。
    1. ヒープ に変更可能なメモリを作り、テキストを入れます。
    2. 1.で作成した参照アドレスを ヒープ に保存し、それを String に保存します(後の章でまた詳しく紹介します。)。

  最後に、我々の友である Ferris と Sarah はプログラムの中では固定な位置であるため、スタック に入ります。
*/

struct SeaCreature {
  animal_type: String,
  name: String,
  arms: i32,
  legs: i32,
  weapon: String,
}

fn main() {
  // SeaCreatureのデータはスタックに入ります。
  let ferris = SeaCreature {
      // String構造体もスタックに入りますが、
      // ヒープに入るデータの参照アドレスが一つ入ります。
      animal_type: String::from("crab"),
      name: String::from("Ferris"),
      arms: 2,
      legs: 4,
      weapon: String::from("claw"),
  };

  let sarah = SeaCreature {
      animal_type: String::from("octopus"),
      name: String::from("Sarah"),
      arms: 8,
      legs: 0,
      weapon: String::from("none"),
  };
  
  println!(
      "{} is a {}. They have {} arms, {} legs, and a {} weapon",
      ferris.name, ferris.animal_type, ferris.arms, ferris.legs, ferris.weapon
  );
  println!(
      "{} is a {}. They have {} arms, and {} legs. They have no weapon..",
      sarah.name, sarah.animal_type, sarah.arms, sarah.legs
  );
}
