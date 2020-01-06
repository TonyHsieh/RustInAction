fn main() {
  let three = 0b11;
  let thirty = 0o36;
  let three_hundred = 0x12C;

  println!("DECIMAL: {} {} {}", three, thirty, three_hundred);
  println!("BINARY: {:b} {:b} {:b}", three, thirty, three_hundred);
  println!("OCTAL: {:o} {:o} {:o}", three, thirty, three_hundred);
  println!("HEXADECIMAL: {:x} {:x} {:x}", three, thirty, three_hundred);

}