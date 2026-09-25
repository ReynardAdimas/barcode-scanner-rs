use super::{BarcodeDetector, DecodedBarcode};
use opencv::core::Mat; 
use opencv::objdetect::QRCodeDetector;

pub struct StandardQRDetector {
    inner: QRCodeDetector,
} 

impl StandardQRDetector {
    pub fn new() -> Result<Self, String> {
        let inner = QRCodeDetector::default().map_err(|e| e.to_string())?;
        Ok(Self { inner })
    } 
} 

impl BarcodeDetector for StandardQRDetector {
    fn detect(&mut self, frame: &Mat) -> Result<Vec<DecodedBarcode>, String> {
        let mut points = Mat::default();
        let data = self.inner.detect_and_decode(frame, &mut points).map_err(|e| e.to_string())?;

        if data.is_empty() {
            Ok(vec![])
        } else {
            Ok(vec![DecodedBarcode {data, points:vec![]}])
        }
    }
}
