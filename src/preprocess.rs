use crate::utils::cv_err; 
use opencv::{
    core::{Mat, Ptr, Size}, 
    imgproc::{self, CLAHE}, 
    prelude::*
}; 

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PreprocessKind {
    Raw, // raw img
    Gray,  // only grayscale
    GrayClahe, // with locale contras
    GrayClaheBlur, // with blur 3x3
    GrayOtsu // grayscale + otsu
} 

impl PreprocessKind {
    pub const ALL: [PreprocessKind; 5] = [
        PreprocessKind::Raw, 
        PreprocessKind::Gray, 
        PreprocessKind::GrayClahe, 
        PreprocessKind::GrayClaheBlur, 
        PreprocessKind::GrayOtsu
    ]; 

    pub fn label(self) -> &'static str {
        match self {
            PreprocessKind::Raw => "raw", 
            PreprocessKind::Gray => "gray", 
            PreprocessKind::GrayClahe => "gray_clahe", 
            PreprocessKind::GrayClaheBlur => "gray_clahe_blur", 
            PreprocessKind::GrayOtsu => "gray_otsu"
        }
    }
} 


pub struct Preprocessor {
    kind: PreprocessKind, 
    clahe: Ptr<CLAHE>
} 

impl Preprocessor {
    pub fn new(kind: PreprocessKind) -> Result<Self, String> {
        let clahe = imgproc::create_clahe(2.0, Size::new(8, 8)).map_err(cv_err)?; 
        Ok(Self { kind, clahe })
    } 

    pub fn kind(&self) -> PreprocessKind {
        self.kind
    } 

    pub fn run(&mut self, src: &Mat) -> Result<Mat, String> {
        if self.kind == PreprocessKind::Raw {
            return src.try_clone().map_err(cv_err);
        } 

        // grayscale
        let gray = if src.channels() == 1 {
            src.try_clone().map_err(cv_err)?
        } else {
            let mut g = Mat::default(); 
            imgproc::cvt_color_def(src, &mut g, imgproc::COLOR_BGR2GRAY).map_err(cv_err)?; 
            g
        }; 

        if self.kind == PreprocessKind::Gray {
            return Ok(gray)
        } 

        // otsu
        if self.kind == PreprocessKind::GrayOtsu {
            let mut bin  = Mat::default(); 
            imgproc::threshold(&gray, &mut bin, 0.0, 255.0, imgproc::THRESH_BINARY | imgproc::THRESH_OTSU)
                .map_err(cv_err)?; 
            return Ok(bin);
        } 

        // clahe
        let mut eq = Mat::default();
        self.clahe.apply(&gray, &mut eq).map_err(cv_err)?; 
        if self.kind == PreprocessKind::GrayClahe {
            return Ok(eq)
        } 

        // blur
        let mut out = Mat::default(); 
        imgproc::gaussian_blur_def(&eq, &mut out, Size::new(3, 3), 0.0).map_err(cv_err)?; 
        Ok(out)

    }
}
