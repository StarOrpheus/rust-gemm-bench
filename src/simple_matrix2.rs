use std::ops::{Index, IndexMut, Mul};

#[derive(Debug)]
pub struct SimpleMatrix2 {
    data: Vec<f32>,
    n: usize,
    m: usize
}

impl Index<usize> for SimpleMatrix2 {
    type Output = [f32];

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.n, "Invalid argument: Out of range!");

        &self.data[index * self.m..(index + 1) * self.m]
    }
}

impl IndexMut<usize> for SimpleMatrix2 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < self.n, "Invalid argument: Out of range!");

        &mut self.data[index * self.m..(index + 1) * self.m]
    }
}

impl Mul for &SimpleMatrix2 {
    type Output = SimpleMatrix2;

    fn mul(self, rhs: Self) -> Self::Output {
        assert_eq!(self.m, rhs.n, "Invalid arguments!");

        let mut result = SimpleMatrix2::new(self.n, rhs.m);

        for i in 0..self.n {
            for k in 0..rhs.m {
                let mut r = 0f32;
                for j in 0..self.m {
                    r += self[i][j] * rhs[j][k]
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
        assert_eq!(data.len(), n * m, "Invalid argument!");

        SimpleMatrix2 {
            data, n, m
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matrix_test() {
        let a = SimpleMatrix2::from(2, 2, vec![1., 2., 3., 4.]);
        let b = SimpleMatrix2::from(2, 2, vec![5., 6., 7., 8.]);
        let c = &a * &b;

        assert_eq!(c.data, [19., 22., 43., 50.]);
    }
}
