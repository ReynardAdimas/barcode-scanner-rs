mod wechat_qr; 
mod standard_qr; 
mod rxing_detector;
mod pipeline; 
mod opencv_barcode;
mod preprocessed;

use std::path;

pub use standard_qr::StandardQrDetector;
pub use wechat_qr::WeChatDetector;
pub use rxing_detector::RxingDetector; 
pub use pipeline::PipelineDetector;
pub use opencv_barcode::OpenCvBarcodeDetector;
pub use preprocessed::PreprocessedDetector; 

use crate::model::ModelPaths;
use opencv::core::Mat; 

pub struct DecodedBarcode { 
    pub data: String, 
    pub points: Vec<(f32, f32)>,
} 

pub trait BarcodeDetector {
    fn detect(&mut self, frame: &Mat) -> Result<Vec<DecodedBarcode>, String>;
}

pub type NamedDetector = (&'static str, Box<dyn BarcodeDetector>); 

fn named<D: BarcodeDetector + 'static>(name: &'static str, d:D) -> NamedDetector {
    (name, Box::new(d))
} 

pub fn set(paths: &ModelPaths) -> Result<Vec<NamedDetector>, String> {
    Ok(vec![
        named("opencv-qr", StandardQrDetector::new()?), 
        named("opencv-id", OpenCvBarcodeDetector::new()?), 
        named("wechat", WeChatDetector::new(paths)?), 
        named("rxing", RxingDetector::new()), 
        named("hybrid", 
            PipelineDetector::new(vec![
                Box::new(StandardQrDetector::new()?), 
                Box::new(WeChatDetector::new(paths)?), 
                Box::new(RxingDetector::new())
            ])
        ), 
        named(
            "hybrid+1d", 
            PipelineDetector::new(vec![
                Box::new(StandardQrDetector::new()?), 
                Box::new(OpenCvBarcodeDetector::new()?), 
                Box::new(WeChatDetector::new(paths)?),
                Box::new(RxingDetector::new())
            ])
        )
    ])
}