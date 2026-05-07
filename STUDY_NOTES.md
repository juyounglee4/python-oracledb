# python-oracledb 스터디 노트

> Oracle Database용 Python DB-API 2.0 드라이버 (cx_Oracle 후속).
> Cython으로 빌드되며, **Thin 모드**(기본, Oracle Client 불필요)와 **Thick 모드**(Oracle Client 라이브러리 사용) 두 가지를 지원.

---

## 1. 전체 아키텍처

### 1.1 계층 구조

```
┌─────────────────────────────────────────────────────────────┐
│      Public API (Pure Python)                                │
│   connection.py, cursor.py, pool.py, var.py, lob.py ...     │
│                    ↓ self._impl                              │
├─────────────────────────────────────────────────────────────┤
│          base_impl.pyx (공통 베이스 클래스)                   │
│   BaseConnImpl, BaseCursorImpl, BaseVarImpl ...              │
├────────────────────────┬────────────────────────────────────┤
│    thin_impl.pyx       │        thick_impl.pyx              │
│  (Oracle 프로토콜을    │   (ODPI-C 라이브러리를 통해         │
│   순수 Python/Cython   │    Oracle Client에 위임)            │
│   으로 직접 구현)       │                                    │
└────────────────────────┴────────────────────────────────────┘
```

### 1.2 핵심 설계 원리

- **Thin 모드** (기본): Oracle Client 설치 없이 TNS 프로토콜을 Python/Cython으로 직접 구현
- **Thick 모드**: `oracledb.init_oracle_client()` 호출 시 활성화, C 라이브러리(ODPI-C) 사용
- **모드 전환**은 프로세스 내에서 한 번만 가능 — [driver_mode.py](src/oracledb/driver_mode.py)의 `DriverModeManager`가 관리
- Public API 클래스는 `self._impl`로 구현체를 참조 → 모드에 따라 thin/thick impl이 주입됨

### 1.3 Cython 확장 모듈 (4개)

[setup.py](setup.py)에서 정의:

| Extension | 소스 | 역할 |
|-----------|------|------|
| `oracledb.base_impl` | `base_impl.pyx` + `impl/base/*.pyx` | 공통 베이스 (타입, 인코딩, 버퍼, 파싱) |
| `oracledb.thin_impl` | `thin_impl.pyx` + `impl/thin/*.pyx` | Thin 모드 (TNS 프로토콜, 패킷, 암호화) |
| `oracledb.thick_impl` | `thick_impl.pyx` + `impl/thick/*.pyx` + ODPI-C | Thick 모드 (ODPI-C FFI) |
| `oracledb.arrow_impl` | `arrow_impl.pyx` + `impl/arrow/*.pyx` + nanoarrow | Apache Arrow/DataFrame 지원 |

> 각 최상위 `.pyx`는 `include` 지시문으로 `impl/` 하위의 `.pyx`들을 인라인 포함.

### 1.4 디렉터리 구성

#### `src/oracledb/` — Public API (Pure Python)

| 파일 | 역할 |
|------|------|
| [connection.py](src/oracledb/connection.py) | `Connection`, `AsyncConnection`, `connect()` |
| [cursor.py](src/oracledb/cursor.py) | `Cursor`, `AsyncCursor` |
| [pool.py](src/oracledb/pool.py) | `ConnectionPool`, `create_pool()` |
| [var.py](src/oracledb/var.py) | `Var` (바인드/페치 변수) |
| [lob.py](src/oracledb/lob.py) | `LOB`, `AsyncLOB` |
| [dbobject.py](src/oracledb/dbobject.py) | `DbObject`, `DbObjectType` |
| [soda.py](src/oracledb/soda.py) | SODA (NoSQL) API |
| [subscr.py](src/oracledb/subscr.py) | DB 변경 알림 |
| [pipeline.py](src/oracledb/pipeline.py) | 파이프라인 작업 |
| [dataframe.py](src/oracledb/dataframe.py) | DataFrame 통합 |
| [connect_params.py](src/oracledb/connect_params.py) | `ConnectParams` |
| [pool_params.py](src/oracledb/pool_params.py) | `PoolParams` |
| [defaults.py](src/oracledb/defaults.py) | 전역 기본 동작 (`fetch_lobs`, `fetch_decimals` 등) |
| [fetch_info.py](src/oracledb/fetch_info.py) | `FetchInfo` 컬럼 메타데이터 |
| [driver_mode.py](src/oracledb/driver_mode.py) | thin/thick 모드 선택 관리 |

