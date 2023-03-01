use std::ops::{Add, Index, IndexMut, Mul};

pub struct Matrix44<T> {
    pub m: [[T; 4]; 4],
}

impl<T> Default for Matrix44<T>
where
    T: Default,
{
    fn default() -> Self {
        Self {
            m: Default::default(),
        }
    }
}

impl<T> Index<usize> for Matrix44<T> {
    type Output = [T; 4];

    fn index(&self, index: usize) -> &Self::Output {
        &self.m[index]
    }
}

impl<T> IndexMut<usize> for Matrix44<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.m[index]
    }
}

impl<T> Mul for Matrix44<T>
where
    T: Copy + Default + Add<Output = T> + Mul<Output = T>,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::multiply(&self, &rhs)
    }
}

impl<T> Matrix44<T> {
    pub fn new(
        a: T,
        b: T,
        c: T,
        d: T,
        e: T,
        f: T,
        g: T,
        h: T,
        i: T,
        j: T,
        k: T,
        l: T,
        m: T,
        n: T,
        o: T,
        p: T,
    ) -> Self
    where
        T: Copy,
    {
        Self {
            m: [[a, b, c, d], [e, f, g, h], [i, j, k, l], [m, n, o, p]],
        }
    }

    pub fn multiply(lhs: &Self, rhs: &Self) -> Self
    where
        T: Default + Copy + Mul<Output = T> + Add<Output = T>,
    {
        let mut result = Self {
            ..Default::default()
        };

        let b0 = rhs[0];
        let b1 = rhs[1];
        let b2 = rhs[2];
        let b3 = rhs[3];

        let a = lhs[0];
        result[0][0] = a[0] * b0[0] + a[1] * b1[0] + a[2] * b2[0] + a[3] * b3[0];
        result[0][1] = a[0] * b0[1] + a[1] * b1[1] + a[2] * b2[1] + a[3] * b3[1];
        result[0][2] = a[0] * b0[2] + a[1] * b1[2] + a[2] * b2[2] + a[3] * b3[2];
        result[0][3] = a[0] * b0[3] + a[1] * b1[3] + a[2] * b2[3] + a[3] * b3[3];

        let a = lhs[1];
        result[1][0] = a[0] * b0[0] + a[1] * b1[0] + a[2] * b2[0] + a[3] * b3[0];
        result[1][1] = a[0] * b0[1] + a[1] * b1[1] + a[2] * b2[1] + a[3] * b3[1];
        result[1][2] = a[0] * b0[2] + a[1] * b1[2] + a[2] * b2[2] + a[3] * b3[2];
        result[1][3] = a[0] * b0[3] + a[1] * b1[3] + a[2] * b2[3] + a[3] * b3[3];

        let a = lhs[2];
        result[2][0] = a[0] * b0[0] + a[1] * b1[0] + a[2] * b2[0] + a[3] * b3[0];
        result[2][1] = a[0] * b0[1] + a[1] * b1[1] + a[2] * b2[1] + a[3] * b3[1];
        result[2][2] = a[0] * b0[2] + a[1] * b1[2] + a[2] * b2[2] + a[3] * b3[2];
        result[2][3] = a[0] * b0[3] + a[1] * b1[3] + a[2] * b2[3] + a[3] * b3[3];

        let a = lhs[3];
        result[3][0] = a[0] * b0[0] + a[1] * b1[0] + a[2] * b2[0] + a[3] * b3[0];
        result[3][1] = a[0] * b0[1] + a[1] * b1[1] + a[2] * b2[1] + a[3] * b3[1];
        result[3][2] = a[0] * b0[2] + a[1] * b1[2] + a[2] * b2[2] + a[3] * b3[2];
        result[3][3] = a[0] * b0[3] + a[1] * b1[3] + a[2] * b2[3] + a[3] * b3[3];
        result
    }

    pub fn transpose(&mut self)
    where
        T: Copy,
    {
        let tmp = Self::new(
            self[0][0], self[1][0], self[2][0], self[3][0], self[0][1], self[1][1], self[2][1],
            self[3][1], self[0][2], self[1][2], self[2][2], self[3][2], self[0][3], self[1][3],
            self[2][3], self[3][3],
        );

        *self = tmp;
    }

    pub fn transposed(&self) -> Self
    where
        T: Copy,
    {
        Self::new(
            self[0][0], self[1][0], self[2][0], self[3][0], self[0][1], self[1][1], self[2][1],
            self[3][1], self[0][2], self[1][2], self[2][2], self[3][2], self[0][3], self[1][3],
            self[2][3], self[3][3],
        )
    }
}

pub type Matrix44f64 = Matrix44<f64>;
pub type Matrix44f32 = Matrix44<f32>;

#[cfg(test)]
mod tests {
    use super::{Matrix44, Matrix44f64};

