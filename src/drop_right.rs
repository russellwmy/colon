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
    // test with 10 items
    let v: Vec<i32> = [1; 10].to_vec();
    
    assert_eq!(drop_right(v, 2).len(), 8);
  }

  #[bench]
  fn bench_drop_right(b: &mut Bencher) {
    // benchmark with 10 items
    let v: Vec<i32> = [1; 10].to_vec();

    b.iter(|| drop_right(v, 2));
  }
}
