extern crate test;

pub fn drop_right<T: Clone>(v: Vec<T>, n: usize) -> Vec<T> {
  let (v_remain, _) = v.split_at(n);
  v_remain.to_vec()
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;

  #[test]
  fn it_works() {
    assert_eq!(
      // test with 10 items
      drop_right::<i32>([1; 10].to_vec(), 2).len(),
      8
    );
  }

  #[bench]
  fn bench_drop_right(b: &mut Bencher) {
    // benchmark with 10 items
    b.iter(|| drop_right::<i32>([1; 10].to_vec(), 2));
  }
}
