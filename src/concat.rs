extern crate test;

pub fn concat<T: Clone>(v1: Vec<T>, v2: Vec<T>) -> Vec<T> {
  let mut v = v1.clone();

  v.extend(v2);
  v
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;

  #[test]
  fn it_works() {
    // test with 2 5-item vectors
    let v1: Vec<i32> = [1; 5].to_vec();
    let v2: Vec<i32> = [1; 5].to_vec();

    assert_eq!(concat(v1, v2).len(), 10);
  }

  #[bench]
  fn bench_concat(b: &mut Bencher) {
    // benchmark with 2 5-item vectors
    let v1: Vec<i32> = [1; 5].to_vec();
    let v2: Vec<i32> = [1; 5].to_vec();

    b.iter(|| concat(v1.clone(), v2.clone()));
  }
}
