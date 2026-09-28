use super::{BarcodeDetector, DecodedBarcode}; 
use crate::preprocess::{PreprocessKind, Preprocessor}; 
use opencv::core::Mat; 

pub struct PreprocessedDetector {
    pre: Preprocessor, 
    inner : Box<dyn BarcodeDetector>
} 

impl PreprocessedDetector {
    pub fn new(kind: PreprocessKind, inner : Box<dyn BarcodeDetector>) -> Result<Self, String> {
        Ok(Self {pre: Preprocessor::new(kind)?, inner})
    }
}

impl BarcodeDetector for PreprocessedDetector {
    fn detect(&mut self, frame: &Mat) -> Result<Vec<DecodedBarcode>, String> {
        let processed = self.pre.run(frame)?; 
        self.inner.detect(&processed)
    }
}