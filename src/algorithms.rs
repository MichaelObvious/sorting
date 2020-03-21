use crate::array::Array;
use rand::Rng;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Algorithm {
    Shuffle,
    Bubble,
    Insertion,
    Selection,
    OddEven,
}

impl Algorithm {
    
    pub fn sort(&self, array: Array) -> (bool, Array) {
        match self {
            // ========== SHUFFLE ==========
            Self::Shuffle => match (array.len(), array.i) {
                (n, -1)          => (false, array.set_ij(n as isize - 1, -1)),
                (n, i)  if i > 0 => (false, array.swap(i as usize, rand::thread_rng().gen_range(0, n)).set_ij(i - 1, -1)),
                (_, 0)           => (true, array),
                (_, _)           => (true, array),
            },


            // ========== BUBBLE SORT ==========
            Self::Bubble => match (array.len(), array.i, array.j) {
                (_, -1, -1)                 => (false, array.set_ij(0, 0)),
                (n, i, j)
                    if i < n as isize && j < n as isize - i - 1
                                            => match (array[j as usize], array[j as usize + 1]) {
                        (a, b) if a > b => (false, array.swap(j as usize, j as usize + 1).set_ij(i, j + 1)),
                        _               => (false, array.set_ij(i, j + 1)),
                    },
                (n, i, _) if i < n as isize => (false, array.set_ij(i + 1, 0)),
                _                           => (true, array),
            },

            // ========== INSERTION SORT ==========
            Self::Insertion => match (array.len(), array.i, array.j) {
                (n, -1, -1)           => (false, array.set_ij(n as isize - 1, 1)),
                (_, i, j)
                    if i > 0 && j > 0 => (false, match (array[j as usize], array[j as usize - 1]) {
                        (a, b) if a < b => array.swap(j as usize, j as usize - 1),
                        _               => array.check(j as usize),
                    }.set_ij(i, j - 1 )),
                (n, i, _) if i > 0    => (false, array.set_ij(i - 1, n as isize - (i - 1))),
                _                     => (true, array),
            },

            // ========== SELECTION SORT ==========
            Self::Selection => match (array.len(), array.i, array.j) {
                (_, -1, -1) => (false, array.set_ij(0, 1).set_lowest(0)),
                (n, i, j)
                    if i < n as isize - 1 && j < n as isize
                            => (false,  match (array[j as usize], array[array.lowest as usize]) {
                                (a, b) if a < b => array.set_ij(i, j + 1).set_lowest(j).check(j as usize),
                                _               => array.set_ij(i, j + 1).check(j as usize),
                            }),
                (n, i, _)
                    if i < n as isize - 1
                           => (false, match array.lowest {
                                x if x == i => array,
                                l           => array.swap(i as usize, l as usize),
                           }.set_ij(i + 1, i + 2).set_lowest(i + 1)),
                _          => (true, array),
            },
            
            // ========== ODD-EVEN SORT ==========
            Self::OddEven => match (array.len(), array.i, array.j, array.sorted) {
                (_, -1, -1, _)   => (false, array.set_ij(0, 1)),
                (n, i, j, _)
                    if i < n as isize - 1
                                 => (false, match (array[i as usize], array[i as usize + 1]) {
                                        (a, b) if a > b => array.swap(i as usize, i as usize + 1).set_ij(i + 2, j).set_sorted(false),
                                        _               => array.check(i as usize).set_ij(i + 2, j),
                                    }),
                (n, i, j, _)
                    if j < n as isize - 1
                                 => (false, match (array[j as usize], array[j as usize + 1]) {
                                        (a, b) if a > b => array.swap(j as usize, j as usize + 1).set_ij(i, j + 2).set_sorted(false),
                                        _               => array.check(j as usize).set_ij(i, j + 2),
                                    }),
                (_, _, _, false) => (false, array.set_ij(0, 1).set_sorted(true)),
                _                => (true, array),

            },
        }
    }

}
