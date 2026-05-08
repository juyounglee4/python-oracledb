pub mod binary_float;
pub mod boolean;
pub mod date;
pub mod interval;
pub mod json;
pub mod number;
pub mod varchar;
pub mod vector;

pub use binary_float::{
    encode_binary_float, decode_binary_float,
    encode_binary_double, decode_binary_double,
};

use pyo3::prelude::*;

use crate::error::CodecError;
use crate::types::{PyTypeNum, TypeNum};

/// 하나의 DB 타입에 대한 Python ↔ Wire 변환을 담당하는 trait.
///
/// 구현체는 상태를 갖지 않는 unit struct로 정의합니다.
/// 새 타입 추가 시 이 trait만 구현하면 됩니다.
pub trait TypeCodec: Send + Sync {
    /// Python 값 → wire bytes (IN bind)
    fn encode(&self, py: Python<'_>, value: &Bound<'_, PyAny>) -> Result<Vec<u8>, CodecError>;

    /// Wire bytes → Python 값 (OUT fetch)
    fn decode(
        &self,
        py: Python<'_>,
        data: &[u8],
        target_type: PyTypeNum,
    ) -> Result<PyObject, CodecError>;

    /// 이 타입의 고정 wire 크기. 가변이면 None.
    fn wire_size(&self) -> Option<usize> {
        None
    }

    /// 이 codec이 처리하는 DB 타입 번호
    fn type_num(&self) -> TypeNum;

    /// 이 타입의 기본 Python 타입 (outputtypehandler 미지정 시)
    fn default_python_type(&self) -> PyTypeNum;
}
