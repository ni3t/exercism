pub fn raindrops(n: u32) -> String {
  let mut string = String::from("");

  let mut any = false;
  if n % 3 == 0 {
    string.push_str("Pling");
    any = true;
  }
  if n % 5 == 0 {
    string.push_str("Plang");
    any = true;
  }
  if n % 7 == 0 {
    string.push_str("Plong");
    any = true;
  }

  if any {
    return string;
  } else {
    return n.to_string();
  }
}
