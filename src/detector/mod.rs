pub mod wechat_qr; 
pub mod standard_qr; 

pub use wechat_qr::WechatQRDetector; 

use opencv::core::Mat; 

pub struct DecodedBarcode {
    pub data: String, 
    pub points: Vec<(f32, f32)>,
} 

pub trait BarcodeDetector {
    fn detect(&self, image: &Mat) -> Result<Vec<DecodedBarcode>, String>;
}