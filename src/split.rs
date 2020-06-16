pub fn split<'a>(input: &'a str) -> Vec<&'a str> {
  let v: Vec<&'a str> = input.split('.').collect();

  return v;
}
