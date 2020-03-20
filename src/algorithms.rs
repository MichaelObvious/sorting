use crate::array::Array;
use rand::Rng;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Algorithm {
    Shuffle,
    Bubble,
    Insertion,
}

impl Algorithm {
    
    pub fn sort(&self, array: Array) -> (bool, Array) {
        match self {
            // ========== SHUFFLE ==========
            Self::Shuffle => match (array.len(), array.i) {
                (n, -1)          => (false, Array::new_ij(array, n as isize - 1, -1)),
                (n, i)  if i > 0 => (false, Array::new_ij(array.swap(i as usize, rand::thread_rng().gen_range(0, n)), i - 1, -1)),
                (_, 0)           => (true, array),
                (_, _)           => (true, array),
            },


            // ========== BUBBLE SORT ==========
            Self::Bubble => match (array.len(), array.i, array.j) {
                (_, -1, -1)                 => (false, Array::new_ij(array, 0, 0)),
                (n, i, j)
                    if i < n as isize && j < n as isize - i - 1
                                            => match (array[j as usize], array[j as usize + 1]) {
                        (a, b) if a > b => (false, Array::new_ij(array.swap(j as usize, j as usize + 1), i, j + 1)),
                        _               => (false, Array::new_ij(array, i, j + 1)),
                    },
                (n, i, _) if i < n as isize => (false, Array::new_ij(array, i + 1, 0)),
                _                           => (true, array),
            }

            // ========== INSERTION SORT ==========
            Self::Insertion => match (array.len(), array.i, array.j) {
                (n, -1, -1)           => (false, Array::new_ij(array, n as isize - 1, 1)),
                (_, i, j)
                    if i > 0 && j > 0 => (false, Array::new_ij(match (array[j as usize], array[j as usize - 1]) {
                        (a, b) if a < b => array.swap(j as usize, j as usize - 1),
                        _               => array,
                    }, i, j - 1 )),
                (n, i, _) if i > 0    => (false, Array::new_ij(array, i - 1, n as isize - (i - 1))),
                _                     => (true, array),
            }
        }
    }

}
