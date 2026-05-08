use pyo3::prelude::*;

use crate::codec::TypeCodec;
use crate::error::CodecError;
use crate::types::{PyTypeNum, TypeNum};

/// NUMBER / BINARY_INTEGER 타입 codec
pub struct NumberCodec;

impl TypeCodec for NumberCodec {
    fn encode(&self, _py: Python<'_>, _value: &Bound<'_, PyAny>) -> Result<Vec<u8>, CodecError> {
        todo!("NUMBER encode: Python value → base-100 wire bytes")
    }

    fn decode(
        &self,
        _py: Python<'_>,
        _data: &[u8],
        target_type: PyTypeNum,
    ) -> Result<PyObject, CodecError> {
        match target_type {
            PyTypeNum::Int => todo!("NUMBER decode → PyInt"),
            PyTypeNum::Float => todo!("NUMBER decode → PyFloat"),
            PyTypeNum::Decimal => todo!("NUMBER decode → Decimal"),
            PyTypeNum::Str => todo!("NUMBER decode → PyStr"),
            _ => Err(CodecError::UnsupportedConversion {
                from: self.type_num(),
                to: target_type,
            }),
        }
    }

    fn type_num(&self) -> TypeNum {
        TypeNum::Number
    }

    fn default_python_type(&self) -> PyTypeNum {
        PyTypeNum::Int
    }
}
