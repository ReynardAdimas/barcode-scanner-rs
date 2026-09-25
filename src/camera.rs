use opencv::core::Mat; 
use opencv::prelude::*;
use opencv::videoio::{self, VideoCaptureTrait}; 

pub struct Camera {
    cap: videoio::VideoCapture,
} 

impl Camera {
    pub fn open(index: i32) -> Result<Self, String> {
        let cap = videoio::VideoCapture::new(index, videoio::CAP_ANY)
            .map_err(|e| e.to_string())?;
        Ok(Self { cap })
    }

    pub fn read_frame(&mut self) -> Result<Option<Mat>, String> {
        let mut frame = Mat::default();
        self.cap.read(&mut frame)
            .map_err(|e| e.to_string())?;
        if frame.empty() {
            Ok(None)
        } else {
            Ok(Some(frame))
        }

    }
}