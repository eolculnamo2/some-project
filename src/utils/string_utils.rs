use std::convert::TryInto;

pub fn find_substr_index(string: &String, substr: &String) -> isize {
  if substr.len() == 0 {
    return -1;
  }

  let substr_chars = substr.chars();
  let string_chars = string.chars();
  for (i,c)  in string_chars.enumerate() {
    if c == substr_chars.nth(0).unwrap() {
      for (y, x) in substr_chars.enumerate() {
        let matches = string_chars.nth(i + y).unwrap() == x;
        if matches == false {
          continue;
        }
      }
      return i.try_into().unwrap();
    }
  }
  -1
}