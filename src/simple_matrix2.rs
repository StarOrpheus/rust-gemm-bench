use std::ops::{Index, IndexMut, Mul};
use std::fmt::{Debug, Formatter};

#[derive(Clone, PartialEq)]
pub struct SimpleMatrix2 {
    data: Vec<f32>,
    n: usize,
    m: usize
}

impl Index<usize> for SimpleMatrix2 {
    type Output = [f32];

    fn index(&self, index: usize) -> &Self::Output {
        debug_assert!(index < self.n, "Invalid argument: Out of range!");

        unsafe { self.data.get_unchecked(index * self.m..(index + 1) * self.m) }
    }
}

impl IndexMut<usize> for SimpleMatrix2 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        debug_assert!(index < self.n, "Invalid argument: Out of range!");

        unsafe { self.data.get_unchecked_mut(index * self.m..(index + 1) * self.m) }
    }
}

impl Mul for &SimpleMatrix2 {
    type Output = SimpleMatrix2;

    fn mul(self, rhs: Self) -> Self::Output {
        debug_assert_eq!(self.m, rhs.n, "Invalid arguments!");

        let mut result = SimpleMatrix2::new(self.n, rhs.m);

        let trans = rhs.transposed();

        for i in 0..self.n {
            for k in 0..rhs.m {
                let mut r = 0f32;
                for j in 0..self.m {
                    r += trans[i][j] * trans[k][j]
                }
                result[i][k] = r;
            }
        }

        result
    }
}

impl SimpleMatrix2 {
    pub fn new(n: usize,
               m: usize) -> SimpleMatrix2 {
        let mut res = Vec::<f32>::new();
        res.resize(n * m, 0.0);
        SimpleMatrix2 {
            data: res, n, m
        }
    }

    pub fn from(n: usize, m: usize, data: Vec<f32>) -> SimpleMatrix2 {
        debug_assert_eq!(data.len(), n * m, "Invalid argument!");

        SimpleMatrix2 {
            data, n, m
        }
    }

    pub fn transposed(&self) -> SimpleMatrix2 {
        let mut result = SimpleMatrix2::new(self.m, self.n);
        for i in 0..self.n {
            for j in 0..self.m {
                result[j][i] = self[i][j]
            }
        }
        result
    }
}

impl Debug for SimpleMatrix2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let sz = if self.data.len() < 8 {self.data.len()} else {8};

        write!(f, "SimpleMatrix2 {{ {:?} n={} m={} }}", &self.data[..sz], self.n, self.m)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matrix_test() {
        let a = SimpleMatrix2::from(2, 2, vec![1., 2., 3., 4.]);

        let a_t = SimpleMatrix2::from(2, 2, vec![1., 3., 2., 4.]);
        assert_eq!(a.transposed(), a_t);

        let b = SimpleMatrix2::from(2, 2, vec![5., 6., 7., 8.]);
        let c = &a * &b;

        assert_eq!(c.data, [19., 22., 43., 50.]);
    }
}
