extern crate test;

pub fn tail<T: Clone>(v: Vec<T>) -> Vec<T> {
  v[1..].into_iter().cloned().collect()
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;

  #[test]
  fn it_works() {
    // test with 10 items
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    assert_eq!(tail(v), vec![2, 3, 4, 5, 6, 7, 8, 9, 10]);
  }

  #[bench]
  fn bench_tail(b: &mut Bencher) {
    // benchmark with 10 items
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    b.iter(|| tail(v.clone()));
  }
}
