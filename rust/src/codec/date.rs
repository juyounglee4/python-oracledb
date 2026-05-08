use pyo3::prelude::*;

use crate::codec::TypeCodec;
use crate::error::CodecError;
use crate::types::{PyTypeNum, TypeNum};

/// DATE/TIMESTAMP/TIMESTAMP_TZ 타입 codec
///
/// 하나의 구현체가 DATE(7 bytes), TIMESTAMP(11 bytes),
/// TIMESTAMP WITH TZ(13 bytes) 모두 처리합니다.
pub struct DateCodec {
    include_fsecond: bool,
    include_tz: bool,
    type_num: TypeNum,
}

impl DateCodec {
    const DATE_WIRE_SIZE: usize = 8;
    const TIMESTAMP_WIRE_SIZE: usize = 12;
    const TIMESTAMP_LTZ_WIRE_SIZE: usize = 12;
    const TIMESTAMP_TZ_WIRE_SIZE: usize = 17;

    pub fn date() -> Self {
        Self {
            include_fsecond: false,
            include_tz: false,
            type_num: TypeNum::Date,
        }
    }

    pub fn timestamp() -> Self {
        Self {
            include_fsecond: true,
            include_tz: false,
            type_num: TypeNum::Timestamp,
        }
    }

    pub fn timestamp_tz() -> Self {
        Self {
            include_fsecond: true,
            include_tz: true,
            type_num: TypeNum::TimestampTz,
        }
    }

    pub fn timestamp_ltz() -> Self {
        Self {
            include_fsecond: true,
            include_tz: true,
            type_num: TypeNum::TimestampLtz,
        }
    }
}

impl TypeCodec for DateCodec {
    fn encode(&self, _py: Python<'_>, _value: &Bound<'_, PyAny>) -> Result<Vec<u8>, CodecError> {
        todo!("DATE/TIMESTAMP encode: Python datetime → wire bytes")
    }

    fn decode(
        &self,
        _py: Python<'_>,
        _data: &[u8],
        target_type: PyTypeNum,
    ) -> Result<PyObject, CodecError> {
        match target_type {
            PyTypeNum::Datetime => todo!("DATE decode → datetime"),
            PyTypeNum::Str => todo!("DATE decode → str"),
            _ => Err(CodecError::UnsupportedConversion {
                from: self.type_num(),
                to: target_type,
            }),
        }
    }

    fn wire_size(&self) -> Option<usize> {
        Some(if self.include_tz {
            Self::TIMESTAMP_TZ_WIRE_SIZE
        } else if self.include_fsecond {
            Self::TIMESTAMP_WIRE_SIZE
        } else {
            Self::DATE_WIRE_SIZE
        })
    }

    fn type_num(&self) -> TypeNum {
        self.type_num
    }

    fn default_python_type(&self) -> PyTypeNum {
        PyTypeNum::Datetime
    }
}


