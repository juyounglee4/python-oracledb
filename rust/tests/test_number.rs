/// Oracle NUMBER encoder/decoder 단위 테스트
///
/// DB 연결 불필요 — 순수 로직 검증
use _rust_impl::codec::{encode_number, decode_number, EncodeError};

// ═══════════════════════════════════════════════════════════
// encode_number 테스트
// ═══════════════════════════════════════════════════════════

#[test]
fn test_encode_zero() {
    assert_eq!(encode_number(b"0").unwrap(), vec![0x80]);
}

#[test]
fn test_encode_positive_integer_single_digit() {
    // 1 → exp=192(0xC0), mantissa: pair(01)=1+1=2
    let result = encode_number(b"1").unwrap();
    assert_eq!(result, vec![0xC1, 0x02]);
}

#[test]
fn test_encode_positive_integer_two_digits() {
    // 12 → exp=0xC1, pair(12)=13
    let result = encode_number(b"12").unwrap();
    assert_eq!(result, vec![0xC1, 0x0D]);
}

#[test]
fn test_encode_positive_integer_three_digits() {
    // 123 → exp=0xC1, pairs: (01)(23) → [2, 24]
    let result = encode_number(b"123").unwrap();
    assert_eq!(result, vec![0xC2, 0x02, 0x18]);
}

#[test]
fn test_encode_positive_decimal() {
    // 123.45 → dpi=3(odd), prepend → [0,1,2,3,4,5] → pairs: (01,23,45) → [2,24,46]
    // exp=(3-1)/2=1, exp_byte=0xC2
    let result = encode_number(b"123.45").unwrap();
    assert_eq!(result, vec![0xC2, 0x02, 0x18, 0x2E]);
}

#[test]
fn test_encode_negative_integer() {
    // -1 → exp NOT, pair complement, sentinel
    let result = encode_number(b"-1").unwrap();
    // exp_byte = !(0xC1) = 0x3E, mantissa: 101-1=100, sentinel: 102
    assert_eq!(result, vec![0x3E, 0x64, 0x66]);
}

#[test]
fn test_encode_negative_decimal() {
    // -12.3 → dpi=2(even), no prepend → [1,2,3,0] → pairs: (12,30) → 101-12=89, 101-30=71
    // exp=(2-1)/2=0, exp_byte=!0xC1=0x3E, sentinel: 102
    let result = encode_number(b"-12.3").unwrap();
    assert_eq!(result, vec![0x3E, 0x59, 0x47, 0x66]);
}

#[test]
fn test_encode_small_decimal() {
    // 0.5 → dpi=0(even), no prepend → [5], append 0 → [5,0] → pair(50)=51=0x33
    // exp=(0-2)/2=-1, exp_byte=0xC0
    let result = encode_number(b"0.5").unwrap();
    assert_eq!(result, vec![0xC0, 0x33]);
}

#[test]
fn test_encode_large_number() {
    // 9999 → dpi=4(even), no prepend → [9,9,9,9] → pairs: (99,99) → [100,100]
    // exp=(4-1)/2=1, exp_byte=0xC2
    let result = encode_number(b"9999").unwrap();
    assert_eq!(result, vec![0xC2, 0x64, 0x64]);
}

#[test]
fn test_encode_leading_zeros_stripped() {
    // 007 == 7
    assert_eq!(encode_number(b"007").unwrap(), encode_number(b"7").unwrap());
}

#[test]
fn test_encode_trailing_zeros_stripped() {
    // 100 → digits after strip: [1], dpi=3(odd)
    let result = encode_number(b"100").unwrap();
    // dpi=3, prepend → [0,1] → pair(01)=1+1=2
    // exp=(3-1)/2=1, exp_byte=0xC2
    assert_eq!(result, vec![0xC2, 0x02]);
}

// ─── 에러 케이스 ───────────────────────────────────────────

#[test]
fn test_encode_empty_input() {
    assert_eq!(encode_number(b"").unwrap_err(), EncodeError::ZeroLength);
}

#[test]
fn test_encode_invalid_chars() {
    assert_eq!(encode_number(b"abc").unwrap_err(), EncodeError::InvalidNumber);
}

