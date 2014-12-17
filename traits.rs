fn print_vec(v: &[int]) {
  for i in v.iter() {
    println!("{:d}", *i);
  }
}

fn print_vec_str(v: &[&str]) {
  for i in v.iter() {
    println!("{:s}", *i);
  }
}

fn main() {
  let vec = [1i, 2i, 3i];
  print_vec(vec);
  let str_vec = ["hey", "there", "yo"];
  print_vec_str(str_vec);
}
