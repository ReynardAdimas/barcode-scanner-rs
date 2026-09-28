use barcode_scanner_rs::detector::{BarcodeDetector, PipelineDetector, RxingDetector, StandardQrDetector, WeChatDetector}; 
use barcode_scanner_rs::model::ModelPaths; 
use opencv::{core::Mat, imgcodecs, prelude::*}; 
use std::path::PathBuf; 

fn image_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR",)).join("test-images")
} 

fn image_folders() -> Vec<String> {
    let mut folders: Vec<String> = std::fs::read_dir(image_root())
        .expect("Test-Images Folder Not Found")
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir())
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect(); 

    folders.sort();
    folders
} 

fn load_image(folder: &str) -> Mat {
    let path = image_root().join(folder).join("barcode.jpg"); 
    let img = imgcodecs::imread(path.to_str().unwrap(), imgcodecs::IMREAD_COLOR)
        .expect("Imread Failed"); 
    assert!(!img.empty(), "Empty File at {:?}", path);
    img
} 

fn all_detector() -> Vec<(&'static str, Box<dyn BarcodeDetector>)> {
    let paths = ModelPaths::wechat_default(); 
    vec![(
        "standard", 
        Box::new(StandardQrDetector::new().expect("Initialization StandardQrDetector Failed"))
    ), 
    (
        "wechat", 
        Box::new(WeChatDetector::new(&paths).expect("Initialization WeChatDetector Failed"))
    ), 
    (
        "hybrid", 
        Box::new(PipelineDetector::new(vec![
            Box::new(StandardQrDetector::new().unwrap()), 
            Box::new(WeChatDetector::new(&paths).unwrap()), 
            Box::new(RxingDetector::new())
        ]))
    )
    ]
} 

#[test]
fn all_detector_on_all_images() {
    let folders = image_folders(); 
    assert_eq!(folders.len(), 15, "Length of folder not suit: {:?}", folders); 

    let mut detectors = all_detector(); 

    println!("\n{:<14} {:<10} {:<6} {}", "Image", "detector", "status", "result");
    println!("{}", "-".repeat(60)); 

    for folder in &folders {
        let img = load_image(folder); 
        for (name, detector) in detectors.iter_mut() {
            match detector.detect(&img) {
                Ok(res) if !res.is_empty() => {
                    let data: Vec<&String> = res.iter().map(|r| &r.data).collect(); 
                    println!("{:<14} {:<10} {:<6} {:?}", folder, name, "OK", data);
                } 
                Ok(_) => println!("{:<14} {:<10} {:<6} -", folder, name, "MISS"),
                Err(e) => println!("{:<14} {:<10} {:<6} {}", folder, name, "ERR", e),
            }
        }
    }

} 

#[test]
fn qrcode_test() {
    let img = load_image("qrcode"); 
    for (name, mut detector) in all_detector() {
        let res = detector
            .detect(&img)
            .unwrap_or_else(|e| panic!("[{name}] detect() error: {e}"));
        assert!(!res.is_empty(), "[{name}] QR code tidak terdeteksi");
    }
}


