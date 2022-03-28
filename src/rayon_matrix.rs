use std::ops::{Index, IndexMut, Mul};
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use rayon::prelude::ParallelSliceMut;
use std::fmt::{Debug, Formatter};

pub struct RayonMatrix {
    data: Vec<f32>,
    n: usize,
    m: usize
}

impl Debug for RayonMatrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let sz = if self.data.len() < 8 {self.data.len()} else {8};

        write!(f, "RayonMatrix {{ {:?} n={} m={} }}", &self.data[..sz], self.n, self.m)
    }
}

impl Index<usize> for RayonMatrix {
    type Output = [f32];

    fn index(&self, index: usize) -> &Self::Output {
        debug_assert!(index < self.n, "Invalid argument: Out of range!");

        unsafe { self.data.get_unchecked(index * self.m..(index + 1) * self.m) }
    }
}

impl IndexMut<usize> for RayonMatrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        debug_assert!(index < self.n, "Invalid argument: Out of range!");

        unsafe { self.data.get_unchecked_mut(index * self.m..(index + 1) * self.m) }
    }
}

impl Mul for &RayonMatrix {
    type Output = RayonMatrix;

    fn mul(self, rhs: Self) -> Self::Output {
        debug_assert_eq!(self.m, rhs.n, "Invalid arguments!");
        let rhs_t = rhs.transposed();

        let mut result: Vec<(usize, f32)> =
            (0..self.n * rhs.m)
                .into_par_iter()
                .map(move |pos| {
                    let i = pos / rhs.m; // (i,j) in the result matrix
                    let k = pos % rhs.m;

                    let column = &rhs_t[k];
                    let line = &self[i];

                    let mut result = 0f32;
                    for j in 0..self.m {
                        result += line[j] * column[j];
                    }

                    (pos, result)
                })
                .collect::<Vec<(usize, f32)>>();

        result.par_sort_by(|l, r| l.0.cmp(&r.0));

        let result =
            result.iter()
                .map(|(_, value)| *value)
                .collect();

        RayonMatrix::from(self.n, rhs.m, result)
    }
}

impl RayonMatrix {
    pub fn new(n: usize,
               m: usize) -> RayonMatrix {
        let mut res = Vec::<f32>::new();
        res.resize(n * m, 0.0);
        RayonMatrix {
            data: res, n, m
        }
    }

    pub fn from(n: usize, m: usize, data: Vec<f32>) -> RayonMatrix {
        debug_assert_eq!(data.len(), n * m, "Invalid argument!");

        RayonMatrix {
            data, n, m
        }
    }

    pub fn transposed(&self) -> RayonMatrix {
        let mut result = RayonMatrix::new(self.m, self.n);
        for i in 0..self.n {
            for j in 0..self.m {
                result[j][i] = self[i][j]
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matrix_test() {
        let a = RayonMatrix::from(2, 2, vec![1., 2., 3., 4.]);
        let b = RayonMatrix::from(2, 2, vec![5., 6., 7., 8.]);
        let c = &a * &b;

        assert_eq!(c.data, [19., 22., 43., 50.]);
    }
}
