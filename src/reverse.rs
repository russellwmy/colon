extern crate test;

pub fn reverse<T: Clone>(v: &mut Vec<T>) {
  v.reverse();
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;

  #[test]
  fn it_works() {
    // test with 10 items
    let mut v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    reverse(&mut v);
    
    assert_eq!(
      v,
      vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1]
    );
  }

  #[bench]
  fn bench_first(b: &mut Bencher) {
    // benchmark with 10 items
    let mut v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    b.iter(|| reverse(&mut v));
  }
}
