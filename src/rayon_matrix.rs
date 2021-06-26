use std::ops::{Index, IndexMut, Mul};
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use rayon::prelude::ParallelSliceMut;

#[derive(Debug)]
pub struct RayonMatrix {
    data: Vec<f32>,
    n: usize,
    m: usize
}

impl Index<usize> for RayonMatrix {
    type Output = [f32];

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.n, "Invalid argument: Out of range!");

        &self.data[index * self.m..(index + 1) * self.m]
    }
}

impl IndexMut<usize> for RayonMatrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < self.n, "Invalid argument: Out of range!");

        &mut self.data[index * self.m..(index + 1) * self.m]
    }
}

impl Mul for &RayonMatrix {
    type Output = RayonMatrix;

    fn mul(self, rhs: Self) -> Self::Output {
        assert_eq!(self.m, rhs.n, "Invalid arguments!");

        let mut result: Vec<(usize, f32)> =
            (0..self.n * rhs.m)
                .into_par_iter()
                .map(move |pos| {
                    let i = pos / rhs.m; // (i,j) in the result matrix
                    let k = pos % rhs.m;

                    let mut result = 0f32;
                    for j in 0..self.m {
                        result += self[i][j] * rhs[j][k];
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
        assert_eq!(data.len(), n * m, "Invalid argument!");

        RayonMatrix {
            data, n, m
        }
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
