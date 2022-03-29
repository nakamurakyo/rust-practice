// add は i32 型（32 ビット長の整数）の引数を 2つ取る
fn add(x: i32, y: i32) -> i32 {
  return x + y;
}

// 関数は、値をタプルで返すことによって、複数の値を返せます。
fn swap(x: i32, y: i32) -> (i32, i32) {
  return (y, x);
}

// 関数に戻り値の型が指定されていない場合、
// unit と呼ばれる空のタプルを返します。
// 空のタプルは () と表記します。
fn make_nothing() -> () {
  return ();
}

// 戻り値は () と推論
fn make_nothing2() {
  // この関数は戻り値が指定されないため () を返す
}

fn main() {
  println!("{}", add(42, 13));

  // 戻り値をタプルで返す
  let result = swap(123, 321);
  println!("{} {}", result.0, result.1);
  
  // タプルを2つの変数に分解
  let (a, b) = swap(result.0, result.1);
  println!("{} {}", a, b);
}
