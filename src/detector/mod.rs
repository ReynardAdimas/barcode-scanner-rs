mod wechat_qr; 
mod standard_qr; 

pub use standard_qr::StandardQrDetector;
pub use wechat_qr::WeChatDetector;

use opencv::core::Mat; 

pub struct DecodedBarcode {
    pub data: String, 
    pub points: Vec<(f32, f32)>,
} 

pub trait BarcodeDetector {
    fn detect(&mut self, frame: &Mat) -> Result<Vec<DecodedBarcode>, String>;
}