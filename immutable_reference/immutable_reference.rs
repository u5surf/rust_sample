fn ref_string(s: &String) {
  println!("{}", s);
}

fn main() {
  let s = "this is a resource".to_string();
	let t = &s;
	ref_string(&s);
}
