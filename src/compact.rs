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
    let v: Vec<Option<i32>> = (1..10).map(|_| None).collect();

    assert_eq!(compact(v).len(), 0);
  }

  #[bench]
  fn bench_compact(b: &mut Bencher) {
    // benchmark with 10 items
    let v: Vec<Option<i32>> = (1..10).map(|_| None).collect();

    b.iter(|| compact(v.clone()));
  }
}
