//! BINARY_FLOAT (4 bytes) 및 BINARY_DOUBLE (8 bytes) 타입 codec.
//!
//! 두 타입 모두 IEEE 754 부동소수점에 Tibero sign-flip 인코딩을 적용하여
//! unsigned byte 비교(memcmp)만으로 정렬 순서가 보존되는 wire format을 사용합니다.
//! 알고리즘이 동일하고 바이트 크기(4 vs 8)만 다르므로 하나의 모듈에 함께 둡니다.

use pyo3::prelude::*;
use pyo3::types::{PyFloat, PyString};

use crate::codec::TypeCodec;
use crate::error::CodecError;
use crate::types::{PyTypeNum, TypeNum};

// ═══════════════════════════════════════════════════════════
// Standalone encode/decode 함수 (테스트 및 외부 사용)
// ═══════════════════════════════════════════════════════════

/// BINARY_FLOAT encode: f32 → 4-byte sortable wire format (big-endian).
///
/// Tibero sign-flip 알고리즘:
/// - 양수: 부호 반전 후 IEEE 754 비트 추출 → big-endian (MSB=1이 됨)
/// - 음수: IEEE 754 비트 추출 후 비트 NOT → big-endian (MSB=0이 됨)
/// - ±0: 고정 `0x80 00 00 00`
/// - NaN: 고정 `0xFF C0 00 00`
///
/// 결과적으로 unsigned byte 비교 시 음수 < 0 < 양수 순서가 보존됩니다.
pub fn encode_binary_float(value: f32) -> [u8; 4] {
    if value.is_nan() {
        return [0xFF, 0xC0, 0x00, 0x00];
    }

    // ±0 처리: f32에서 +0과 -0은 ==로 같으므로 bits로 구분
    if value == 0.0 {
        // tbjdbc는 +0과 -0 모두 0x80000000으로 인코딩
        return [0x80, 0x00, 0x00, 0x00];
    }

    if value > 0.0 {
        // 양수: 부호 반전(-value)의 IEEE 754 비트를 big-endian으로 저장
        // -value의 MSB(sign bit)는 1이므로 결과 첫 바이트의 MSB=1
        let bits = (-value).to_bits();
        bits.to_be_bytes()
    } else {
        // 음수: IEEE 754 비트를 NOT 후 big-endian으로 저장
        // 음수의 MSB(sign bit)는 1이고, NOT하면 0이 됨
        let bits = !value.to_bits();
        bits.to_be_bytes()
    }
}

/// BINARY_FLOAT decode: 4-byte sortable wire format → f32.
///
/// encode의 역변환:
/// - MSB=1 (intBits < 0 as i32): 양수였음 → intBitsToFloat 후 부호 반전
/// - MSB=0, 비영 (intBits > 0 as i32): 음수였음 → NOT 후 intBitsToFloat
/// - 0: 0.0
pub fn decode_binary_float(data: &[u8; 4]) -> f32 {
    let bits = u32::from_be_bytes(*data);
    let signed = bits as i32;

    if signed < 0 {
        // MSB=1 → 원래 양수 (인코딩 시 -value의 비트를 저장했음)
        // 비트를 float로 해석하면 음수 → 부호 반전
        let f = f32::from_bits(bits);
        -f
    } else if signed > 0 {
        // MSB=0, 비영 → 원래 음수 (인코딩 시 NOT 했음)
        f32::from_bits(!bits)
    } else {
        // 0x00000000 → 0.0
        0.0f32
    }
}

