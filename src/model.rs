pub struct ModelPaths {
    pub detect_protoxt: String, 
    pub detect_caffemodel: String, 
    pub sr_prototxt: String,
    pub sr_caffemodel: String,
} 

impl ModelPaths {
    pub fn wechat_default() -> Self {
        Self {
            detect_protoxt: "./models/detect.prototxt".into(),
            detect_caffemodel: "./models/detect.caffemodel".into(),
            sr_prototxt: "./models/sr.prototxt".into(),
            sr_caffemodel: "./models/sr.caffemodel".into(),
        }
    }
}