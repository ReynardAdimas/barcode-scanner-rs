pub mod wechat_qr; 
pub mod standard_qr; 

use opencv::core::Mat;

pub trait BarcodeDetector {
    fn detect(&mut self, frame: &Mat) -> Result<Vec<DecodedBarcode>, String>; 
}

#[derive(Debug, Clone)]
pub struct DecodedBarcode {
    pub data: String, 
    pub points: Vec<opencv::core::Point>
}