fn rebind() {
  let sum = 0;
  for i in 0..10 {
    let sum = sum + 1;
  }
  println!("{}", sum);
}

fn ressign() {
  let mut sum = 0;
  for i in 0..10 {
    sum = sum + i;
  } 
  println!("{}", sum);
}

fn main() {
  rebind();
  ressign();
}
