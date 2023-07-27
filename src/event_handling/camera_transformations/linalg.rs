pub fn mag(vec: [f32; 3]) -> f32 {
    (vec[0] * vec[0] + vec[1] * vec[1] + vec[2] * vec[2]).sqrt()
}

pub fn norm(vec: [f32; 3]) -> [f32; 3] {
    let mag = mag(vec);
    if mag > f32::EPSILON {
        [ vec[0] / mag, vec[1] / mag, vec[2] / mag ]
    } else {
        [ 0.0, 0.0, 0.0 ]
    }
}

#[allow(dead_code)]
pub fn cross(vec0: [f32; 3], vec1: [f32; 3]) -> [f32; 3] {
    [
        vec0[1] * vec1[2] - vec0[2] * vec1[1],
        vec0[2] * vec1[0] - vec0[0] * vec1[2],
        vec0[0] * vec1[1] - vec0[1] * vec1[0]
    ]
}

pub fn matmulvec3(mat: [[f32; 3]; 3], vec: [f32; 3]) -> [f32; 3] {
    [
        mat[0][0] * vec[0] + mat[0][1] * vec[1] + mat[0][2] * vec[2],
        mat[1][0] * vec[0] + mat[1][1] * vec[1] + mat[1][2] * vec[2],
        mat[2][0] * vec[0] + mat[2][1] * vec[1] + mat[2][2] * vec[2]           
    ]
}

pub fn rotmat(axis: [f32; 3], angle: f32) -> [[f32; 3]; 3] {
    let norm : f32 = (axis[0] * axis[0] + axis[1] * axis[1] + axis[2] * axis[2]).sqrt();
    let vn : [f32; 3] = [axis[0] / norm, axis[1] / norm, axis[2] / norm];
    let sinth : f32 = (0.5 * angle).sin();
    let costh : f32 = (0.5 * angle).cos();
    [
        [
            1.0 - 2.0 * (vn[1] * vn[1] + vn[2] * vn[2]) * sinth * sinth,
            2.0 * (vn[0] * vn[1] * sinth * sinth + vn[2] * sinth * costh),
            2.0 * (vn[0] * vn[2] * sinth * sinth - vn[1] * sinth * costh)
        ],
        [
            2.0 * (vn[0] * vn[1] * sinth * sinth - vn[2] * sinth * costh),
            1.0 - 2.0 * (vn[0] * vn[0] + vn[2] * vn[2]) * sinth * sinth,
            2.0 * (vn[1] * vn[2] * sinth * sinth + vn[0] * sinth * costh)
        ],
        [
            2.0 * (vn[0] * vn[2] * sinth * sinth + vn[1] * sinth * costh),
            2.0 * (vn[1] * vn[2] * sinth * sinth - vn[0] * sinth * costh),
            1.0 - 2.0 * (vn[0] * vn[0] + vn[1] * vn[1]) * sinth * sinth
        ]
    ]    
}

