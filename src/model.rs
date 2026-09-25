pub struct ModelPaths {
    pub detect_protoxt: String, 
    pub detect_caffemodel: String, 
    pub sr_protoxt: String,
    pub sr_caffemodel: String,
} 

impl ModelPaths {
    pub fn wechat_default() -> Self {
        Self {
            detect_protoxt: "./model/detect.prototxt".into(),
            detect_caffemodel: "./model/detect.caffemodel".into(),
            sr_protoxt: "./model/sr.prototxt".into(),
            sr_caffemodel: "./model/sr.caffemodel".into(),
        }
    }
}