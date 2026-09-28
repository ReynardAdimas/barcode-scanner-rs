use crate::augment::{apply, Condition};
use crate::utils::cv_err;
use opencv::{core::Mat, imgcodecs, prelude::*};
use std::{fs, path::Path};

pub struct Case {
    pub folder: String,
    pub condition: String,
    pub expected: Vec<String>,
    pub img: Mat,
}

pub fn load_cases(root: &Path, conds: &[Condition]) -> Result<Vec<Case>, String> {
    let mut folders: Vec<_> = fs::read_dir(root)
        .map_err(|e| format!("{root:?}: {e}"))?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir())
        .collect();
    folders.sort_by_key(|e| e.file_name());

    let mut out = Vec::new();
    for f in folders {
        let name = f.file_name().to_string_lossy().into_owned();

        let img_path = f.path().join("barcode.jpg");
        let base = imgcodecs::imread(img_path.to_str().unwrap(), imgcodecs::IMREAD_COLOR)
            .map_err(cv_err)?;
        if base.empty() {
            return Err(format!("gagal membaca {img_path:?}"));
        }

        let expected: Vec<String> = fs::read_to_string(f.path().join("expected.txt"))
            .map(|s| {
                s.lines()
                    .map(str::trim)
                    .filter(|l| !l.is_empty())
                    .map(String::from)
                    .collect()
            })
            .unwrap_or_default();

        for c in conds {
            out.push(Case {
                folder: name.clone(),
                condition: c.name.to_string(),
                expected: expected.clone(),
                img: apply(&base, &c.ops)?,
            });
        }
    }
    Ok(out)
}