#### `src/oracledb/impl/` — Cython 구현체

```
impl/
├── base/          # 공통 구현 (27개 .pyx)
│   ├── connection.pyx, cursor.pyx, var.pyx, pool.pyx
│   ├── buffer.pyx, encoders.pyx, decoders.pyx
│   ├── types.pyx, vector.pyx, oson.pyx
│   └── ...
├── thin/          # Thin 전용 (19개 .pyx + messages/)
│   ├── connection.pyx, cursor.pyx, pool.pyx
│   ├── protocol.pyx, packet.pyx, transport.pyx
│   ├── crypto.pyx, capabilities.pyx
│   └── messages/  (26개 메시지 타입)
├── thick/         # Thick 전용 (13개 .pyx)
│   ├── connection.pyx, cursor.pyx, pool.pyx
│   └── odpi.pxd   (ODPI-C 헤더 선언)
└── arrow/         # Arrow 통합
    ├── array.pyx, dataframe.pyx, schema.pyx
    └── nanoarrow/  (내장 nanoarrow C 라이브러리)
```

### 1.5 기타 특징

- **Async 지원**: `AsyncConnection`, `AsyncCursor` — thin 모드에서 asyncio 지원
- **템플릿 코드 생성**: `connection.py`, `pool.py`는 [utils/build_from_template.py](utils/build_from_template.py)로 sync/async 동시 생성
- **플러그인**: [src/oracledb/plugins/](src/oracledb/plugins/) — Azure/OCI 인증 및 설정 제공자
- **빌드 의존성**: Cython ~3.2, setuptools >= 77 / 런타임: `cryptography` + `typing_extensions`
- **지원 Python**: 3.9 ~ 3.14 (CPython 전용)

---

## 2. Python ↔ DB 타입 캐스팅

### 2.1 변환 파이프라인

#### 페치 (DB → Python)

```
Oracle Wire Format
    ↓ decoders.pyx (decode_number, decode_date, decode_binary_double 등)
OracleDataBuffer (내부 중간 형식)
    ↓ converters.pyx :: convert_oracle_data_to_python()
Python 객체 (float, int, str, datetime 등)
    ↓ outconverter (있으면)
최종 Python 값
```

#### 바인드 (Python → DB)

```
Python 값
    ↓ inconverter (있으면)
    ↓ connection._check_value()  — 타입 검증 + 자동 변환
    ↓ converters.pyx :: convert_python_to_oracle_data()
OracleDataBuffer
    ↓ encoders.pyx (encode_number, encode_date 등)
Oracle Wire Format
```

### 2.2 기본 타입 매핑

[types.pyx](src/oracledb/impl/base/types.pyx)에서 `DbType` 생성 시 `default_py_type_num`으로 결정.