    #[test]
    fn create_matrix() {
        let m1 = Matrix44f64 {
            ..Default::default()
        };
        for i in 0..4 {
            for j in 0..4 {
                assert_eq!(m1[i][j], 0f64);
            }
        }

        let m2 = Matrix44f64::new(
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
        );
        assert_eq!(m2[0][0], 1.0);
        assert_eq!(m2[0][1], 2.0);
        assert_eq!(m2[0][2], 3.0);
        assert_eq!(m2[0][3], 4.0);
        assert_eq!(m2[1][0], 5.0);
        assert_eq!(m2[1][1], 6.0);
        assert_eq!(m2[1][2], 7.0);
        assert_eq!(m2[1][3], 8.0);
        assert_eq!(m2[2][0], 9.0);
        assert_eq!(m2[2][1], 10.0);
        assert_eq!(m2[2][2], 11.0);
        assert_eq!(m2[2][3], 12.0);
        assert_eq!(m2[3][0], 13.0);
        assert_eq!(m2[3][1], 14.0);
        assert_eq!(m2[3][2], 15.0);
        assert_eq!(m2[3][3], 16.0);
    }

    #[test]
    fn multiply_matrieces() {
        let m1 = Matrix44::new(5, 7, 9, 10, 2, 3, 3, 8, 8, 10, 2, 3, 3, 3, 4, 8);
        let m2 = Matrix44::new(3, 10, 12, 18, 12, 1, 4, 9, 9, 10, 12, 2, 3, 12, 4, 10);
        let result = Matrix44::multiply(&m1, &m2);
        assert_eq!(result[0][0], 210);
        assert_eq!(result[0][1], 267);
        assert_eq!(result[0][2], 236);
        assert_eq!(result[0][3], 271);
        assert_eq!(result[1][0], 93);
        assert_eq!(result[1][1], 149);
        assert_eq!(result[1][2], 104);
        assert_eq!(result[1][3], 149);
        assert_eq!(result[2][0], 171);
        assert_eq!(result[2][1], 146);
        assert_eq!(result[2][2], 172);
        assert_eq!(result[2][3], 268);
        assert_eq!(result[3][0], 105);
        assert_eq!(result[3][1], 169);
        assert_eq!(result[3][2], 128);
        assert_eq!(result[3][3], 169);

        let result = m1 * m2;
        assert_eq!(result[0][0], 210);
        assert_eq!(result[0][1], 267);
        assert_eq!(result[0][2], 236);
        assert_eq!(result[0][3], 271);
        assert_eq!(result[1][0], 93);
        assert_eq!(result[1][1], 149);
        assert_eq!(result[1][2], 104);
        assert_eq!(result[1][3], 149);
        assert_eq!(result[2][0], 171);
        assert_eq!(result[2][1], 146);
        assert_eq!(result[2][2], 172);
        assert_eq!(result[2][3], 268);
        assert_eq!(result[3][0], 105);
        assert_eq!(result[3][1], 169);
        assert_eq!(result[3][2], 128);
        assert_eq!(result[3][3], 169);
    }

    #[test]
    fn transpose_test() {
        let mut m = Matrix44::new(5, 7, 9, 10, 2, 3, 3, 8, 8, 10, 2, 3, 3, 3, 4, 8);
        m.transpose();
        assert_eq!(m[0][0], 5);
        assert_eq!(m[0][1], 2);
        assert_eq!(m[0][2], 8);
        assert_eq!(m[0][3], 3);
        assert_eq!(m[1][0], 7);
        assert_eq!(m[1][1], 3);
        assert_eq!(m[1][2], 10);
        assert_eq!(m[1][3], 3);
        assert_eq!(m[2][0], 9);
        assert_eq!(m[2][1], 3);
        assert_eq!(m[2][2], 2);
        assert_eq!(m[2][3], 4);
        assert_eq!(m[3][0], 10);
        assert_eq!(m[3][1], 8);
        assert_eq!(m[3][2], 3);
        assert_eq!(m[3][3], 8);

        let transposed = m.transposed();
        assert_eq!(transposed[0][0], 5);
        assert_eq!(transposed[0][1], 7);
        assert_eq!(transposed[0][2], 9);
        assert_eq!(transposed[0][3], 10);
        assert_eq!(transposed[1][0], 2);
        assert_eq!(transposed[1][1], 3);
        assert_eq!(transposed[1][2], 3);
        assert_eq!(transposed[1][3], 8);
        assert_eq!(transposed[2][0], 8);
        assert_eq!(transposed[2][1], 10);
        assert_eq!(transposed[2][2], 2);
        assert_eq!(transposed[2][3], 3);
        assert_eq!(transposed[3][0], 3);
        assert_eq!(transposed[3][1], 3);
        assert_eq!(transposed[3][2], 4);
        assert_eq!(transposed[3][3], 8);
    }
}
