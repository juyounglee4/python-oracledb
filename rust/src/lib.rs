pub mod buffer;
pub mod codec;
pub mod error;
pub mod registry;
pub mod types;

use pyo3::prelude::*;

/// Tibero Python driver – Rust data conversion core
#[pymodule]
fn _rust_impl(_py: Python<'_>, _m: &Bound<'_, PyModule>) -> PyResult<()> {
    todo!("PyO3 모듈 초기화: Python 바인딩 등록")
}