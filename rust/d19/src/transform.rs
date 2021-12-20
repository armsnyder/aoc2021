use std::ops::{Add, Index, Mul, MulAssign, Neg, Sub};

pub type Coord = i32;

#[derive(Default, Eq, PartialEq, Hash, Copy, Clone, Ord, PartialOrd)]
pub struct Vector3 {
    pub x: Coord,
    pub y: Coord,
    pub z: Coord,
}

impl Vector3 {
    pub fn manhattan_distance(self) -> Coord {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl From<String> for Vector3 {
    fn from(s: String) -> Self {
        let mut output = Vector3::default();
        let mut split = s.split(",");
        output.x = split.next().unwrap().parse().unwrap();
        output.y = split.next().unwrap().parse().unwrap();
        output.z = split.next().unwrap().parse().unwrap();
        output
    }
}

impl Sub for Vector3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Neg for Vector3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vector3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Matrix4([[Coord; 4]; 4]);

impl Mul for Matrix4 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Matrix4([
            [
                self[0][0] * rhs[0][0] + self[0][1] * rhs[1][0] + self[0][2] * rhs[2][0] + self[0][3] * rhs[3][0],
                self[0][0] * rhs[0][1] + self[0][1] * rhs[1][1] + self[0][2] * rhs[2][1] + self[0][3] * rhs[3][1],
                self[0][0] * rhs[0][2] + self[0][1] * rhs[1][2] + self[0][2] * rhs[2][2] + self[0][3] * rhs[3][2],
                self[0][0] * rhs[0][3] + self[0][1] * rhs[1][3] + self[0][2] * rhs[2][3] + self[0][3] * rhs[3][3],
            ],
            [
                self[1][0] * rhs[0][0] + self[1][1] * rhs[1][0] + self[1][2] * rhs[2][0] + self[1][3] * rhs[3][0],
                self[1][0] * rhs[0][1] + self[1][1] * rhs[1][1] + self[1][2] * rhs[2][1] + self[1][3] * rhs[3][1],
                self[1][0] * rhs[0][2] + self[1][1] * rhs[1][2] + self[1][2] * rhs[2][2] + self[1][3] * rhs[3][2],
                self[1][0] * rhs[0][3] + self[1][1] * rhs[1][3] + self[1][2] * rhs[2][3] + self[1][3] * rhs[3][3],
            ],
            [
                self[2][0] * rhs[0][0] + self[2][1] * rhs[1][0] + self[2][2] * rhs[2][0] + self[2][3] * rhs[3][0],
                self[2][0] * rhs[0][1] + self[2][1] * rhs[1][1] + self[2][2] * rhs[2][1] + self[2][3] * rhs[3][1],
                self[2][0] * rhs[0][2] + self[2][1] * rhs[1][2] + self[2][2] * rhs[2][2] + self[2][3] * rhs[3][2],
                self[2][0] * rhs[0][3] + self[2][1] * rhs[1][3] + self[2][2] * rhs[2][3] + self[2][3] * rhs[3][3],
            ],
            [
                self[3][0] * rhs[0][0] + self[3][1] * rhs[1][0] + self[3][2] * rhs[2][0] + self[3][3] * rhs[3][0],
                self[3][0] * rhs[0][1] + self[3][1] * rhs[1][1] + self[3][2] * rhs[2][1] + self[3][3] * rhs[3][1],
                self[3][0] * rhs[0][2] + self[3][1] * rhs[1][2] + self[3][2] * rhs[2][2] + self[3][3] * rhs[3][2],
                self[3][0] * rhs[0][3] + self[3][1] * rhs[1][3] + self[3][2] * rhs[2][3] + self[3][3] * rhs[3][3],
            ],
        ])
    }
}

impl MulAssign for Matrix4 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl Mul<Vector3> for Matrix4 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Self::Output {
        Vector3 {
            x: self[0][0] * rhs.x + self[0][1] * rhs.y + self[0][2] * rhs.z + self[0][3],
            y: self[1][0] * rhs.x + self[1][1] * rhs.y + self[1][2] * rhs.z + self[1][3],
            z: self[2][0] * rhs.x + self[2][1] * rhs.y + self[2][2] * rhs.z + self[2][3],
        }
    }
}

