use pyo3::prelude::*;

use crate::codec::TypeCodec;
use crate::error::CodecError;
use crate::types::{PyTypeNum, TypeNum};

/// INTERVAL DAY TO SECOND 타입 codec (11 bytes)
pub struct IntervalDsCodec;

impl IntervalDsCodec {
    const WIRE_SIZE: usize = 12;
}

impl TypeCodec for IntervalDsCodec {
    fn encode(&self, _py: Python<'_>, _value: &Bound<'_, PyAny>) -> Result<Vec<u8>, CodecError> {
        todo!("INTERVAL_DS encode: Python timedelta → 11-byte wire")
    }

    fn decode(
        &self,
        _py: Python<'_>,
        _data: &[u8],
        target_type: PyTypeNum,
    ) -> Result<PyObject, CodecError> {
        match target_type {
            PyTypeNum::Timedelta => todo!("INTERVAL_DS decode → timedelta"),
            PyTypeNum::Str => todo!("INTERVAL_DS decode → str"),
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
        TypeNum::IntervalDs
    }

    fn default_python_type(&self) -> PyTypeNum {
        PyTypeNum::Timedelta
    }
}

/// INTERVAL YEAR TO MONTH 타입 codec (5 bytes)
pub struct IntervalYmCodec;

impl IntervalYmCodec {
    const WIRE_SIZE: usize = 5;
}

impl TypeCodec for IntervalYmCodec {
    fn encode(&self, _py: Python<'_>, _value: &Bound<'_, PyAny>) -> Result<Vec<u8>, CodecError> {
        todo!("INTERVAL_YM encode: Python IntervalYM → 5-byte wire")
    }

    fn decode(
        &self,
        _py: Python<'_>,
        _data: &[u8],
        target_type: PyTypeNum,
    ) -> Result<PyObject, CodecError> {
        match target_type {
            PyTypeNum::IntervalYm => todo!("INTERVAL_YM decode → IntervalYM"),
            PyTypeNum::Str => todo!("INTERVAL_YM decode → str"),
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
        TypeNum::IntervalYm
    }

    fn default_python_type(&self) -> PyTypeNum {
        PyTypeNum::IntervalYm
    }
}
