use std::collections::HashMap;

use crate::codec::TypeCodec;
use crate::error::CodecError;
use crate::types::TypeNum;

/// 타입별 codec 레지스트리
///
/// TypeNum → Box<dyn TypeCodec> 매핑으로 1-step O(1) dispatch를 수행합니다.
/// 애플리케이션 시작 시 한 번 빌드하고 이후 읽기 전용으로 사용합니다.
pub struct TypeCodecRegistry {
    codecs: HashMap<TypeNum, Box<dyn TypeCodec>>,
}

impl TypeCodecRegistry {
    /// 빈 레지스트리 생성
    pub fn new() -> Self {
        Self {
            codecs: HashMap::new(),
        }
    }

    /// codec 등록. 같은 TypeNum이 이미 있으면 덮어씁니다.
    pub fn register(&mut self, codec: Box<dyn TypeCodec>) {
        let type_num = codec.type_num();
        self.codecs.insert(type_num, codec);
    }

    /// TypeNum으로 codec 조회
    pub fn get(&self, type_num: TypeNum) -> Result<&dyn TypeCodec, CodecError> {
        self.codecs
            .get(&type_num)
            .map(|c| c.as_ref())
            .ok_or(CodecError::UnsupportedType(type_num))
    }

    /// 기본 codec 전부 등록된 레지스트리 생성
    pub fn default_registry() -> Self {
        todo!("모든 기본 codec 인스턴스 등록")
    }
}
