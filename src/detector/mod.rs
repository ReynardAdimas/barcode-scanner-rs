mod wechat_qr; 
mod standard_qr; 
mod rxing_detector;
mod pipeline; 



pub use standard_qr::StandardQrDetector;
pub use wechat_qr::WeChatDetector;
pub use rxing_detector::RxingDetector; 
pub use pipeline::PipelineDetector;

use opencv::core::Mat; 

pub struct DecodedBarcode {
    pub data: String, 
    pub points: Vec<(f32, f32)>,
} 

pub trait BarcodeDetector {
    fn detect(&mut self, frame: &Mat) -> Result<Vec<DecodedBarcode>, String>;
}