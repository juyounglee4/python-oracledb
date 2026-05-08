/// Oracle BINARY_DOUBLE/FLOAT encoder/decoder 단위 테스트
///
/// DB 연결 불필요 — 순수 로직 검증
use _rust_impl::codec::{
    encode_binary_double, decode_binary_double,
    encode_binary_float, decode_binary_float,
};

// ═══════════════════════════════════════════════════════════
// BINARY_DOUBLE 테스트
// ═══════════════════════════════════════════════════════════

#[test]
fn test_encode_double_positive_one() {
    let result = encode_binary_double(1.0);
    // positive → MSB | 0x80
    assert!(result[0] & 0x80 != 0, "MSB should be set for positive");
}

#[test]
fn test_encode_double_negative_one() {
    let result = encode_binary_double(-1.0);
    // negative → all bytes inverted, MSB should be 0
    assert!(result[0] & 0x80 == 0, "MSB should be clear for negative");
}

#[test]
fn test_encode_double_zero() {
    let result = encode_binary_double(0.0);
    // +0.0: IEEE754 BE is [0x00, ...], after flip MSB: [0x80, 0, 0, 0, 0, 0, 0, 0]
    assert_eq!(result, [0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
}

#[test]
fn test_encode_double_negative_zero() {
    let result = encode_binary_double(-0.0);
    // -0.0: IEEE754 BE is [0x80, 0, ...], negative path: invert all → [0x7F, 0xFF, ...]
    assert_eq!(result, [0x7F, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
}

#[test]
fn test_double_ordering_preserved() {
    // Oracle's encoding preserves memcmp ordering
    let values = [-100.0, -1.0, -0.001, 0.0, 0.001, 1.0, 100.0];
    let encoded: Vec<[u8; 8]> = values.iter().map(|&v| encode_binary_double(v)).collect();

    for i in 0..encoded.len() - 1 {
        assert!(
            encoded[i] < encoded[i + 1],
            "ordering broken: {:?} ({}) should < {:?} ({})",
            encoded[i], values[i], encoded[i + 1], values[i + 1]
        );
    }
}

#[test]
fn test_double_roundtrip() {
    let values = [
        0.0, 1.0, -1.0, 0.1, -0.1,
        f64::MAX, f64::MIN,
        f64::MIN_POSITIVE,
        std::f64::consts::PI,
        -std::f64::consts::E,
        1e100, -1e100,
        1e-100, -1e-100,
    ];
    for v in values {
        let encoded = encode_binary_double(v);
        let decoded = decode_binary_double(&encoded);
        assert_eq!(decoded, v, "roundtrip failed for {}", v);
    }
}

#[test]
fn test_double_roundtrip_subnormal() {
    let subnormal = 5e-324; // smallest positive f64
    let encoded = encode_binary_double(subnormal);
    let decoded = decode_binary_double(&encoded);
    assert_eq!(decoded, subnormal);
}

// ═══════════════════════════════════════════════════════════
// BINARY_FLOAT 테스트
// ═══════════════════════════════════════════════════════════

#[test]
fn test_encode_float_positive_one() {
    let result = encode_binary_float(1.0f32);
    assert!(result[0] & 0x80 != 0, "MSB should be set for positive");
}

#[test]
fn test_encode_float_negative_one() {
    let result = encode_binary_float(-1.0f32);
    assert!(result[0] & 0x80 == 0, "MSB should be clear for negative");
}

#[test]
fn test_encode_float_zero() {
    let result = encode_binary_float(0.0f32);
    assert_eq!(result, [0x80, 0x00, 0x00, 0x00]);
}

#[test]
fn test_float_ordering_preserved() {
    let values: Vec<f32> = vec![-100.0, -1.0, -0.001, 0.0, 0.001, 1.0, 100.0];
    let encoded: Vec<[u8; 4]> = values.iter().map(|&v| encode_binary_float(v)).collect();

    for i in 0..encoded.len() - 1 {
        assert!(
            encoded[i] < encoded[i + 1],
            "ordering broken at index {}: {:?} ({}) should < {:?} ({})",
            i, encoded[i], values[i], encoded[i + 1], values[i + 1]
        );
    }
}

#[test]
fn test_float_roundtrip() {
    let values: Vec<f32> = vec![
        0.0, 1.0, -1.0, 0.5, -0.5,
        f32::MAX, f32::MIN,
        f32::MIN_POSITIVE,
        std::f32::consts::PI,
        -std::f32::consts::E,
        1e30, -1e30,
    ];
    for v in values {
        let encoded = encode_binary_float(v);
        let decoded = decode_binary_float(&encoded);
        assert_eq!(decoded, v, "roundtrip failed for {}", v);
    }
}
