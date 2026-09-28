use crate::utils::cv_err;
use opencv::{
    core::{self, Mat, Point2f, Scalar, Size},
    imgproc,
    prelude::*,
};

const CANVAS_W: i32 = 1280;
const CANVAS_H: i32 = 720;

#[derive(Clone, Copy, Debug)]
pub enum Op {
    Canvas { scale: f64 },
    Blur { k: i32 }, 
    Rotate { deg: f64 },
    LowContrast,
    Noise { sigma: f64 },
}

pub struct Condition {
    pub name: &'static str,
    pub ops: Vec<Op>,
}

pub fn standard_conditions() -> Vec<Condition> {
    vec![
        Condition { name: "clean", ops: vec![] },
        Condition { name: "canvas4x", ops: vec![Op::Canvas { scale: 4.0 }] },
        Condition { name: "canvas4x+blur", ops: vec![Op::Canvas { scale: 4.0 }, Op::Blur { k: 5 }] },
        Condition { name: "canvas4x+rot20", ops: vec![Op::Canvas { scale: 4.0 }, Op::Rotate { deg: 20.0 }] },
        Condition { name: "canvas4x+lowcontrast", ops: vec![Op::Canvas { scale: 4.0 }, Op::LowContrast] },
        Condition { name: "canvas4x+noise", ops: vec![Op::Canvas { scale: 4.0 }, Op::Noise { sigma: 15.0 }] },
    ]
}

pub fn apply(src: &Mat, ops: &[Op]) -> Result<Mat, String> {
    let mut cur = src.try_clone().map_err(cv_err)?;
    for op in ops {
        cur = apply_one(&cur, *op)?;
    }
    Ok(cur)
}

fn apply_one(src: &Mat, op: Op) -> Result<Mat, String> {
    let mut dst = Mat::default();
    match op {
        Op::Canvas { scale } => {
            let w = (src.cols() as f64 * scale).round() as i32;
            let h = (src.rows() as f64 * scale).round() as i32;
            if w > CANVAS_W || h > CANVAS_H {
                return Err(format!("scale {scale} terlalu besar untuk kanvas ({w}x{h})"));
            }
            let mut resized = Mat::default();
            imgproc::resize(src, &mut resized, Size::new(w, h), 0.0, 0.0, imgproc::INTER_LINEAR)
                .map_err(cv_err)?;
            let (left, top) = ((CANVAS_W - w) / 2, (CANVAS_H - h) / 2);
            core::copy_make_border(
                &resized,
                &mut dst,
                top,
                CANVAS_H - h - top,
                left,
                CANVAS_W - w - left,
                core::BORDER_CONSTANT,
                Scalar::all(255.0),
            )
            .map_err(cv_err)?;
        }
        Op::Blur { k } => {
            imgproc::gaussian_blur_def(src, &mut dst, Size::new(k, k), 0.0).map_err(cv_err)?;
        }
        Op::Rotate { deg } => {
            let center = Point2f::new(src.cols() as f32 / 2.0, src.rows() as f32 / 2.0);
            let m = imgproc::get_rotation_matrix_2d(center, deg, 1.0).map_err(cv_err)?;
            imgproc::warp_affine(
                src,
                &mut dst,
                &m,
                src.size().map_err(cv_err)?,
                imgproc::INTER_LINEAR,
                core::BORDER_CONSTANT,
                Scalar::all(255.0), 
            )
            .map_err(cv_err)?;
        }
        Op::LowContrast => {
            // dst = src * 0.4 + 90
            src.convert_to(&mut dst, -1, 0.4, 90.0).map_err(cv_err)?;
        }
        Op::Noise { sigma } => {
            let mut s16 = Mat::default();
            src.convert_to(&mut s16, core::CV_16S, 1.0, 0.0).map_err(cv_err)?;
            let mut noise = Mat::new_size_with_default(
                s16.size().map_err(cv_err)?,
                s16.typ(),
                Scalar::all(0.0),
            )
            .map_err(cv_err)?;
            core::randn(&mut noise, &Scalar::all(0.0), &Scalar::all(sigma)).map_err(cv_err)?;
            let mut sum = Mat::default();
            core::add(&s16, &noise, &mut sum, &core::no_array(), -1).map_err(cv_err)?;
            sum.convert_to(&mut dst, core::CV_8U, 1.0, 0.0).map_err(cv_err)?; // saturate ke 0..255
        }
    }
    Ok(dst)
}