use super::{BarcodeDetector, DecodedBarcode};
use image::{DynamicImage, GrayImage};
use opencv::{core::Mat, imgproc, prelude::*};
use ::rxing::{
    common::HybridBinarizer,
    multi::{GenericMultipleBarcodeReader, MultipleBarcodeReader},
    BinaryBitmap, BufferedImageLuminanceSource, DecodeHints, Exceptions,
    MultiUseMultiFormatReader,
};
pub struct RxingDetector; 

impl RxingDetector {
    pub fn new() -> Self {
        Self
    }
} 


fn mat_to_img(frame: &Mat) -> Result<DynamicImage, String> {
    let mut gray = Mat::default(); 
    if frame.channels() == 1 {
        gray = frame.try_clone().map_err(|e| e.to_string())?; 
    } else {
        imgproc::cvt_color_def(frame, &mut gray, imgproc::COLOR_BGR2GRAY)
            .map_err(|e| e.to_string())?;
    } 

    let (w,h) = (gray.cols() as u32, gray.rows() as u32); 

    let bytes = gray.data_bytes().map_err(|e| e.to_string())?;

    let buf = GrayImage::from_raw(w,h,bytes.to_vec())
        .ok_or_else(|| "Buffer Size image not match".to_string())?;

    Ok(DynamicImage::ImageLuma8(buf))
}

impl BarcodeDetector for RxingDetector {
    fn detect(&mut self, frame: &Mat) -> Result<Vec<DecodedBarcode>, String> {
        let img = mat_to_img(frame)?; 
        let mut hints = DecodeHints::default();
        hints.TryHarder = Some(true); 

        let mut scanner = GenericMultipleBarcodeReader::new(MultiUseMultiFormatReader::default()); 
        let mut bitmap = BinaryBitmap::new(HybridBinarizer::new(BufferedImageLuminanceSource::new(img))); 

        match scanner.decode_multiple_with_hints(&mut bitmap, &hints) {
            Ok(results) => Ok(results
                .iter()
                .map(|r| DecodedBarcode {
                    data: r.getText().to_string(),
                    points: vec![],
                })
                .collect()),
            Err(Exceptions::NotFoundException(_)) => Ok(vec![]),
            Err(e) => Err(e.to_string()),
        }

    }
}
