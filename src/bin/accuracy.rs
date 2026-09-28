use barcode_scanner_rs::{
    augment::standard_conditions, 
    dataset::load_cases,
    detector::{set, DecodedBarcode}, 
    model::ModelPaths,
    preprocess::{PreprocessKind, Preprocessor}
}; 

#[derive(Default)]
struct Stat {
    n: usize, 
    ok: usize, 
    miss: usize, 
    err: usize, 
    err: usize, 
    wrong: usize, 
    ms: Vec<f64>
} 

