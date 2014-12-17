fn div_by_three(num: int) -> bool {
  num % 3 == 0
}

fn div_by_five(num: int) -> bool {
  num % 5 == 0
}

fn main() {
  for num in range(1i, 101) {
    let answer =
      if div_by_three(num) && div_by_five(num) {
        "FizzBuzz".to_string()
      } else if div_by_three(num) {
        "Fizz".to_string()
      } else if div_by_five(num) {
        "Buzz".to_string()
      } else {
        num.to_string()
      };

    println!("{}", answer);
  }
}

#[test]
fn test_div_by_three() {
  if div_by_three(1) {
    fail!("One is not divisible by three");
  }
}

#[test]
fn test_div_by_three_with_three() {
  if !div_by_three(3) {
    fail!("Three should be divisible by three");
  }
}

#[test]
fn test_div_by_five() {
  if div_by_five(1) {
    fail!("One is not divisible by five");
  }
}

#[test]
fn test_div_by_five_with_five() {
  if !div_by_five(5) {
    fail!("Five should be divisible by five");
  }
}

#[test]
fn test_div_by_fifteen() {
  if div_by_three(1) && div_by_five(1) {
    fail!("One is not divisible by three or five");
  }
}

#[test]
fn test_div_by_fifteen_with_fifteen() {
  if !div_by_three(15) && !div_by_five(15) {
    fail!("Fifteen should be divisible by three and five");
  }
}

