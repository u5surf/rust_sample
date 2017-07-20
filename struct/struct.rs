//Unit structure
struct Dummy;

//tuple structure
struct Point(f64, f64);

//structure
struct Color {
  r: u8,
  g: u8,
  b: u8,
}

fn main() {
  let dummy = Dummy;

  let point = Point(0.0,0.0);

  let x = point.0;

  let black = Color { r: 0, g: 0, b: 0};

  let r = black.r;
  println!("{}", x);
  println!("{}", r);

}
