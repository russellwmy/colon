extern crate test;

pub fn first<T: Clone>(v: Vec<T>) -> Option<T> {
  v.first().cloned()
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;

  #[test]
  fn it_works() {
    // test with 10 items
    let v: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    assert_eq!(first(v), Some(1));
  }

  #[bench]
  fn bench_first(b: &mut Bencher) {
    // benchmark with 10 items
    let v: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    b.iter(|| first(v.clone()));
  }
}
