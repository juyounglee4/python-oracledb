pub mod read;
pub mod write;

/// 길이 인코딩/디코딩 유틸리티 (Chunked Transfer 방식)
///
/// Wire format 규칙:
/// - 0x00          → NULL
/// - 0x01..0xFE    → (value - 1) 바이트가 바로 따라옴 (최대 253 bytes)
/// - 0xFE          → Chunked: 뒤이어 chunk 들이 오며 0x00으로 종료
pub fn decode_length(data: &[u8], offset: &mut usize) -> Option<usize> {
    todo!("length byte 해석 → Some(len) 또는 None(NULL)")
}

pub fn encode_length(buf: &mut Vec<u8>, len: usize) {
    todo!("데이터 길이에 따라 1-byte 또는 chunked 인코딩")
}
