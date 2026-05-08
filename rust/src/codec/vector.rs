use pyo3::prelude::*;

use crate::codec::TypeCodec;
use crate::error::CodecError;
use crate::types::{PyTypeNum, TypeNum};

/// Tibero VECTOR 타입 codec
///
/// Oracle의 VECTOR와 인코딩 방식이 다르므로 별도 구현체를 사용합니다.
pub struct TbVectorCodec;

impl TypeCodec for TbVectorCodec {
    fn encode(&self, _py: Python<'_>, _value: &Bound<'_, PyAny>) -> Result<Vec<u8>, CodecError> {
        todo!("VECTOR encode: Python array → Tibero VECTOR wire")
    }

    fn decode(
        &self,
        _py: Python<'_>,
        _data: &[u8],
        target_type: PyTypeNum,
    ) -> Result<PyObject, CodecError> {
        match target_type {
            PyTypeNum::Bytes => todo!("VECTOR decode → Python array"),
            _ => Err(CodecError::UnsupportedConversion {
                from: self.type_num(),
                to: target_type,
            }),
        }
    }

    fn type_num(&self) -> TypeNum {
        TypeNum::Vector
    }

    fn default_python_type(&self) -> PyTypeNum {
        PyTypeNum::Bytes
    }
}
