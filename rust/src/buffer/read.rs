/// Wire bytes 읽기 버퍼
///
/// Cursor 패턴으로 구현합니다. offset을 내부적으로 관리하여
/// 순차적으로 데이터를 읽습니다.
pub struct ReadBuffer<'a> {
    data: &'a [u8],
    offset: usize,
}

impl<'a> ReadBuffer<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, offset: 0 }
    }

    /// 현재 offset 반환
    pub fn offset(&self) -> usize {
        self.offset
    }

    /// 남은 바이트 수
    pub fn remaining(&self) -> usize {
        self.data.len().saturating_sub(self.offset)
    }

    /// n 바이트 읽기 (offset 전진)
    pub fn read_bytes(&mut self, n: usize) -> Result<&'a [u8], crate::error::CodecError> {
        todo!("슬라이스 반환 및 offset 전진")
    }

    /// 1 바이트 읽기
    pub fn read_u8(&mut self) -> Result<u8, crate::error::CodecError> {
        todo!("1바이트 읽기")
    }

    /// Big-endian u16 읽기
    pub fn read_u16_be(&mut self) -> Result<u16, crate::error::CodecError> {
        todo!("2바이트 BE 읽기")
    }

    /// Big-endian u32 읽기
    pub fn read_u32_be(&mut self) -> Result<u32, crate::error::CodecError> {
        todo!("4바이트 BE 읽기")
    }

    /// Big-endian i32 읽기
    pub fn read_i32_be(&mut self) -> Result<i32, crate::error::CodecError> {
        todo!("4바이트 BE signed 읽기")
    }

    /// 길이-접두 데이터 읽기 (length byte 해석 포함)
    pub fn read_length_prefixed(&mut self) -> Result<Option<&'a [u8]>, crate::error::CodecError> {
        todo!("길이 바이트 해석 후 데이터 반환, NULL이면 None")
    }
}
