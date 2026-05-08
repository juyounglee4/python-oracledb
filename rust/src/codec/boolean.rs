use pyo3::prelude::*;

use crate::codec::TypeCodec;
use crate::error::CodecError;
use crate::types::{PyTypeNum, TypeNum};

/// BOOLEAN 타입 codec
pub struct BooleanCodec;

impl TypeCodec for BooleanCodec {
    fn encode(&self, _py: Python<'_>, _value: &Bound<'_, PyAny>) -> Result<Vec<u8>, CodecError> {
        todo!("BOOLEAN encode: Python bool → wire bytes")
    }

    fn decode(
        &self,
        _py: Python<'_>,
        _data: &[u8],
        target_type: PyTypeNum,
    ) -> Result<PyObject, CodecError> {
        match target_type {
            PyTypeNum::Bool => todo!("BOOLEAN decode → bool"),
            _ => Err(CodecError::UnsupportedConversion {
                from: self.type_num(),
                to: target_type,
            }),
        }
    }

    fn type_num(&self) -> TypeNum {
        TypeNum::Boolean
    }

    fn default_python_type(&self) -> PyTypeNum {
        PyTypeNum::Bool
    }
}
