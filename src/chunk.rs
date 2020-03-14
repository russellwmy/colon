extern crate test;

pub fn chunk<T: Clone>(v: Vec<T>, n: usize) -> Vec<Vec<T>> {
  v.chunks(n).map(|x| x.to_vec()).collect::<Vec<Vec<T>>>()
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;

  #[test]
  fn it_works() {
    // test with 10 items
    let chunks = chunk::<i32>((0..10).collect(), 2);

    assert_eq!(chunks.len(), 10 / 2);
  }

  #[bench]
  fn bench_chunk(b: &mut Bencher) {
    // benchmark with 10 items
    b.iter(|| chunk::<i32>((0..10).collect(), 2));
  }
}
