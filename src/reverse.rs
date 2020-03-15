extern crate test;

pub fn reverse<T: Clone>(v: Vec<T>) -> Vec<T> {
  let mut r = v.clone();

  r.reverse();
  r
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;

  #[test]
  fn it_works() {
    // test with 10 items
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    assert_eq!(reverse(v), vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1]);
  }

  #[bench]
  fn bench_first(b: &mut Bencher) {
    // benchmark with 10 items
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    b.iter(|| reverse(v.clone()));
  }
}
