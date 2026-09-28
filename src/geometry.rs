use opencv::{core::Mat, prelude::*}; 

pub type Polygon = Vec<(f32, f32)>; 

pub fn mat_to_polygon(m: &Mat) -> Result<Vec<Polygon>, String> {
    if m.empty() {
        return Ok(vec![]);
    } 
    let flat = if m.channels() == 1 {
        m.try_clone().map_err(|e| e.to_string())?
    } else {
        m.reshape(1, 0) 
            .map_err(op)
    }
}