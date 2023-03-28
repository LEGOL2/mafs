use std::ops::{Add, Div, Index, IndexMut, Mul, MulAssign, Sub};

pub trait Sqrt {
    fn sqrt(&self) -> Self;
}

pub trait Tuple<T> {
    fn new(x: T, y: T, z: T) -> Self
    where
        T: Copy;

    fn zeros() -> Self
    where
        T: Default;

    fn normalize(&mut self)
    where
        T: Copy
            + Add<Output = T>
            + Mul<Output = T>
            + Div<Output = T>
            + MulAssign
            + Sqrt
            + PartialOrd,
        f32: Into<T>,
        f64: Into<T>;
}

pub trait Vector<T>: Tuple<T> {
    fn dot(lhs: &Self, rhs: &Self) -> T
    where
        T: Copy + Add<Output = T> + Mul<Output = T>;

    fn cross(&self, other: &Self) -> Self
    where
        T: Copy + Add<Output = T> + Mul<Output = T> + Sub<Output = T>;

    fn magnitude(&self) -> T
    where
        T: Copy + Add<Output = T> + Mul<Output = T> + Sqrt;
}

pub trait Point<T>: Tuple<T> {
    fn distance_from_origin(&self) -> T
    where
        T: Copy + Add<Output = T> + Mul<Output = T> + Sqrt;
}

#[derive(Default, Clone, Copy, PartialEq)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

#[derive(Default, Clone, Copy, PartialEq)]
pub struct Point3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Tuple<T> for Vec3<T> {
    fn new(x: T, y: T, z: T) -> Self
    where
        T: Copy,
    {
        Self { x, y, z }
    }

    fn zeros() -> Self
    where
        T: Default,
    {
        Self {
            ..Default::default()
        }
    }

    fn normalize(&mut self)
    where
        T: Copy
            + Add<Output = T>
            + Mul<Output = T>
            + Div<Output = T>
            + MulAssign
            + Sqrt
            + PartialOrd,
        f32: Into<T>,
        f64: Into<T>,
    {
        let len = self.magnitude();
        if len > 0.0.into() {
            let inv_len = 1.0.into() / len;
            self.x *= inv_len;
            self.y *= inv_len;
            self.z *= inv_len;
        }
    }
}

impl<T> Tuple<T> for Point3<T> {
    fn new(x: T, y: T, z: T) -> Self
    where
        T: Copy,
    {
        Self { x, y, z }
    }

    fn zeros() -> Self
    where
        T: Default,
    {
        Self {
            ..Default::default()
        }
    }

    fn normalize(&mut self)
    where
        T: Copy
            + Add<Output = T>
            + Mul<Output = T>
            + Div<Output = T>
            + MulAssign
            + Sqrt
            + PartialOrd,
        f32: Into<T>,
        f64: Into<T>,
    {
        let len = self.distance_from_origin();
        if len > 0.0.into() {
            let inv_len = 1.0.into() / len;
            self.x *= inv_len;
            self.y *= inv_len;
            self.z *= inv_len;
        }
    }
}

impl Sqrt for f32 {
    fn sqrt(&self) -> Self {
        f32::sqrt(*self)
    }
}

impl Sqrt for f64 {
    fn sqrt(&self) -> Self {
        f64::sqrt(*self)
    }
}

impl<T> Vector<T> for Vec3<T> {
    fn dot(lhs: &Self, rhs: &Self) -> T
    where
        T: Copy + Add<Output = T> + Mul<Output = T>,
    {
        lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z
    }

    fn cross(&self, other: &Self) -> Self
    where
        T: Copy + Add<Output = T> + Mul<Output = T> + Sub<Output = T>,
    {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    fn magnitude(&self) -> T
    where
        T: Copy + Add<Output = T> + Mul<Output = T> + Sqrt,
    {
        let value = self.x * self.x + self.y * self.y + self.z * self.z;
        value.sqrt()
    }
}

impl<T: Add<Output = T>> Add for Vec3<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Vec3<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T: Copy + Add<Output = T> + Mul<Output = T> + Sub<Output = T>> Mul for Vec3<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self.cross(&rhs)
    }
}

impl<T: Add<Output = T>> Add for Point3<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Point3<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T> Point<T> for Point3<T> {
    fn distance_from_origin(&self) -> T
    where
        T: Copy + Add<Output = T> + Mul<Output = T> + Sqrt,
    {
        let value = self.x * self.x + self.y * self.y + self.z * self.z;
        value.sqrt()
    }
}

impl<T> Index<usize> for Vec3<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Out of bound access in Vec3<T>!"),
        }
    }
}

impl<T> IndexMut<usize> for Vec3<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Out of bound access in Vec3<T>!"),
        }
    }
}

impl<T> Index<usize> for Point3<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Out of bound access in Point3<T>!"),
        }
    }
}

impl<T> IndexMut<usize> for Point3<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Out of bound access in Point3<T>!"),
        }
    }
}

pub type Vec3d = Vec3<f64>;
pub type Vec3f = Vec3<f32>;

pub type Point3d = Point3<f64>;
pub type Point3f = Point3<f32>;

#[cfg(test)]
mod tests {
    use super::{Point3, Tuple, Vec3, Vector};

