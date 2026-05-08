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
    // tbjdbc: -0.0과 +0.0은 동일하게 0x80 00 ... 으로 인코딩 (DB에서 같은 값)
    assert_eq!(result, [0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
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

#[test]
fn test_encode_double_nan() {
    let result = encode_binary_double(f64::NAN);
    assert_eq!(result, [0xFF, 0xF8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
}

#[test]
fn test_decode_double_nan() {
    let wire = [0xFF, 0xF8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    let decoded = decode_binary_double(&wire);
    assert!(decoded.is_nan(), "expected NaN, got {}", decoded);
}

#[test]
fn test_double_infinity_roundtrip() {
    let pos_inf = encode_binary_double(f64::INFINITY);
    let neg_inf = encode_binary_double(f64::NEG_INFINITY);

    assert_eq!(decode_binary_double(&pos_inf), f64::INFINITY);
    assert_eq!(decode_binary_double(&neg_inf), f64::NEG_INFINITY);

    // ordering: -INF < 0 < +INF
    assert!(neg_inf < encode_binary_double(0.0));
    assert!(encode_binary_double(0.0) < pos_inf);
}

#[test]
fn test_double_negative_zero_loses_sign() {
    // tbjdbc: -0.0과 +0.0은 동일 wire 표현이므로 decode 시 부호 구분 소멸
    let encoded = encode_binary_double(-0.0);
    let decoded = decode_binary_double(&encoded);
    assert_eq!(decoded, 0.0);
    assert!(decoded.is_sign_positive(), "-0.0 should decode as +0.0");
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

#[test]
fn test_encode_float_negative_zero() {
    let result = encode_binary_float(-0.0f32);
    // tbjdbc: -0.0과 +0.0 동일하게 인코딩
    assert_eq!(result, [0x80, 0x00, 0x00, 0x00]);
}

#[test]
fn test_encode_float_nan() {
    let result = encode_binary_float(f32::NAN);
    assert_eq!(result, [0xFF, 0xC0, 0x00, 0x00]);
}

#[test]
fn test_decode_float_nan() {
    let wire = [0xFF, 0xC0, 0x00, 0x00];
    let decoded = decode_binary_float(&wire);
    assert!(decoded.is_nan(), "expected NaN, got {}", decoded);
}

#[test]
fn test_float_infinity_roundtrip() {
    let pos_inf = encode_binary_float(f32::INFINITY);
    let neg_inf = encode_binary_float(f32::NEG_INFINITY);

    assert_eq!(decode_binary_float(&pos_inf), f32::INFINITY);
    assert_eq!(decode_binary_float(&neg_inf), f32::NEG_INFINITY);

    // ordering: -INF < 0 < +INF
    assert!(neg_inf < encode_binary_float(0.0f32));
    assert!(encode_binary_float(0.0f32) < pos_inf);
}

#[test]
fn test_float_negative_zero_loses_sign() {
    let encoded = encode_binary_float(-0.0f32);
    let decoded = decode_binary_float(&encoded);
    assert_eq!(decoded, 0.0f32);
    assert!(decoded.is_sign_positive(), "-0.0 should decode as +0.0");
}

#[test]
fn test_float_roundtrip_subnormal() {
    let subnormal: f32 = 1.4e-45; // smallest positive f32
    let encoded = encode_binary_float(subnormal);
    let decoded = decode_binary_float(&encoded);
    assert_eq!(decoded, subnormal);
}
