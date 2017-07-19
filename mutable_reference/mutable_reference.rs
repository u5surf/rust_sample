fn refmut_string(s: &mut String){
  println!("{}", s);
}

fn main() {
  let mut s = "this is a resourece".to_string();
	let t = &mut s;
	//refmut_string(&mut s);
}
