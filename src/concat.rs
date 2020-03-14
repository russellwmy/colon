extern crate test;

pub fn concat<T: Clone>(mut v1: Vec<T>, v2: Vec<T>) -> Vec<T> {
  v1.extend(v2);
  v1
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;

  #[test]
  fn it_works() {
    // test with 2 5-item vectors
    assert_eq!(concat::<i32>([1; 5].to_vec(), [1; 5].to_vec()).len(), 10);
  }

  #[bench]
  fn bench_concat(b: &mut Bencher) {
     // benchmark with 2 5-item vectors
    b.iter(|| concat::<i32>([1; 5].to_vec(), [1; 5].to_vec()));
  }
}