/// BINARY_DOUBLE encode: f64 → 8-byte sortable wire format (big-endian).
///
/// BINARY_FLOAT와 동일한 sign-flip 알고리즘의 8바이트 확장.
pub fn encode_binary_double(value: f64) -> [u8; 8] {
    if value.is_nan() {
        return [0xFF, 0xF8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    }

    if value == 0.0 {
        return [0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    }

    if value > 0.0 {
        let bits = (-value).to_bits();
        bits.to_be_bytes()
    } else {
        let bits = !value.to_bits();
        bits.to_be_bytes()
    }
}

/// BINARY_DOUBLE decode: 8-byte sortable wire format → f64.
pub fn decode_binary_double(data: &[u8; 8]) -> f64 {
    let bits = u64::from_be_bytes(*data);
    let signed = bits as i64;

    if signed < 0 {
        let f = f64::from_bits(bits);
        -f
    } else if signed > 0 {
        f64::from_bits(!bits)
    } else {
        0.0f64
    }
}

// ═══════════════════════════════════════════════════════════
// BINARY_FLOAT TypeCodec
// ═══════════════════════════════════════════════════════════

/// BINARY_FLOAT 타입 codec (IEEE 754 + sign flip, 4 bytes)
pub struct BinaryFloatCodec;

impl BinaryFloatCodec {
    const WIRE_SIZE: usize = 4;
}

impl TypeCodec for BinaryFloatCodec {
    fn encode(&self, _py: Python<'_>, value: &Bound<'_, PyAny>) -> Result<Vec<u8>, CodecError> {
        let f: f32 = value.extract::<f64>().map_err(CodecError::Python)? as f32;
        Ok(encode_binary_float(f).to_vec())
    }

    fn decode(
        &self,
        py: Python<'_>,
        data: &[u8],
        target_type: PyTypeNum,
    ) -> Result<PyObject, CodecError> {
        if data.len() != Self::WIRE_SIZE {
            return Err(CodecError::InvalidData {
                type_num: self.type_num(),
                detail: format!(
                    "BINARY_FLOAT expects {} bytes, got {}",
                    Self::WIRE_SIZE,
                    data.len()
                ),
            });
        }
        let bytes: [u8; 4] = data.try_into().unwrap();
        let f = decode_binary_float(&bytes);

        match target_type {
            PyTypeNum::Float => Ok(PyFloat::new_bound(py, f as f64).into_any().unbind()),
            PyTypeNum::Str => {
                let s = f.to_string();
                Ok(PyString::new_bound(py, &s).into_any().unbind())
            }
            PyTypeNum::Int => {
                use pyo3::IntoPy;
                Ok((f as i64).into_py(py))
            }
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

// ═══════════════════════════════════════════════════════════
// BINARY_DOUBLE TypeCodec
// ═══════════════════════════════════════════════════════════

/// BINARY_DOUBLE 타입 codec (IEEE 754 + sign flip, 8 bytes)
pub struct BinaryDoubleCodec;

impl BinaryDoubleCodec {
    const WIRE_SIZE: usize = 8;
}

impl TypeCodec for BinaryDoubleCodec {
    fn encode(&self, _py: Python<'_>, value: &Bound<'_, PyAny>) -> Result<Vec<u8>, CodecError> {
        let f: f64 = value.extract::<f64>().map_err(CodecError::Python)?;
        Ok(encode_binary_double(f).to_vec())
    }

    fn decode(
        &self,
        py: Python<'_>,
        data: &[u8],
        target_type: PyTypeNum,
    ) -> Result<PyObject, CodecError> {
        if data.len() != Self::WIRE_SIZE {
            return Err(CodecError::InvalidData {
                type_num: self.type_num(),
                detail: format!(
                    "BINARY_DOUBLE expects {} bytes, got {}",
                    Self::WIRE_SIZE,
                    data.len()
                ),
            });
        }
        let bytes: [u8; 8] = data.try_into().unwrap();
        let f = decode_binary_double(&bytes);

        match target_type {
            PyTypeNum::Float => Ok(PyFloat::new_bound(py, f).into_any().unbind()),
            PyTypeNum::Str => {
                let s = f.to_string();
                Ok(PyString::new_bound(py, &s).into_any().unbind())
            }
            PyTypeNum::Int => {
                use pyo3::IntoPy;
                Ok((f as i64).into_py(py))
            }
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
