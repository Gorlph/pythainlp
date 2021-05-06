


pub mod tokenizer;
pub mod fixed_bytes_str;

use crate::tokenizer::newmm_custom::Newmm;


use pyo3::wrap_pyfunction;
use pyo3::prelude::*;
use tokenizer::tokenizer_trait::Tokenizer;
const CARGO_PATH:&str = env!("CARGO_MANIFEST_DIR");

#[pyfunction]
fn segment(text:&str,safe:Option<bool>) -> PyResult<Vec<String>> {
    
    let newmm =  Newmm::new(None);
    let result = newmm.segment(text, safe);
    Ok(result)
}

/// A Python module implemented in Rust.
#[pymodule]
fn oxidized_thainlp(_py: Python, m: &PyModule) -> PyResult<()> {


    m.add_function(wrap_pyfunction!(segment, m)?)?;

    Ok(())
}