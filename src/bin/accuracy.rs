use std::{collections::BTreeMap, path::PathBuf, time::Instant};

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
    wrong: usize, 
    ms: Vec<f64>
} 

fn percentile(sorted: &[f64],p: f64) -> f64 {
    if sorted.is_empty() {
        return 0.0
    } 

    sorted[((sorted.len()-1) as f64 *p).round() as usize]
} 

fn judge(expected: &[String], got: &[DecodedBarcode]) -> (bool, bool) {
    if expected.is_empty() {
        return (true, false);
    }
    let texts: Vec<&str> = got.iter().map(|d| d.data.as_str()).collect();
    let ok = expected.iter().all(|e| texts.contains(&e.as_str()));
    let wrong = texts.iter().any(|t| !expected.iter().any(|e| e==t));
    (ok, wrong)
} 

fn main() -> Result<(), String> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test-images"); 
    let cases = load_cases(&root, &standard_conditions())?;
    let mut dets = set(&ModelPaths::wechat_default())?; 
    let det_names: Vec<&str> = dets.iter().map(|(n, _) | *n).collect();

    if let Some(c) = cases.first() {
        for (_, d) in dets.iter_mut() {
            let _ = d.detect(&c.img);
        }
    } 

    let mut stats: BTreeMap<(String, String), Stat> = BTreeMap::new();
    let mut coverage: BTreeMap<(String,String), &'static str> = BTreeMap::new(); 
    let mut preview: BTreeMap<(String, String), String> = BTreeMap::new(); 

    for kind in PreprocessKind::ALL {
        let mut pre = Preprocessor::new(kind)?; 
        for case in &cases {
            let img = pre.run(&case.img)?; 
            for(name, det) in dets.iter_mut() {
                let t0 = Instant::now();
                let res = det.detect(&img);
                let dt = t0.elapsed().as_secs_f64() * 1000.0; 

                let st = stats.entry((kind.label().to_string(), name.to_string())).or_default();
                st.n += 1; 
                st.ms.push(dt); 

                let verdict: &'static str = match &res {
                    Err(_) => {
                        st.err += 1; 
                        "ERR"
                    }
                    Ok(r) if r.is_empty() => {
                        st.miss += 1; 
                        "MISS"
                    }
                    Ok(r) => {
                        let (ok, wrong) = judge(&case.expected, r); 
                        if ok {
                            st.ok += 1;
                        } 
                        if wrong {
                            st.wrong += 1; 
                        }
                        if ok && !wrong {"OK"} else if wrong {"WRONG"} else {"MISS"}
                    }
                }; 

                if kind == PreprocessKind::Raw && case.condition == "clean" {
                    coverage.insert((case.folder.clone(), name.to_string()), verdict); 
                    if let Ok(r) = &res {
                        if let Some(first) = r.first() {
                            preview.insert((case.folder.clone(), name.to_string()), first.data.clone());
                        }
                    }
                }
            }
        }
    }

    println!("\nCoverage Matrix "); 
    print!("{:<14}", "folder"); 
    for n in &det_names {
        print!("{:<10}", n);
    }
    println!();
    let mut folders: Vec<&String> = coverage.keys().map(|(f, _)| f).collect(); 
    folders.dedup();
    for f in &folders {
        print!("{:<14}", f); 
        for n in &det_names {
            let v = coverage.get(&((*f).clone(), n.to_string())).copied().unwrap_or("-"); 
            print!("{:<10}", v);
        }
        println!();
    }
    Ok(())
}
