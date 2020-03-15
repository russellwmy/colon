use crate::first;

pub fn head<T: Clone>(v: Vec<T>) -> Option<T> {
  first::first(v)
}
