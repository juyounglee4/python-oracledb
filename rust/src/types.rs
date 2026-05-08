/// DB 타입 번호 (wire protocol에서 사용하는 타입 코드)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum TypeNum {
    Varchar = 1,
    Number = 2,
    BinaryInteger = 3,
    Float = 4,
    Double = 5,
    Raw = 6,
    Char = 7,
    Long = 8,
    LongRaw = 9,
    Date = 12,
    Timestamp = 180,
    TimestampTz = 181,
    TimestampLtz = 231,
    IntervalDs = 183,
    IntervalYm = 182,
    Boolean = 252,
    Clob = 112,
    Blob = 113,
    Json = 119,
    Vector = 127,
    Cursor = 102,
    Rowid = 104,
    Object = 108,
    // Tibero 전용 타입은 여기에 추가
}

/// Python 대상 타입 번호 (outputtypehandler에서 라우팅용)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PyTypeNum {
    Int,
    Float,
    Decimal,
    Str,
    Bytes,
    Datetime,
    Timedelta,
    Bool,
    IntervalYm,
}

/// 컬럼 메타데이터 (codec lookup 및 변환에 필요한 정보)
pub struct ColumnMetadata {
    pub type_num: TypeNum,
    pub python_type: Option<PyTypeNum>,
}