| Oracle Type | PY_TYPE_NUM | Python Type |
|---|---|---|
| `NUMBER` | `FLOAT` (2) | `float` (scale=0이면 `int`) |
| `BINARY_DOUBLE` / `BINARY_FLOAT` | `FLOAT` | `float` |
| `BINARY_INTEGER` | `INT` (3) | `int` |
| `VARCHAR2`, `CHAR`, `NVARCHAR2`, `NCHAR`, `LONG` | `STR` (5) | `str` |
| `RAW`, `LONG RAW` | `BYTES` (10) | `bytes` |
| `DATE`, `TIMESTAMP`, `TIMESTAMP WITH TZ/LTZ` | `DATETIME` (7) | `datetime.datetime` |
| `INTERVAL DAY TO SECOND` | `TIMEDELTA` (8) | `datetime.timedelta` |
| `INTERVAL YEAR TO MONTH` | `ORACLE_INTERVAL_YM` (12) | `oracledb.IntervalYM` |
| `BOOLEAN` | `BOOL` (4) | `bool` |
| `CLOB`, `NCLOB`, `BLOB`, `BFILE` | `ORACLE_LOB` (1) | `oracledb.LOB` |
| `CURSOR` | `ORACLE_CURSOR` (6) | `oracledb.Cursor` |
| `JSON` | `OBJECT` (9) | `dict` / `list` 등 |
| `VECTOR` | `ARRAY` (13) | `array.array` |
| `OBJECT` | `ORACLE_OBJECT` (11) | `oracledb.DbObject` |

#### NUMBER의 특수 규칙

