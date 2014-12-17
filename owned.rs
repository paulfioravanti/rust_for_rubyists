fn plus_one_box_pointer(ptr: Box<int>) -> int {
  *ptr + 1
}

fn plus_one_pointer(ptr: &int) -> int {
  *ptr + 1
}

fn main() {
  let x = box 10i;
  /* let y = x.clone(); */
  let y = 10i;
  let z = y;
  println!("{:d}", *x);
  println!("{:d}", plus_one_box_pointer(x));
  println!("{:d}", plus_one_pointer(&z));
}
