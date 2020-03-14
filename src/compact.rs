extern crate test;

pub fn compact<T: Clone>(v: Vec<Option<T>>) -> Vec<T> {
  v.into_iter()
    .filter(|x| x.is_some())
    .map(|x| x.unwrap())
    .collect::<Vec<T>>()
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;

  #[test]
  fn it_works() {
    // test with 10 items
    assert_eq!(compact::<i32>((1..10).map(|_| None).collect()).len(), 0);
  }

  #[bench]
  fn bench_compat(b: &mut Bencher) {
    // benchmark with 10 items
    b.iter(|| compact::<i32>((1..10).map(|_| None).collect()));
  }
}
