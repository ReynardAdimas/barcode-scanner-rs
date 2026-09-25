use barcode_scanner_rs::camera::Camera; 
use barcode_scanner_rs::detector::{BarcodeDetector,WeChatDetector}; 
use barcode_scanner_rs::model::ModelPaths;
use opencv::highgui; 

fn main() -> Result<(), String> {
    let paths = ModelPaths::wechat_default();
    let mut detector = Box::new(WeChatDetector::new(&paths)?);
    let mut cam = Camera::open(0)?; 

    loop {
        let Some(frame) = cam.read_frame()? else {break}; 

//         let results = detector.detect(&frame); 
// 
//         for r in &results {
//             println!("Decoded: {}", r.data);
//         } 
            match detector.detect(&frame) {
            Ok(results) => {
                for r in &results {
                    println!("Decoded: {}", r.data);
                }
            }
            Err(e) => {
                return Err(e);
            }
        }
        highgui::imshow("frame", &frame).map_err(|e| e.to_string())?;
        highgui::wait_key(1).map_err(|e| e.to_string())?;
    }
    Ok(())
}