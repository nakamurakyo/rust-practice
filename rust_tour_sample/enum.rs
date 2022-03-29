#![allow(dead_code)] // この行でコンパイラのwaringsメッセージを止めます。

/*
  列挙型はキーワード enum で新しい型を生成することができ、
  この型はいくつかのタグ付された値を持つことができる。

  match は保有する全ての列挙値を処理する手助けすることができる。
  enum は一個もしくは複数な型のデータを持つことができ、C言語の union の様な表現ができます。

  match を用いて列挙値に対するパターンマッチングを行う際、各データを変数名に束縛することができます。

  列挙 のメモリ事情:
    - 列挙型のメモリサイズはそれが持つ最大要素のサイズと等しい。これにより全ての代入可能な値が同じサイズのメモリ空間を利用することを可能にします。
    - 要素の型以外に、各要素には数字値がついており、どのタグであるかについて示しています。

  その他の事情:
    - Rust の 列挙 は tagged-union とも言われています。
    - 複数の型を組み合わせて新しい型を作ることができます。   
      これが Rust には algebraic types を持つと言われる理由です。
*/

enum Species { Crab, Octopus, Fish, Clam }
enum PoisonType { Acidic, Painful, Lethal }
enum Size { Big, Small }
enum Weapon {
    Claw(i32, Size),
    Poison(PoisonType),
    None
}

struct SeaCreature {
    species: Species,
    name: String,
    arms: i32,
    legs: i32,
    weapon: Weapon,
}

fn main() {
    // SeaCreatureのデータはスタックに入ります。
    let ferris = SeaCreature {
        // String構造体もスタックに入りますが、
        // ヒープに入るデータの参照アドレスが一つ入ります。
        species: Species::Crab,
        name: String::from("Ferris"),
        arms: 2,
        legs: 4,
        weapon: Weapon::Claw(2, Size::Small),
    };

    match ferris.species {
        Species::Crab => {
            match ferris.weapon {
                Weapon::Claw(num_claws,size) => {
                    let size_description = match size {
                        Size::Big => "big",
                        Size::Small => "small"
                    };
                    println!("ferris is a crab with {} {} claws", num_claws, size_description)
                },
                _ => println!("ferris is a crab with some other weapon")
            }
        },
        _ => println!("ferris is some other animal"),
    }
}