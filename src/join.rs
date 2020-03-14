extern crate test;

pub fn join<T: Clone + ToString>(v: Vec<T>, sep: &str) -> String {
  v.into_iter().map(|x| x.to_string()).collect::<Vec<String>>().join(sep)
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;

  #[test]
  fn it_works() {
    assert_eq!(
      // test with 10 items
      join::<i32>([1; 10].to_vec(), ","),
      "1,1,1,1,1,1,1,1,1,1"
    );
  }

  #[bench]
  fn bench_join(b: &mut Bencher) {
    // benchmark with 10 items
    b.iter(|| join::<i32>([1; 10].to_vec(), ","));
  }
}
