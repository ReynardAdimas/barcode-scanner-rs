use barcode_scanner_rs::detector::WeChatDetector;
use barcode_scanner_rs::model::ModelPaths; 

#[test]
fn detector_initializes() {
    let paths = ModelPaths::default();
    let result = WeChatDetector::new(&paths);
    assert!(result.is_ok(), "failed to initialize detector. error : {:?}", result.err())
}