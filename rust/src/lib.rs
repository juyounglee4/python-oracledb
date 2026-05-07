use pyo3::prelude::*;

/// Oracle NUMBER wire format encoder
#[pyfunction]
fn encode_number(input: &[u8]) -> PyResult<Vec<u8>> {
    // TODO: Phase 1에서 구현
    Ok(vec![0x80]) // placeholder: zero
}

/// Oracle NUMBER wire format decoder
#[pyfunction]
fn decode_number(data: &[u8]) -> PyResult<String> {
    // TODO: Phase 1에서 구현
    Ok("0".to_string()) // placeholder
}

#[pymodule]
fn _rust_impl(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(encode_number, m)?)?;
    m.add_function(wrap_pyfunction!(decode_number, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_number_zero() {
        // Phase 1에서 구현 후 실제 검증
        let result = encode_number_internal(b"0");
        assert_eq!(result, vec![0x80]);
    }
}