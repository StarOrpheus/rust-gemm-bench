use std::ops::{Index, IndexMut, Mul};
use std::fmt::{Debug, Formatter};

pub struct SimpleMatrix {
    data: Vec<f32>,
    n: usize,
    m: usize
}

impl Index<usize> for SimpleMatrix {
    type Output = [f32];

    fn index(&self, index: usize) -> &Self::Output {
        debug_assert!(index < self.n, "Invalid argument: Out of range!");

        unsafe { self.data.get_unchecked(index * self.m..(index + 1) * self.m) }
    }
}

impl IndexMut<usize> for SimpleMatrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        debug_assert!(index < self.n, "Invalid argument: Out of range!");

        unsafe { self.data.get_unchecked_mut(index * self.m..(index + 1) * self.m) }
    }
}

impl Mul for &SimpleMatrix {
    type Output = SimpleMatrix;

    fn mul(self, rhs: Self) -> Self::Output {
        debug_assert_eq!(self.m, rhs.n, "Invalid arguments!");

        let mut result = SimpleMatrix::new(self.n, rhs.m);

        for i in 0..self.n {
            for k in 0..rhs.m {
                for j in 0..self.m {
                    result[i][k] += self[i][j] * rhs[j][k]
                }
            }
        }

        result
    }
}

impl SimpleMatrix {
    pub fn new(n: usize,
               m: usize) -> SimpleMatrix {
        let mut res = Vec::<f32>::new();
        res.resize(n * m, 0.0);
        SimpleMatrix {
            data: res, n, m
        }
    }

    pub fn from(n: usize, m: usize, data: Vec<f32>) -> SimpleMatrix {
        debug_assert_eq!(data.len(), n * m, "Invalid argument!");

        SimpleMatrix {
            data, n, m
        }
    }
}

impl Debug for SimpleMatrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let sz = if self.data.len() < 8 {self.data.len()} else {8};

        write!(f, "SimpleMatrix {{ {:?} n={} m={} }}", &self.data[..sz], self.n, self.m)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matrix_test() {
        let a = SimpleMatrix::from(2, 2, vec![1., 2., 3., 4.]);
        let b = SimpleMatrix::from(2, 2, vec![5., 6., 7., 8.]);
        let c = &a * &b;

        assert_eq!(c.data, [19., 22., 43., 50.]);
    }
}
