fn main() {
  let mut x = 0;
  let v = loop {
      x += 1;
      if x == 13 {
          break "13 を発見";
      }
  };
  println!("loop の戻り値: {}", v);
}