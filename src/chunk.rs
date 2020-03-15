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
    let v: Vec<i32> = vec![1, 2, 3, 4, 56, 7, 8, 9, 10];
    let chunks = chunk(v, 2);

    assert_eq!(chunks.len(), 10 / 2);
  }

  #[bench]
  fn bench_chunk(b: &mut Bencher) {
    // benchmark with 10 items
    let v: Vec<i32> = vec![1, 2, 3, 4, 56, 7, 8, 9, 10];

    b.iter(|| chunk(v.clone(), 2));
  }
}
