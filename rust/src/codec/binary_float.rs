use pyo3::prelude::*;

use crate::codec::TypeCodec;
use crate::error::CodecError;
use crate::types::{PyTypeNum, TypeNum};

/// BINARY_FLOAT 타입 codec (IEEE 754 + Oracle sign flip, 4 bytes)
pub struct BinaryFloatCodec;

impl BinaryFloatCodec {
    const WIRE_SIZE: usize = 4;
}

impl TypeCodec for BinaryFloatCodec {
    fn encode(&self, _py: Python<'_>, _value: &Bound<'_, PyAny>) -> Result<Vec<u8>, CodecError> {
        todo!("BINARY_FLOAT encode: f32 → 4-byte Oracle wire")
    }

    fn decode(
        &self,
        _py: Python<'_>,
        _data: &[u8],
        target_type: PyTypeNum,
    ) -> Result<PyObject, CodecError> {
        match target_type {
            PyTypeNum::Float => todo!("BINARY_FLOAT decode → PyFloat"),
            PyTypeNum::Str => todo!("BINARY_FLOAT decode → str"),
            PyTypeNum::Int => todo!("BINARY_FLOAT decode → int"),
            _ => Err(CodecError::UnsupportedConversion {
                from: self.type_num(),
                to: target_type,
            }),
        }
    }

    fn wire_size(&self) -> Option<usize> {
        Some(Self::WIRE_SIZE)
    }

    fn type_num(&self) -> TypeNum {
        TypeNum::Float
    }

    fn default_python_type(&self) -> PyTypeNum {
        PyTypeNum::Float
    }
}

/// BINARY_DOUBLE 타입 codec (IEEE 754 + Oracle sign flip, 8 bytes)
pub struct BinaryDoubleCodec;

impl BinaryDoubleCodec {
    const WIRE_SIZE: usize = 8;
}

impl TypeCodec for BinaryDoubleCodec {
    fn encode(&self, _py: Python<'_>, _value: &Bound<'_, PyAny>) -> Result<Vec<u8>, CodecError> {
        todo!("BINARY_DOUBLE encode: f64 → 8-byte Oracle wire")
    }

    fn decode(
        &self,
        _py: Python<'_>,
        _data: &[u8],
        target_type: PyTypeNum,
    ) -> Result<PyObject, CodecError> {
        match target_type {
            PyTypeNum::Float => todo!("BINARY_DOUBLE decode → PyFloat"),
            PyTypeNum::Str => todo!("BINARY_DOUBLE decode → str"),
            PyTypeNum::Int => todo!("BINARY_DOUBLE decode → int"),
            _ => Err(CodecError::UnsupportedConversion {
                from: self.type_num(),
                to: target_type,
            }),
        }
    }

    fn wire_size(&self) -> Option<usize> {
        Some(Self::WIRE_SIZE)
    }

    fn type_num(&self) -> TypeNum {
        TypeNum::Double
    }

    fn default_python_type(&self) -> PyTypeNum {
        PyTypeNum::Float
    }
}
