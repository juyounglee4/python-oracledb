use pyo3::prelude::*;

use crate::codec::TypeCodec;
use crate::error::CodecError;
use crate::types::{PyTypeNum, TypeNum};

/// VARCHAR / CHAR / LONG 타입 codec (가변 길이 문자열)
pub struct VarcharCodec {
    type_num: TypeNum,
}

impl VarcharCodec {
    pub fn varchar() -> Self {
        Self {
            type_num: TypeNum::Varchar,
        }
    }

    pub fn char() -> Self {
        Self {
            type_num: TypeNum::Char,
        }
    }

    pub fn long() -> Self {
        Self {
            type_num: TypeNum::Long,
        }
    }
}

impl TypeCodec for VarcharCodec {
    fn encode(&self, _py: Python<'_>, _value: &Bound<'_, PyAny>) -> Result<Vec<u8>, CodecError> {
        todo!("VARCHAR encode: Python str → UTF-8 bytes")
    }

    fn decode(
        &self,
        _py: Python<'_>,
        _data: &[u8],
        target_type: PyTypeNum,
    ) -> Result<PyObject, CodecError> {
        match target_type {
            PyTypeNum::Str => todo!("VARCHAR decode → str"),
            PyTypeNum::Bytes => todo!("VARCHAR decode → bytes"),
            _ => Err(CodecError::UnsupportedConversion {
                from: self.type_num(),
                to: target_type,
            }),
        }
    }

    fn type_num(&self) -> TypeNum {
        self.type_num
    }

    fn default_python_type(&self) -> PyTypeNum {
        PyTypeNum::Str
    }
}

/// RAW / LONG RAW 타입 codec (가변 길이 바이너리)
pub struct RawCodec {
    type_num: TypeNum,
}

impl RawCodec {
    pub fn raw() -> Self {
        Self {
            type_num: TypeNum::Raw,
        }
    }

    pub fn long_raw() -> Self {
        Self {
            type_num: TypeNum::LongRaw,
        }
    }
}

impl TypeCodec for RawCodec {
    fn encode(&self, _py: Python<'_>, _value: &Bound<'_, PyAny>) -> Result<Vec<u8>, CodecError> {
        todo!("RAW encode: Python bytes → raw wire bytes")
    }

    fn decode(
        &self,
        _py: Python<'_>,
        _data: &[u8],
        target_type: PyTypeNum,
    ) -> Result<PyObject, CodecError> {
        match target_type {
            PyTypeNum::Bytes => todo!("RAW decode → bytes"),
            _ => Err(CodecError::UnsupportedConversion {
                from: self.type_num(),
                to: target_type,
            }),
        }
    }

    fn type_num(&self) -> TypeNum {
        self.type_num
    }

    fn default_python_type(&self) -> PyTypeNum {
        PyTypeNum::Bytes
    }
}
