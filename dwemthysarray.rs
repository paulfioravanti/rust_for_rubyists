struct Monster {
  health: int,
  attack: int
}

impl Monster {
  fn attack(&self) {
    println!("The monster attacks for {:d} damage", self.attack);
  }

  fn count() {
    println!("There are a bunch of monsters out tonight.");
  }
}

fn main() {
  let m = Monster { health: 10, attack: 20 };
  /* println!("{:s}", m.health.to_string()); */
  /* println!("{:s}", m.attack.to_string()); */
  m.attack();
  Monster::count(); 
}
