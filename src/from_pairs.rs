extern crate test;

use std::collections::HashMap;

pub fn from_pairs<T: Clone>(v: Vec<(&str, T)>) -> HashMap<&str, T> {
  let m: HashMap<_, _> = v.into_iter().collect();
  m
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;

  #[test]
  fn it_works() {
    assert_eq!(
      // test with 10 items
      from_pairs(vec![
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("ten", 10),
       ]).is_empty(),
      false
    );
  }

  #[bench]
  fn bench_from_pairs(b: &mut Bencher) {
    // benchmark with 10 items
    b.iter(|| from_pairs(vec![
      ("one", 1),
      ("two", 2),
      ("three", 3),
      ("four", 4),
      ("five", 5),
      ("six", 6),
      ("seven", 7),
      ("eight", 8),
      ("nine", 9),
      ("ten", 10),
     ]));
  }
}