[metadata.pyx#L131-L137](src/oracledb/impl/base/metadata.pyx):
- `scale == 0` 또는 `(scale == -127 and precision == 0)` → `int`
- 그 외 → `float`

### 2.3 Python 값 → 바인드 타입 결정

[metadata.pyx#L411-L457](src/oracledb/impl/base/metadata.pyx) `OracleMetadata.from_value()`:

| Python 값 | 선택되는 DB_TYPE |
|---|---|
| `None` | `DB_TYPE_VARCHAR` (size=1) |
| `bool` | `DB_TYPE_BOOLEAN` |
| `str` | `DB_TYPE_VARCHAR` |
| `bytes` | `DB_TYPE_RAW` |
| `int` | `DB_TYPE_NUMBER` (py_type=INT) |
| `float` | `DB_TYPE_NUMBER` (py_type=FLOAT) |
| `Decimal` | `DB_TYPE_NUMBER` (py_type=DECIMAL) |
| `date` / `datetime` | `DB_TYPE_DATE` |
| `timedelta` | `DB_TYPE_INTERVAL_DS` |
| `DbObject` | `DB_TYPE_OBJECT` |
| `LOB` | LOB의 `type` 속성 |
| `Cursor` | `DB_TYPE_CURSOR` |
| `array.array` / `SparseVector` | `DB_TYPE_VECTOR` |

### 2.4 `_check_value()` — 바인드 시 자동 변환

[connection.pyx#L39-L165](src/oracledb/impl/base/connection.pyx) 주요 동작:

- `BINARY_FLOAT/DOUBLE` 대상 → `float()` 강제 변환
- `BINARY_INTEGER` 대상 → `int()` 강제 변환
- `VARCHAR/CHAR` 대상에 `bytes` → `.decode()` 자동 호출
- `RAW` 대상에 `str` → `.encode()` 자동 호출
- `BOOLEAN` 대상 → `bool()` 강제 변환
- `BLOB` 대상에 문자열 → 임시 LOB 생성
- `VECTOR` 대상에 `list` → `array.array('d', value)` 변환

### 2.5 Type Handler 메커니즘

#### Output Type Handler (페치 시)

- `cursor.outputtypehandler` 또는 `connection.outputtypehandler`에 설정
- [cursor.pyx#L146-L240](src/oracledb/impl/base/cursor.pyx)의 `_create_fetch_var()`에서 호출
- 두 가지 시그니처:
  - **신규**: `handler(cursor, fetch_info)` — `FetchInfo` 객체 전달
  - **레거시**: `handler(cursor, name, dbtype, size, precision, scale)`
- `cursor.var()`로 생성한 `Var` 반환 시 해당 타입으로 페치
- `None` 반환 시 기본 처리

#### Input Type Handler (바인드 시)

- `cursor.inputtypehandler` / `connection.inputtypehandler`
- 바인드 전 Python 값의 DB 타입 결정을 오버라이드

#### `Var`의 inconverter / outconverter

- `var._impl.inconverter`: 바인드 전 Python→Python 변환 함수
- `var._impl.outconverter`: 페치 후 Python→Python 변환 함수
- `var.convert_nulls`: NULL도 outconverter에 통과시킬지 여부

### 2.6 호환 변환 (`_check_fetch_conversion`)

[var.pyx#L113-L250](src/oracledb/impl/base/var.pyx) — output type handler가 DB와 다른 타입의 `Var`를 반환했을 때 호환성 체크. 예:

- `NUMBER` ↔ `VARCHAR`
- `CLOB` → `VARCHAR/LONG`  (문자열로 직접 읽기)
- `BLOB` → `RAW/LONG RAW`
- `DATE/TIMESTAMP` ↔ `VARCHAR`

### 2.7 Defaults에 의한 자동 조정

[cursor.pyx#L210-L240](src/oracledb/impl/base/cursor.pyx) `_create_fetch_var()`:

- `defaults.fetch_decimals = True` → NUMBER를 `decimal.Decimal`로 반환
- `defaults.fetch_lobs = False` → BLOB→`bytes`, CLOB→`str` 직접 반환 (LOB 객체 대신)
- `is_oson` 컬럼 → `decode_oson` outconverter 자동 설정
- `is_json` 컬럼 → JSON converter 자동 설정

### 2.8 핵심 파일 정리

| 파일 | 역할 |
|------|------|
| [src/oracledb/impl/base/types.pyx](src/oracledb/impl/base/types.pyx) | `DbType` 정의 + 모든 Oracle 타입 상수 + 기본 Python 타입 매핑 |
| [src/oracledb/impl/base/metadata.pyx](src/oracledb/impl/base/metadata.pyx) | `OracleMetadata.from_type()`, `from_value()` |
| [src/oracledb/impl/base/converters.pyx](src/oracledb/impl/base/converters.pyx) | 변환 엔진 핵심 (`convert_oracle_data_to_python`, `convert_python_to_oracle_data`) |
| [src/oracledb/impl/base/decoders.pyx](src/oracledb/impl/base/decoders.pyx) | wire format → 내부 버퍼 디코딩 |
| [src/oracledb/impl/base/encoders.pyx](src/oracledb/impl/base/encoders.pyx) | 내부 버퍼 → wire format 인코딩 |
| [src/oracledb/impl/base/var.pyx](src/oracledb/impl/base/var.pyx) | `BaseVarImpl` + 타입 호환성 체크 |
| [src/oracledb/impl/base/connection.pyx](src/oracledb/impl/base/connection.pyx) | `_check_value()` (바인드 자동 변환) |
| [src/oracledb/impl/base/cursor.pyx](src/oracledb/impl/base/cursor.pyx) | `_create_fetch_var()` (페치 변수 생성, type handler 호출) |
| [src/oracledb/var.py](src/oracledb/var.py) | `Var` 공개 API |
| [src/oracledb/fetch_info.py](src/oracledb/fetch_info.py) | `FetchInfo` 컬럼 메타데이터 |
| [src/oracledb/defaults.py](src/oracledb/defaults.py) | `fetch_lobs`, `fetch_decimals` 등 전역 기본값 |

---

## 3. 공부 시작 추천 경로

1. **API 사용감 먼저 파악**: `samples/` 폴더의 예제 (특히 [samples/query.py](samples/query.py), [samples/bind_insert.py](samples/bind_insert.py))
2. **Public API 흐름 따라가기**: [connection.py](src/oracledb/connection.py) `connect()` → `_impl` 생성 흐름
3. **모드 분기점**: [driver_mode.py](src/oracledb/driver_mode.py)
4. **타입 변환 양방향 진입점**:
   - 페치: `converters.pyx`의 `convert_oracle_data_to_python()`
   - 바인드: `metadata.pyx`의 `OracleMetadata.from_value()`
5. **Thin 프로토콜 메시지**: `impl/thin/messages/` 디렉터리의 execute, fetch, auth 등
