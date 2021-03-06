extern crate test;

pub fn zip<T: Clone>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
  let first = v.first();
  match first {
    Some(o) => {
      let l = o.len();
      (0..l)
        .collect::<Vec<usize>>()
        .iter()
        .map(|x| v.iter().map(|y| y[*x].to_owned()).collect())
        .collect()
    }
    None => vec![],
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;

  #[test]
  fn it_works() {
    // test with 10 items
    let v = vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8], vec![9, 10]];
    assert_eq!(zip(v), vec![vec![1, 3, 5, 7, 9], vec![2, 4, 6, 8, 10]]);
  }

  #[bench]
  fn bench_zip(b: &mut Bencher) {
    // benchmark with 10 items
    let v = vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8], vec![9, 10]];

    b.iter(|| zip(v.clone()));
  }
}
