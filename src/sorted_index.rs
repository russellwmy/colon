extern crate test;

use std::cmp::PartialEq;
use std::cmp::PartialOrd;

pub fn sorted_index<T: Clone + PartialEq + PartialOrd>(v: Vec<T>, val: T) -> usize {
  let loc = v.binary_search_by(|x| x.partial_cmp(&val).expect("Couldn't compare values"));

  match loc {
    Ok(i) => {
      let j = *loc.iter().next().unwrap();
      match j {
        x if x > 0 => j - 1,
        _ => 0,
      }
    }
    Err(i) => i,
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;

  #[test]
  fn it_works() {
    // test with 10 items
    let v = vec![1, 2, 3, 4, 4, 5, 6, 7, 8, 9, 10];

    assert_eq!(sorted_index(v, 4), 3);
  }

  #[bench]
  fn bench_sorted_index(b: &mut Bencher) {
    // benchmark with 10 items
    let v = vec![1, 2, 3, 4, 4, 5, 6, 7, 8, 9, 10];

    b.iter(|| sorted_index(v.clone(), 4));
  }
}
