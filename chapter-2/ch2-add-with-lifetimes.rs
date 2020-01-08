fn add_with_liftimes<'a, 'b>(i: &'a i32, j: &'b i32) -> i32 {
  *i + *j
}

fn main() {
  let res = add_with_liftimes(&10, &20);
  println!("{}", res);
}