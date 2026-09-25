use super::{BarcodeDetector, DecodedBarcode}; 
use crate::model::ModelPaths; 
use opencv::core::Mat; 
use opencv::wechat_qrcode::WeChatQRCode; 

pub struct WeChatDetector {
    inner: WeChatQRCode,
} 

impl WeChatDetector {
    pub fn new(paths: &ModelPaths) -> Result<Self, String> {
        let inner = WeChatQRCode::new(
            &paths.detect_protoxt, 
            &paths.detect_caffemodel, 
            &paths.sr_prototxt, 
            &paths.sr_caffemodel
        )
        .map_err(|e| e.to_string())?;
        Ok(Self {inner})
    }
} 

impl BarcodeDetector for WeChatDetector {
    fn detect(&mut self, frame: &Mat) -> Result<Vec<DecodedBarcode>, String> {
        let mut points = opencv::core::Vector::<Mat>::new();
        let data = self
            .inner
            .detect_and_decode(frame, &mut points)
            .map_err(|e| e.to_string())?;
        Ok(data 
            .iter()
            .map(|d| DecodedBarcode {data: d.to_string(), points: vec![]})
            .collect())
    }
}