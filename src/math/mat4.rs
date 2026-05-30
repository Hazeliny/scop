// Mat4 — 4x4 matrix math (no external lib)
use super::vec3::Vec3;

/// Column-major 4x4 matrix — matches OpenGL's expected memory layout.
/// data[col][row], so data[0] is the first column.
///
/// Visual layout (what mathematicians write):
/// | data[0][0]  data[1][0]  data[2][0]  data[3][0] |
/// | data[0][1]  data[1][1]  data[2][1]  data[3][1] |
/// | data[0][2]  data[1][2]  data[2][2]  data[3][2] |
/// | data[0][3]  data[1][3]  data[2][3]  data[3][3] |
#[derive(Debug, Clone, Copy)]
pub struct Mat4 {
    pub data: [[f32; 4]; 4],
}

impl Mat4 {
    // ── Constructors ──────────────────────────────────────────────────────────

    pub fn zero() -> Self {
        Mat4 { data: [[0.0; 4]; 4] }
    }

    /// Identity matrix — multiplying by this changes nothing
    pub fn identity() -> Self {
        let mut m = Mat4::zero();
        m.data[0][0] = 1.0;
        m.data[1][1] = 1.0;
        m.data[2][2] = 1.0;
        m.data[3][3] = 1.0;
        m
    }

    // ── Transform matrices ────────────────────────────────────────────────────

    /// Move the object by (x, y, z)
    pub fn translation(x: f32, y: f32, z: f32) -> Self {
        let mut m = Mat4::identity();
        m.data[3][0] = x;
        m.data[3][1] = y;
        m.data[3][2] = z;
        m
    }

    /// Uniform scale
    pub fn scale(x: f32, y: f32, z: f32) -> Self {
        let mut m = Mat4::identity();
        m.data[0][0] = x;
        m.data[1][1] = y;
        m.data[2][2] = z;
        m
    }

    /// Rotate around the X axis by `angle` radians
    pub fn rotation_x(angle: f32) -> Self {
        let mut m = Mat4::identity();
        let c = angle.cos();
        let s = angle.sin();
        m.data[1][1] =  c;
        m.data[1][2] =  s;
        m.data[2][1] = -s;
        m.data[2][2] =  c;
        m
    }

    /// Rotate around the Y axis by `angle` radians
    pub fn rotation_y(angle: f32) -> Self {
        let mut m = Mat4::identity();
        let c = angle.cos();
        let s = angle.sin();
        m.data[0][0] =  c;
        m.data[0][2] = -s;
        m.data[2][0] =  s;
        m.data[2][2] =  c;
        m
    }

    /// Rotate around the Z axis by `angle` radians
    pub fn rotation_z(angle: f32) -> Self {
        let mut m = Mat4::identity();
        let c = angle.cos();
        let s = angle.sin();
        m.data[0][0] =  c;
        m.data[0][1] =  s;
        m.data[1][0] = -s;
        m.data[1][1] =  c;
        m
    }

    // ── Camera matrices ───────────────────────────────────────────────────────

    /// View matrix: positions the camera in the world.
    /// eye    = camera position
    /// center = point the camera looks at
    /// up     = which direction is "up" (usually Y axis)
    pub fn look_at(eye: Vec3, center: Vec3, up: Vec3) -> Self {
        let f = (center - eye).normalize();       // forward
        let r = f.cross(up).normalize();          // right
        let u = r.cross(f);                       // true up

        let mut m = Mat4::identity();
        m.data[0][0] =  r.x;
        m.data[1][0] =  r.y;
        m.data[2][0] =  r.z;
        m.data[0][1] =  u.x;
        m.data[1][1] =  u.y;
        m.data[2][1] =  u.z;
        m.data[0][2] = -f.x;
        m.data[1][2] = -f.y;
        m.data[2][2] = -f.z;
        m.data[3][0] = -r.dot(eye);
        m.data[3][1] = -u.dot(eye);
        m.data[3][2] =  f.dot(eye);
        m
    }

    /// Perspective projection matrix.
    /// fov_y  = vertical field of view in radians (e.g. 45° = PI/4)
    /// aspect = width / height
    /// near   = near clipping plane distance
    /// far    = far clipping plane distance
    pub fn perspective(fov_y: f32, aspect: f32, near: f32, far: f32) -> Self {
        let mut m = Mat4::zero();
        let tan_half = (fov_y / 2.0).tan();
        m.data[0][0] = 1.0 / (aspect * tan_half);
        m.data[1][1] = 1.0 / tan_half;
        m.data[2][2] = -(far + near) / (far - near);
        m.data[2][3] = -1.0;
        m.data[3][2] = -(2.0 * far * near) / (far - near);
        m
    }

    // ── Utility ───────────────────────────────────────────────────────────────

    /// Returns a flat [f32; 16] array for passing to OpenGL uniforms
    pub fn as_ptr(&self) -> *const f32 {
        self.data.as_ptr() as *const f32
    }
}

// Matrix multiplication: self × other
use std::ops::Mul;

impl Mul for Mat4 {
    type Output = Mat4;
    fn mul(self, other: Mat4) -> Mat4 {
        let mut result = Mat4::zero();
        for col in 0..4 {
            for row in 0..4 {
                result.data[col][row] =
                    self.data[0][row] * other.data[col][0]
                  + self.data[1][row] * other.data[col][1]
                  + self.data[2][row] * other.data[col][2]
                  + self.data[3][row] * other.data[col][3];
            }
        }
        result
    }
}