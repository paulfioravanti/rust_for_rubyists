fn main() {
  let your_favourite_numbers = vec!(1i, 2i, 3i);
  let my_favourite_numbers = vec!(4i, 5i, 6i);
  let our_favourite_numbers = your_favourite_numbers + my_favourite_numbers;

  println!(
    "The third favourite number is {:d}.", *our_favourite_numbers.get(2)
  );
}
