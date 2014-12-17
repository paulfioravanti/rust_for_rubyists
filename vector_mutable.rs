fn main() {
  let mut another_vector = vec!(4i);
  another_vector.push_all([1, 2, 3]);
  println!("The second number is {:d}", *another_vector.get(1));

  let a_vector = vec!(1i, 2i, 3i);
  let mut mut_vector = a_vector;
  *mut_vector.get_mut(0) = 5;

  println!("The first number is {:d}", *mut_vector.get(0));
}
