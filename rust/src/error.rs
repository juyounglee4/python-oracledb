use pyo3::prelude::*;
use pyo3::exceptions;

use crate::types::{PyTypeNum, TypeNum};

/// Data conversion 에러 타입
#[derive(Debug)]
pub enum CodecError {
    /// encode/decode 중 데이터 형식 오류
    InvalidData { type_num: TypeNum, detail: String },

    /// 지원하지 않는 타입
    UnsupportedType(TypeNum),

    /// 지원하지 않는 변환 (예: NUMBER → Timedelta)
    UnsupportedConversion { from: TypeNum, to: PyTypeNum },

    /// 값 범위 초과 (예: NUMBER 40자리 초과)
    Overflow { type_num: TypeNum, detail: String },

    /// Wire bytes 부족 (truncated)
    InsufficientData { expected: usize, actual: usize },

    /// PyO3 변환 오류
    Python(PyErr),
}

impl From<PyErr> for CodecError {
    fn from(err: PyErr) -> Self {
        CodecError::Python(err)
    }
}

impl From<CodecError> for PyErr {
    fn from(err: CodecError) -> Self {
        match err {
            CodecError::InvalidData { detail, .. } => {
                exceptions::PyValueError::new_err(detail)
            }
            CodecError::UnsupportedType(t) => {
                exceptions::PyNotImplementedError::new_err(format!(
                    "Unsupported DB type: {:?}",
                    t
                ))
            }
            CodecError::UnsupportedConversion { from, to } => {
                exceptions::PyTypeError::new_err(format!(
                    "Cannot convert {:?} to {:?}",
                    from, to
                ))
            }
            CodecError::Overflow { detail, .. } => {
                exceptions::PyOverflowError::new_err(detail)
            }
            CodecError::InsufficientData { expected, actual } => {
                exceptions::PyValueError::new_err(format!(
                    "Expected {} bytes, got {}",
                    expected, actual
                ))
            }
            CodecError::Python(e) => e,
        }
    }
}
