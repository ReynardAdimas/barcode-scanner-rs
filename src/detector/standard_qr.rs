use super::{BarcodeDetector, DecodedBarcode};
use opencv::core::Mat; 
use opencv::objdetect::QRCodeDetector;
use opencv::prelude::*;



pub struct StandardQrDetector {
    inner: QRCodeDetector,
} 

impl StandardQrDetector {
    pub fn new() -> Result<Self, String> {
        let inner = QRCodeDetector::default().map_err(|e| e.to_string())?;
        Ok(Self { inner })
    } 
} 

impl BarcodeDetector for StandardQrDetector {
    fn detect(&mut self, frame: &Mat) -> Result<Vec<DecodedBarcode>, String> {
        let mut points = Mat::default();
        let mut straight_qrcode = Mat::default();
        let data = self.inner.detect_and_decode(frame, &mut points, &mut straight_qrcode).map_err(|e| e.to_string())?;

        if data.is_empty() {
            Ok(vec![])
        } else {
            let decoded = String::from_utf8_lossy(&data).into_owned();
            Ok(vec![DecodedBarcode {data: decoded, points:vec![]}])
        }
    }
}
