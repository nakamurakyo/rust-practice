fn main() {
  /*
    関数(function)と違って、メソッド(method)は特定のデータ型と紐づく関数のことです。
    スタティックメソッド - ある型そのものに紐付き、演算子 :: で呼び出せます。
    インスタンスメソッド - ある型のインスタンスに紐付き、演算子 . で呼び出せます。
  */

  // スタティックメソッドでStringインスタンスを作成する。
  let s = String::from("Hello world!");
  // インスタンスを使ってメソッド呼び出す。
  println!("{} is {} characters long.", s, s.len());
}