    #[macro_export]
    macro_rules! assert_approx_eq {
        ($a:expr, $b:expr) => {{
            let eps = 1.0e-6;
            let (a, b) = (&$a, &$b);
            assert!(
                (*a - *b).abs() < eps,
                "assertion failed: `(left !== right)` \
                 (left: `{:?}`, right: `{:?}`, expect diff: `{:?}`, real diff: `{:?}`)",
                *a,
                *b,
                eps,
                (*a - *b).abs()
            );
        }};
        ($a:expr, $b:expr, $eps:expr) => {{
            let (a, b) = (&$a, &$b);
            let eps = $eps;
            assert!(
                (*a - *b).abs() < eps,
                "assertion failed: `(left !== right)` \
                 (left: `{:?}`, right: `{:?}`, expect diff: `{:?}`, real diff: `{:?}`)",
                *a,
                *b,
                eps,
                (*a - *b).abs()
            );
        }};
    }

    #[test]
    fn create_and_modify_vector() {
        let zeros: Vec3<f64> = Vec3::zeros();
        assert_eq!(zeros.x, 0.0);
        assert_eq!(zeros[0], 0.0);
        assert_eq!(zeros.y, 0.0);
        assert_eq!(zeros[1], 0.0);
        assert_eq!(zeros.z, 0.0);
        assert_eq!(zeros[2], 0.0);

        let vec: Vec3<f64> = Vec3::new(1.1, 2.2, 3.3);
        assert_eq!(vec.x, 1.1);
        assert_eq!(vec.y, 2.2);
        assert_eq!(vec.z, 3.3);

        let mut vec: Vec3<f64> = Vec3::zeros();
        vec[0] = 1.1;
        vec[1] = 2.2;
        vec[2] = 3.3;
        assert_eq!(vec[0], 1.1);
        assert_eq!(vec[1], 2.2);
        assert_eq!(vec[2], 3.3);
    }

    #[test]
    fn create_point() {
        let zeros: Point3<f64> = Point3::zeros();
        assert_eq!(zeros.x, 0.0);
        assert_eq!(zeros.y, 0.0);
        assert_eq!(zeros.z, 0.0);

        let vec: Point3<f64> = Point3::new(1.1, 2.2, 3.3);
        assert_eq!(vec.x, 1.1);
        assert_eq!(vec.y, 2.2);
        assert_eq!(vec.z, 3.3);
    }

    #[test]
    fn generic_sqrt() {
        let vec: Vec3<f64> = Vec3::new(4.0, 49.0, 64.0);
        assert_eq!(vec.x.sqrt(), 2.0);
        assert_eq!(vec.y.sqrt(), 7.0);
        assert_eq!(vec.z.sqrt(), 8.0);

        let vec: Vec3<f32> = Vec3::new(4.0, 49.0, 64.0);
        assert_eq!(vec.x.sqrt(), 2.0);
        assert_eq!(vec.y.sqrt(), 7.0);
        assert_eq!(vec.z.sqrt(), 8.0);
    }

    #[test]
    fn dot_product() {
        let lhs: Vec3<f64> = Vec3::new(3.1, 5.0, -2.0);
        let rhs: Vec3<f64> = Vec3::new(11.27, -9.0, 0.0);
        let dot = Vec3::dot(&lhs, &rhs);

        assert_approx_eq!(dot, -10.063, 1e-12);
    }

    #[test]
    fn cross_product() {
        let lhs: Vec3<f64> = Vec3::new(3.1, 5.0, -2.0);
        let rhs: Vec3<f64> = Vec3::new(11.27, -9.0, 0.0);
        let vec = Vec3::cross(&lhs, &rhs);
        assert_eq!(vec.x, -18.0);
        assert_eq!(vec.y, -22.54);
        assert_eq!(vec.z, -84.25);

        let vec = lhs * rhs;
        assert_eq!(vec.x, -18.0);
        assert_eq!(vec.y, -22.54);
        assert_eq!(vec.z, -84.25);
    }

    #[test]
    fn vec_magnitude() {
        let vec: Vec3<f64> = Vec3::new(2.0, 0.0, 0.0);
        assert_eq!(vec.magnitude(), 2.0);
    }

    #[test]
    fn basic_vector_ops() {
        let lhs: Vec3<f64> = Vec3::new(1.0, 2.0, 3.0);
        let rhs: Vec3<f64> = Vec3::new(4.0, 5.0, 6.0);
        let sum = lhs + rhs;
        let diff = lhs - rhs;

        assert_eq!(sum.x, 5.0);
        assert_eq!(sum.y, 7.0);
        assert_eq!(sum.z, 9.0);

        assert_eq!(diff.x, -3.0);
        assert_eq!(diff.y, -3.0);
        assert_eq!(diff.z, -3.0);
    }

    #[test]
    fn normalize_vector_and_point() {
        let mut vec: Vec3<f64> = Vec3::new(1.0, 2.0, 3.0);
        vec.normalize();
        assert_approx_eq!(vec.x, 0.2672612419124244, 1e-12);
        assert_approx_eq!(vec.y, 0.5345224838248488, 1e-12);
        assert_approx_eq!(vec.z, 0.8017837257372732, 1e-12);

        let mut point: Point3<f64> = Point3::new(1.0, 2.0, 3.0);
        point.normalize();
        assert_approx_eq!(point.x, 0.2672612419124244, 1e-12);
        assert_approx_eq!(point.y, 0.5345224838248488, 1e-12);
        assert_approx_eq!(point.z, 0.8017837257372732, 1e-12);
    }
}
