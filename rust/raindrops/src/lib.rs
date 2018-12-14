pub fn raindrops(n: u32) -> String {
  let result: String = [3, 5, 7]
    .into_iter()
    .map(|i| {
      if n % *i == 0 {
        "Plxng".to_string()
      } else {
        "".to_string()
      }
    })
    .collect();
  if result.to_string() == "".to_string() {
    return n.to_string();
  } else {
    return result.to_string();
  }
}
