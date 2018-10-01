#[allow(unused_variables)]

fn main() {
  let mut s = String::from("Hello");
  let len = calc_len(&s);

  println!("string = {}, len = {}", s, len);
  
  let r = &mut s;
  change_string(r);
  println!("new string = {}", r);
}


fn calc_len(s: &String) ->  usize{
  s.len()
}

fn change_string(s: &mut String) {
  s.push_str(", world!");
}