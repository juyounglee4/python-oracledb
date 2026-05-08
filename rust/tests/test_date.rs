/// Oracle DATE/TIMESTAMP encoder/decoder 단위 테스트
///
/// DB 연결 불필요 — 순수 로직 검증
use _rust_impl::codec::{encode_date, decode_date, OracleDate};

// ═══════════════════════════════════════════════════════════
// encode_date 테스트
// ═══════════════════════════════════════════════════════════

#[test]
fn test_encode_date_basic() {
    // 2024-03-15 10:30:45
    let date = OracleDate {
        year: 2024,
        month: 3,
        day: 15,
        hour: 10,
        minute: 30,
        second: 45,
        fsecond: 0,
        tz_hour_offset: 0,
        tz_minute_offset: 0,
    };
    let result = encode_date(&date);
    // century: 2024/100 + 100 = 120
    // year_in_century: 2024%100 + 100 = 124
    // hour+1=11, minute+1=31, second+1=46
    assert_eq!(result, vec![120, 124, 3, 15, 11, 31, 46]);
}

#[test]
fn test_encode_date_midnight() {
    // 2000-01-01 00:00:00
    let date = OracleDate {
        year: 2000,
        month: 1,
        day: 1,
        hour: 0,
        minute: 0,
        second: 0,
        fsecond: 0,
        tz_hour_offset: 0,
        tz_minute_offset: 0,
    };
    let result = encode_date(&date);
    // century: 120, year_in_century: 100
    // hour+1=1, min+1=1, sec+1=1
    assert_eq!(result, vec![120, 100, 1, 1, 1, 1, 1]);
}

#[test]
fn test_encode_date_with_fractional_seconds() {
    // 2024-06-15 14:30:45.123456000 (nanoseconds)
    let date = OracleDate {
        year: 2024,
        month: 6,
        day: 15,
        hour: 14,
        minute: 30,
        second: 45,
        fsecond: 123456000, // nanoseconds
        tz_hour_offset: 0,
        tz_minute_offset: 0,
    };
    let result = encode_date(&date);
    assert_eq!(result.len(), 11); // TIMESTAMP = 7 + 4 bytes
    assert_eq!(&result[..7], &[120, 124, 6, 15, 15, 31, 46]);
    // fsecond as BE uint32
    let fs_bytes = &result[7..11];
    let fs = u32::from_be_bytes([fs_bytes[0], fs_bytes[1], fs_bytes[2], fs_bytes[3]]);
    assert_eq!(fs, 123456000);
}

#[test]
fn test_encode_date_19th_century() {
    // 1899-12-31 23:59:59
    let date = OracleDate {
        year: 1899,
        month: 12,
        day: 31,
        hour: 23,
        minute: 59,
        second: 59,
        fsecond: 0,
        tz_hour_offset: 0,
        tz_minute_offset: 0,
    };
    let result = encode_date(&date);
    // century: 1899/100 + 100 = 18 + 100 = 118
    // year_in_century: 1899%100 + 100 = 99 + 100 = 199
    assert_eq!(result, vec![118, 199, 12, 31, 24, 60, 60]);
}

#[test]
fn test_encode_date_bc() {
    // Year 1 BC → year=0 in Oracle convention is tricky
    // year=1: century=1/100+100=100, year_in_century=1%100+100=101
    let date = OracleDate {
        year: 1,
        month: 1,
        day: 1,
        hour: 0,
        minute: 0,
        second: 0,
        fsecond: 0,
        tz_hour_offset: 0,
        tz_minute_offset: 0,
    };
    let result = encode_date(&date);
    assert_eq!(result, vec![100, 101, 1, 1, 1, 1, 1]);
}

// ═══════════════════════════════════════════════════════════
// decode_date 테스트
// ═══════════════════════════════════════════════════════════

#[test]
fn test_decode_date_basic() {
    let wire = [120u8, 124, 3, 15, 11, 31, 46];
    let date = decode_date(&wire).unwrap();
    assert_eq!(date.year, 2024);
    assert_eq!(date.month, 3);
    assert_eq!(date.day, 15);
    assert_eq!(date.hour, 10);
    assert_eq!(date.minute, 30);
    assert_eq!(date.second, 45);
    assert_eq!(date.fsecond, 0);
}

#[test]
fn test_decode_timestamp() {
    // 11 bytes = TIMESTAMP
    let mut wire = vec![120u8, 124, 6, 15, 15, 31, 46];
    wire.extend_from_slice(&123456000u32.to_be_bytes());
    let date = decode_date(&wire).unwrap();
    assert_eq!(date.year, 2024);
    assert_eq!(date.month, 6);
    assert_eq!(date.day, 15);
    assert_eq!(date.hour, 14);
    assert_eq!(date.minute, 30);
    assert_eq!(date.second, 45);
    assert_eq!(date.fsecond, 123456000);
}

#[test]
fn test_decode_timestamp_with_tz() {
    // 13 bytes = TIMESTAMP WITH TZ
    let mut wire = vec![120u8, 124, 6, 15, 15, 31, 46];
    wire.extend_from_slice(&0u32.to_be_bytes()); // fsecond=0
    wire.push(20 + 9);  // tz_hour = +9 (KST)
    wire.push(60 + 0);  // tz_min = 0
    let date = decode_date(&wire).unwrap();
    assert_eq!(date.tz_hour_offset, 9);
    assert_eq!(date.tz_minute_offset, 0);
}

#[test]
fn test_decode_too_short() {
    let wire = [120u8, 124, 3, 15, 11, 31]; // 6 bytes < 7
    assert!(decode_date(&wire).is_err());
}

// ─── Roundtrip 테스트 ──────────────────────────────────────

#[test]
fn test_date_roundtrip() {
    let dates = vec![
        OracleDate { year: 2024, month: 1, day: 1, hour: 0, minute: 0, second: 0, fsecond: 0, tz_hour_offset: 0, tz_minute_offset: 0 },
        OracleDate { year: 1999, month: 12, day: 31, hour: 23, minute: 59, second: 59, fsecond: 0, tz_hour_offset: 0, tz_minute_offset: 0 },
        OracleDate { year: 2024, month: 6, day: 15, hour: 14, minute: 30, second: 45, fsecond: 500000000, tz_hour_offset: 0, tz_minute_offset: 0 },
    ];

    for date in dates {
        let encoded = encode_date(&date);
        let decoded = decode_date(&encoded).unwrap();
        assert_eq!(decoded, date, "roundtrip failed for {:?}", date);
    }
}
