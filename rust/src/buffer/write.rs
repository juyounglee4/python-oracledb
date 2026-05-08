/// Wire bytes 쓰기 버퍼
///
/// 내부 Vec<u8>에 데이터를 축적합니다.
pub struct WriteBuffer {
    buf: Vec<u8>,
}

impl WriteBuffer {
    pub fn new() -> Self {
        Self { buf: Vec::new() }
    }

    pub fn with_capacity(cap: usize) -> Self {
        Self {
            buf: Vec::with_capacity(cap),
        }
    }

    /// 현재까지 쓴 바이트 수
    pub fn len(&self) -> usize {
        self.buf.len()
    }

    /// 버퍼가 비었는지
    pub fn is_empty(&self) -> bool {
        self.buf.is_empty()
    }

    /// 원시 바이트 추가
    pub fn write_bytes(&mut self, data: &[u8]) {
        todo!("raw bytes 추가")
    }

    /// 1바이트 쓰기
    pub fn write_u8(&mut self, val: u8) {
        todo!("1바이트 쓰기")
    }

    /// Big-endian u16 쓰기
    pub fn write_u16_be(&mut self, val: u16) {
        todo!("2바이트 BE 쓰기")
    }

    /// Big-endian u32 쓰기
    pub fn write_u32_be(&mut self, val: u32) {
        todo!("4바이트 BE 쓰기")
    }

    /// Big-endian i32 쓰기
    pub fn write_i32_be(&mut self, val: i32) {
        todo!("4바이트 BE signed 쓰기")
    }

    /// 길이-접두 데이터 쓰기 (length encoding 포함)
    pub fn write_length_prefixed(&mut self, data: &[u8]) {
        todo!("길이 인코딩 + 데이터 쓰기")
    }

    /// NULL 마커 쓰기 (0x00)
    pub fn write_null(&mut self) {
        todo!("NULL 마커 쓰기")
    }

    /// 내부 버퍼 소비하여 Vec<u8> 반환
    pub fn into_bytes(self) -> Vec<u8> {
        self.buf
    }
}
