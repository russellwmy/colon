extern crate test;

pub fn join<T: Clone + ToString>(v: Vec<T>, sep: &str) -> String {
  v.iter()
    .map(|x| x.to_string())
    .collect::<Vec<String>>()
    .join(sep)
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;

  #[test]
  fn it_works() {
    // test with 10 items
    let v = [1; 10].to_vec();

    assert_eq!(join(v, ","), "1,1,1,1,1,1,1,1,1,1");
  }

  #[bench]
  fn bench_join(b: &mut Bencher) {
    // benchmark with 10 items
    let v = [1; 10].to_vec();

    b.iter(|| join(v.clone(), ","));
  }
}
