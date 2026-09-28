use super::{BarcodeDetector, DecodedBarcode}; 
use crate::utils::cv_err; 
use opencv::objdetect::BarcodeDetector as CvBarcodeDetector;
use opencv::core::{Mat, Vector}; 
use opencv::prelude::*; 

pub struct OpenCvBarcodeDetector {
    inner:CvBarcodeDetector
}

impl OpenCvBarcodeDetector {
    pub fn new() -> Result<Self, String> {
        let inner = CvBarcodeDetector::default().map_err(cv_err)?; 
        Ok(Self {inner})
    }
}

impl BarcodeDetector for OpenCvBarcodeDetector {
    fn detect(&mut self, frame: &Mat) -> Result<Vec<DecodedBarcode>, String> {
        let mut infos: Vector<String> = Vector::new(); 
        let mut types: Vector<String> = Vector::new(); 
        let mut points = Mat::default(); 
        self.inner
            .detect_and_decode_with_type(frame, &mut infos, &mut types, &mut points)
            .map_err(cv_err)?; 

        Ok(infos
            .iter()
            .filter(|s| !s.is_empty())
            .map(|s| DecodedBarcode {data: s, points: vec![]})
            .collect())
    }
}