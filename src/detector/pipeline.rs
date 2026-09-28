use super::{BarcodeDetector, DecodedBarcode}; 
use opencv::core::Mat; 

pub struct PipelineDetector {
    stages: Vec<Box<dyn BarcodeDetector>>
} 

impl PipelineDetector {
    pub fn new(stages: Vec<Box<dyn BarcodeDetector>>) -> Self {
        Self { stages }
    } 
} 

impl BarcodeDetector for PipelineDetector {
    fn detect(&mut self, frame: &Mat) -> Result<Vec<DecodedBarcode>, String> {
        for stage in self.stages.iter_mut() {
            match stage.detect(frame) {
                Ok(res) if !res.is_empty() => return Ok(res), 
                Ok(_) => continue, 
                Err(e) => eprintln!("[pipeline] error, continue: {e}")
            }
        } 
        Ok(vec![])
    }
}