impl Add<Vector3> for Matrix4 {
    type Output = Self;

    fn add(self, rhs: Vector3) -> Self::Output {
        Matrix4([
            [self[0][0], self[0][1], self[0][2], self[0][3] + rhs.x],
            [self[1][0], self[1][1], self[1][2], self[1][3] + rhs.y],
            [self[2][0], self[2][1], self[2][2], self[2][3] + rhs.z],
            [self[3][0], self[3][1], self[3][2], self[3][3]],
        ])
    }
}

impl Index<usize> for Matrix4 {
    type Output = [Coord; 4];

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl Matrix4 {
    pub const IDENTITY: Matrix4 = Matrix4([
        [1, 0, 0, 0],
        [0, 1, 0, 0],
        [0, 0, 1, 0],
        [0, 0, 0, 1],
    ]);

    pub const ROTATE_X_90: Matrix4 = Matrix4([
        [1, 0, 0, 0],
        [0, 0, -1, 0],
        [0, 1, 0, 0],
        [0, 0, 0, 1],
    ]);

    pub const ROTATE_Y_90: Matrix4 = Matrix4([
        [0, 0, 1, 0],
        [0, 1, 0, 0],
        [-1, 0, 0, 0],
        [0, 0, 0, 1],
    ]);

    pub const ROTATE_Z_90: Matrix4 = Matrix4([
        [0, -1, 0, 0],
        [1, 0, 0, 0],
        [0, 0, 1, 0],
        [0, 0, 0, 1],
    ]);

    pub fn translation(self) -> Vector3 {
        Vector3 {
            x: self[0][3],
            y: self[1][3],
            z: self[2][3],
        }
    }
}

lazy_static! {
    pub static ref ALL_ORIENTATIONS: Vec<Matrix4> = {
        let rotate_x_180 = Matrix4::ROTATE_X_90 * Matrix4::ROTATE_X_90;
        let rotate_x_270 = rotate_x_180 * Matrix4::ROTATE_X_90;
        let rotate_y_180 = Matrix4::ROTATE_Y_90 * Matrix4::ROTATE_Y_90;
        let rotate_y_270 = rotate_y_180 * Matrix4::ROTATE_Y_90;
        let rotate_z_180 = Matrix4::ROTATE_Z_90 * Matrix4::ROTATE_Z_90;
        let rotate_z_270 = rotate_z_180 * Matrix4::ROTATE_Z_90;

        // All 24 orientations, using +x as default face.
        vec![
            // +x
            Matrix4::IDENTITY,
            Matrix4::ROTATE_X_90,
            rotate_x_180,
            rotate_x_270,
            // -x
            rotate_z_180,
            rotate_z_180 * Matrix4::ROTATE_X_90,
            rotate_z_180 * rotate_x_180,
            rotate_z_180 * rotate_x_270,
            // +y
            rotate_z_270,
            rotate_z_270 * Matrix4::ROTATE_Y_90,
            rotate_z_270 * rotate_y_180,
            rotate_z_270 * rotate_y_270,
            // -y
            Matrix4::ROTATE_Z_90,
            Matrix4::ROTATE_Z_90 * Matrix4::ROTATE_Y_90,
            Matrix4::ROTATE_Z_90 * rotate_y_180,
            Matrix4::ROTATE_Z_90 * rotate_y_270,
            // +z
            Matrix4::ROTATE_Y_90,
            Matrix4::ROTATE_Y_90 * Matrix4::ROTATE_Z_90,
            Matrix4::ROTATE_Y_90 * rotate_z_180,
            Matrix4::ROTATE_Y_90 * rotate_z_270,
            // -z
            rotate_y_270,
            rotate_y_270 * Matrix4::ROTATE_Z_90,
            rotate_y_270 * rotate_z_180,
            rotate_y_270 * rotate_z_270,
        ]
    };
}
