use pyo3::prelude::*;

use crate::codec::TypeCodec;
use crate::error::CodecError;
use crate::types::{PyTypeNum, TypeNum};

/// Tibero JSON 타입 codec
///
/// Oracle의 OSON과 포맷이 다르므로 별도 구현체를 사용합니다.
pub struct TbJsonCodec;

impl TypeCodec for TbJsonCodec {
    fn encode(&self, _py: Python<'_>, _value: &Bound<'_, PyAny>) -> Result<Vec<u8>, CodecError> {
        todo!("JSON encode: Python dict/list → Tibero JSON wire")
    }

    fn decode(
        &self,
        _py: Python<'_>,
        _data: &[u8],
        target_type: PyTypeNum,
    ) -> Result<PyObject, CodecError> {
        match target_type {
            PyTypeNum::Str => todo!("JSON decode → Python dict/list/str"),
            _ => Err(CodecError::UnsupportedConversion {
                from: self.type_num(),
                to: target_type,
            }),
        }
    }

    fn type_num(&self) -> TypeNum {
        TypeNum::Json
    }

    fn default_python_type(&self) -> PyTypeNum {
        PyTypeNum::Str
    }
}
