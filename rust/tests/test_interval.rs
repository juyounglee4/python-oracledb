/// Oracle INTERVAL encoder/decoder 단위 테스트
///
/// DB 연결 불필요 — 순수 로직 검증
use _rust_impl::codec::{
    encode_interval_ds, decode_interval_ds, OracleIntervalDS,
    encode_interval_ym, decode_interval_ym, OracleIntervalYM,
};

// ═══════════════════════════════════════════════════════════
// INTERVAL DAY TO SECOND 테스트
// ═══════════════════════════════════════════════════════════

#[test]
fn test_encode_interval_ds_zero() {
    let interval = OracleIntervalDS {
        days: 0,
        hours: 0,
        minutes: 0,
        seconds: 0,
        fseconds: 0,
    };
    let result = encode_interval_ds(&interval);
    // days: 0 + 0x80000000 = 0x80000000
    // hours: 0+60=60, minutes: 0+60=60, seconds: 0+60=60
    // fseconds: 0 + 0x80000000 = 0x80000000
    assert_eq!(result[0..4], [0x80, 0x00, 0x00, 0x00]);
    assert_eq!(result[4], 60);
    assert_eq!(result[5], 60);
    assert_eq!(result[6], 60);
    assert_eq!(result[7..11], [0x80, 0x00, 0x00, 0x00]);
}

#[test]
fn test_encode_interval_ds_positive() {
    let interval = OracleIntervalDS {
        days: 5,
        hours: 3,
        minutes: 30,
        seconds: 15,
        fseconds: 500000000, // 0.5 seconds in nanoseconds
    };
    let result = encode_interval_ds(&interval);
    // days: 5 + 0x80000000
    let days_bytes = (5i32.wrapping_add(i32::MIN) as u32).to_be_bytes();
    assert_eq!(&result[0..4], &days_bytes);
    assert_eq!(result[4], 63);  // 3+60
    assert_eq!(result[5], 90);  // 30+60
    assert_eq!(result[6], 75);  // 15+60
}

#[test]
fn test_encode_interval_ds_negative() {
    let interval = OracleIntervalDS {
        days: -10,
        hours: -5,
        minutes: -30,
        seconds: -45,
        fseconds: -123000000,
    };
    let result = encode_interval_ds(&interval);
    let days_bytes = (-10i32).wrapping_add(i32::MIN) as u32;
    assert_eq!(&result[0..4], &days_bytes.to_be_bytes());
    assert_eq!(result[4], 55);  // -5+60
    assert_eq!(result[5], 30);  // -30+60
    assert_eq!(result[6], 15);  // -45+60
}

#[test]
fn test_interval_ds_roundtrip() {
    let intervals = vec![
        OracleIntervalDS { days: 0, hours: 0, minutes: 0, seconds: 0, fseconds: 0 },
        OracleIntervalDS { days: 1, hours: 2, minutes: 3, seconds: 4, fseconds: 500000000 },
        OracleIntervalDS { days: -1, hours: -2, minutes: -3, seconds: -4, fseconds: -500000000 },
        OracleIntervalDS { days: 999, hours: 23, minutes: 59, seconds: 59, fseconds: 999999999 },
        OracleIntervalDS { days: -999, hours: -23, minutes: -59, seconds: -59, fseconds: -999999999 },
    ];

    for interval in intervals {
        let encoded = encode_interval_ds(&interval);
        let decoded = decode_interval_ds(&encoded);
        assert_eq!(decoded, interval, "roundtrip failed for {:?}", interval);
    }
}

// ═══════════════════════════════════════════════════════════
// INTERVAL YEAR TO MONTH 테스트
// ═══════════════════════════════════════════════════════════

#[test]
fn test_encode_interval_ym_zero() {
    let interval = OracleIntervalYM { years: 0, months: 0 };
    let result = encode_interval_ym(&interval);
    assert_eq!(result[0..4], [0x80, 0x00, 0x00, 0x00]);
    assert_eq!(result[4], 60);
}

#[test]
fn test_encode_interval_ym_positive() {
    let interval = OracleIntervalYM { years: 5, months: 6 };
    let result = encode_interval_ym(&interval);
    let years_bytes = (5i32.wrapping_add(i32::MIN) as u32).to_be_bytes();
    assert_eq!(&result[0..4], &years_bytes);
    assert_eq!(result[4], 66);  // 6+60
}

#[test]
fn test_encode_interval_ym_negative() {
    let interval = OracleIntervalYM { years: -3, months: -11 };
    let result = encode_interval_ym(&interval);
    let years_bytes = (-3i32).wrapping_add(i32::MIN) as u32;
    assert_eq!(&result[0..4], &years_bytes.to_be_bytes());
    assert_eq!(result[4], 49);  // -11+60
}

#[test]
fn test_interval_ym_roundtrip() {
    let intervals = vec![
        OracleIntervalYM { years: 0, months: 0 },
        OracleIntervalYM { years: 1, months: 6 },
        OracleIntervalYM { years: -1, months: -6 },
        OracleIntervalYM { years: 100, months: 11 },
        OracleIntervalYM { years: -100, months: -11 },
    ];

    for interval in intervals {
        let encoded = encode_interval_ym(&interval);
        let decoded = decode_interval_ym(&encoded);
        assert_eq!(decoded, interval, "roundtrip failed for {:?}", interval);
    }
}
