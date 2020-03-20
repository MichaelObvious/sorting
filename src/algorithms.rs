use crate::array::Array;
use rand::Rng;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Algorithm {
    Shuffle,
}

impl Algorithm {
    
    pub fn sort(&self, array: Array) -> (bool, Array) {
        match self {
            Self::Shuffle => match (array.len(), array.i) {
                (n, -1)          => (false, Array::new_ij(array, n as isize - 1, -1)),
                (n, i)  if i > 0 => (false, Array::new_ij(array.swap(i as usize, rand::thread_rng().gen_range(0, n)), i - 1, -1)),
                (_, 0)           => (true, array),
                (_, _)           => (true, array),
            }
        }
    } 

}