#[test]
fn test_encode_just_sign() {
    assert_eq!(encode_number(b"-").unwrap_err(), EncodeError::InvalidNumber);
}

#[test]
fn test_encode_double_dot() {
    assert_eq!(encode_number(b"1.2.3").unwrap_err(), EncodeError::InvalidNumber);
}

#[test]
fn test_encode_exponent_only() {
    assert_eq!(encode_number(b"1e").unwrap_err(), EncodeError::InvalidExponent);
}

#[test]
fn test_encode_too_many_digits() {
    // 41 digits → should fail
    let input = "9".repeat(41);
    assert_eq!(
        encode_number(input.as_bytes()).unwrap_err(),
        EncodeError::NoRepresentation
    );
}

// ─── 지수 표기 ───────────────────────────────────────────

#[test]
fn test_encode_scientific_positive() {
    // 1e2 == 100
    assert_eq!(
        encode_number(b"1e2").unwrap(),
        encode_number(b"100").unwrap()
    );
}

#[test]
fn test_encode_scientific_negative_exp() {
    // 1e-1 == 0.1
    assert_eq!(
        encode_number(b"1e-1").unwrap(),
        encode_number(b"0.1").unwrap()
    );
}

// ═══════════════════════════════════════════════════════════
// decode_number 테스트
// ═══════════════════════════════════════════════════════════

#[test]
fn test_decode_zero() {
    assert_eq!(decode_number(&[0x80]).unwrap(), "0");
}

#[test]
fn test_decode_positive_single() {
    // [0xC1, 0x02] → 1
    assert_eq!(decode_number(&[0xC1, 0x02]).unwrap(), "1");
}

#[test]
fn test_decode_positive_multi() {
    // [0xC2, 0x02, 0x18] → encode(123)
    let encoded = encode_number(b"123").unwrap();
    assert_eq!(decode_number(&encoded).unwrap(), "123");
}

#[test]
fn test_decode_negative() {
    let encoded = encode_number(b"-1").unwrap();
    assert_eq!(decode_number(&encoded).unwrap(), "-1");
}

#[test]
fn test_decode_decimal() {
    let encoded = encode_number(b"0.5").unwrap();
    let decoded = decode_number(&encoded).unwrap();
    assert_eq!(decoded, "0.5");
}

// ─── Roundtrip 테스트 ──────────────────────────────────────

#[test]
fn test_roundtrip_integers() {
    let values = ["0", "1", "9", "10", "99", "100", "999", "1000", "9999"];
    for v in values {
        let encoded = encode_number(v.as_bytes()).unwrap();
        let decoded = decode_number(&encoded).unwrap();
        assert_eq!(decoded, v, "roundtrip failed for {}", v);
    }
}

#[test]
fn test_roundtrip_negative_integers() {
    let values = ["-1", "-9", "-10", "-99", "-100", "-999"];
    for v in values {
        let encoded = encode_number(v.as_bytes()).unwrap();
        let decoded = decode_number(&encoded).unwrap();
        assert_eq!(decoded, v, "roundtrip failed for {}", v);
    }
}

#[test]
fn test_roundtrip_decimals() {
    let values = ["0.1", "0.5", "1.5", "12.34", "0.01", "123.456"];
    for v in values {
        let encoded = encode_number(v.as_bytes()).unwrap();
        let decoded = decode_number(&encoded).unwrap();
        assert_eq!(decoded, v, "roundtrip failed for {}", v);
    }
}

#[test]
fn test_roundtrip_negative_decimals() {
    let values = ["-0.1", "-0.5", "-1.5", "-12.34"];
    for v in values {
        let encoded = encode_number(v.as_bytes()).unwrap();
        let decoded = decode_number(&encoded).unwrap();
        assert_eq!(decoded, v, "roundtrip failed for {}", v);
    }
}

#[test]
fn test_roundtrip_large_numbers() {
    let values = [
        "12345678901234567890",
        "99999999999999999999",
        "1000000000",
    ];
    for v in values {
        let encoded = encode_number(v.as_bytes()).unwrap();
        let decoded = decode_number(&encoded).unwrap();
        assert_eq!(decoded, v, "roundtrip failed for {}", v);
    }
}
