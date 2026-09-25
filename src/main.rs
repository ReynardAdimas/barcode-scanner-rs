use barcode_scanner_rs::camera::Camera; 
use barcode_scanner_rs::detector::wechat_qr::WeChatDetector;
use barcode_scanner_rs::detector::{BarcodeDetector,WechatQRDetector}; 
use barcode_scanner_rs::model::ModelPaths;
use opencv::highgui; 

fn main() -> Result<(), String> {
    let paths = ModelPaths::wechat_default();
    let mut detector = Box::new(WeChatDetector::new(&paths))?;
    let mut cam = Camera::open(0)?; 

    loop {
        let Some(frame) = cam.read_frame()? else {break}; 

        let results = detector.detect(&frame); 

        for r in &results {
            println!("Decoded: {}", r.data);
        } 

        highgui::imshow("frame", &frame)?;
        highgui::wait_key(1)?;
    }
    Ok(())
}