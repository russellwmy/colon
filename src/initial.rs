extern crate test;

pub fn initial<T: Clone>(mut v: Vec<T>) -> Vec<T> {
  v.remove(v.len() - 1);
  v
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;

  #[test]
  fn it_works() {
    // test with 10 items
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    assert_eq!(initial(v), vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
  }

  #[bench]
  fn bench_initial(b: &mut Bencher) {
    // benchmark with 10 items
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    b.iter(|| initial(v.clone()));
  }
}
