fn print_string(s: String) {
  println!("{}",s);
}

fn main() {
  let s = "this is a resource".to_string();
	let t = s;
	print_string(t);
  //print t once more but that has moved ownership
	//print_string(t);
	//print s but occure the error that has not ownership
	//print_string(s);
	
}
