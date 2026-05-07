# python-oracledb Thin Mode 아키텍처 문서

> **대상 독자**: 프로젝트에 처음 합류하는 신입 개발자  
> **범위**: Thin 모드 한정 (Oracle Client 라이브러리 없이 직접 TNS 프로토콜 통신)

---

## 1. 프로젝트 개요

`python-oracledb`는 Python에서 Oracle Database에 접속하기 위한 드라이버입니다.  
[PEP 249 (Python DB-API 2.0)](https://peps.python.org/pep-0249/) 표준을 준수하며, **Cython**으로 작성되어 네이티브 C 확장 모듈로 컴파일됩니다.

### 1.1 Thin 모드란?

- Oracle Client 라이브러리(OCI) **없이** 동작
- 순수 Python/Cython으로 TNS(Transparent Network Substrate) 프로토콜을 직접 구현
- 설치 의존성이 최소화되어 배포가 간편

```mermaid
graph LR
    A[Python Application] --> B[python-oracledb<br/>Thin Mode]
    B -->|TNS Protocol<br/>over TCP/TLS| C[Oracle Database]
    
    style B fill:#f9f,stroke:#333,stroke-width:2px
```

---

## 2. 계층 구조 (Layered Architecture)

프로젝트는 명확한 **3계층 구조**로 설계되어 있습니다.

```mermaid
graph TD
    subgraph "Layer 1: Public API (Pure Python)"
        A1[connection.py]
        A2[cursor.py]
        A3[pool.py]
        A4[connect_params.py]
        A5[lob.py / var.py / ...]
    end

    subgraph "Layer 2: Base Implementation (Cython)"
        B1[base_impl.pyx]
        B2["impl/base/*.pyx<br/>(BaseConnImpl, BaseCursorImpl, ...)"]
    end

    subgraph "Layer 3: Thin Implementation (Cython)"
        C1[thin_impl.pyx]
        C2["impl/thin/*.pyx<br/>(Protocol, Transport, Messages, ...)"]
    end

    subgraph "Network"
        D1[Oracle Database<br/>TNS Listener]
    end

    A1 --> B1
    A2 --> B1
    A3 --> B1
    B1 --> C1
    C1 -->|TCP/TLS Socket| D1

    style A1 fill:#e1f5fe
    style A2 fill:#e1f5fe
    style A3 fill:#e1f5fe
    style B1 fill:#fff3e0
    style C1 fill:#fce4ec
```

| 계층 | 역할 | 파일 위치 |
|------|------|-----------|
| **Public API** | 사용자에게 노출되는 Python 클래스 (`Connection`, `Cursor`, `Pool`) | `src/oracledb/*.py` |
| **Base Impl** | 공통 로직 (바인딩, 타입 변환, 파싱 등). Thin/Thick 모두 공유 | `src/oracledb/base_impl.pyx` → `impl/base/` |
| **Thin Impl** | TNS 프로토콜 직접 구현 (소켓, 패킷, 메시지) | `src/oracledb/thin_impl.pyx` → `impl/thin/` |

---

## 3. Cython 빌드 구조

Cython `.pyx` 파일은 `include` 문을 통해 여러 파일을 **하나의 컴파일 단위**로 합칩니다.

```mermaid
graph TD
    subgraph "base_impl.pyx (하나의 .so로 컴파일)"
        BI[base_impl.pyx] -->|include| BI1[impl/base/connection.pyx]
        BI -->|include| BI2[impl/base/cursor.pyx]
        BI -->|include| BI3[impl/base/buffer.pyx]
        BI -->|include| BI4[impl/base/converters.pyx]
        BI -->|include| BI5[impl/base/parsers.pyx]
        BI -->|include| BI6["... (27개 파일)"]
    end

    subgraph "thin_impl.pyx (하나의 .so로 컴파일)"
        TI[thin_impl.pyx] -->|include| TI1[impl/thin/protocol.pyx]
        TI -->|include| TI2[impl/thin/transport.pyx]
        TI -->|include| TI3[impl/thin/packet.pyx]
        TI -->|include| TI4[impl/thin/connection.pyx]
        TI -->|include| TI5[impl/thin/cursor.pyx]
        TI -->|include| TI6[impl/thin/messages/*.pyx]
        TI -->|include| TI7["... (19개 파일)"]
    end

    BI -.->|cimport| TI

    style BI fill:#fff3e0
    style TI fill:#fce4ec
```

> **핵심 포인트**: `impl/thin/connection.pyx`는 독립 파일이 아니라 `thin_impl.pyx` 안에 포함(embed)됩니다.  
> IDE에서 단독으로 열면 심볼 해석이 안 될 수 있습니다.

---

## 4. 핵심 클래스 다이어그램

### 4.1 Connection 계층

```mermaid
classDiagram
    class Connection {
        <<Python - Public API>>
        +_impl: BaseConnImpl
        +connect(dsn, params)
        +cursor() Cursor
        +commit()
        +rollback()
        +close()
    }

    class BaseConnImpl {
        <<Cython - base_impl>>
        +dsn: str
        +username: str
        +autocommit: bool
        +create_cursor_impl()
        +commit()
        +rollback()
    }

    class BaseThinConnImpl {
        <<Cython - thin_impl>>
        +_protocol: BaseProtocol
        +_statement_cache: StatementCache
        +_connect_params: ConnectParamsImpl
        +connect()
        +_create_message(type)
    }

    class ThinConnImpl {
        <<Cython - thin_impl>>
        +connect()
        +commit()
        +ping()
    }

    class AsyncThinConnImpl {
        <<Cython - thin_impl>>
        +connect()
        +commit()
        +ping()
    }

    Connection --> BaseConnImpl : _impl
    BaseConnImpl <|-- BaseThinConnImpl
    BaseThinConnImpl <|-- ThinConnImpl
    BaseThinConnImpl <|-- AsyncThinConnImpl
```

### 4.2 Cursor 계층

```mermaid
classDiagram
    class Cursor {
        <<Python - Public API>>
        +_impl: BaseCursorImpl
        +execute(sql, params)
        +executemany(sql, params_list)
        +fetchone()
        +fetchmany()
        +fetchall()
    }

    class BaseCursorImpl {
        <<Cython - base_impl>>
        +bind_vars: list
        +fetch_vars: list
        +rowcount: uint64
        +arraysize: uint32
        +_bind_values()
        +_fetch_rows()
    }

    class BaseThinCursorImpl {
        <<Cython - thin_impl>>
        +_conn_impl: BaseThinConnImpl
        +_statement: Statement
        +_create_message(type)
        +execute()
        +fetch()
    }

    class ThinCursorImpl {
        <<Cython - thin_impl>>
        +execute()
        +fetch_next_rows()
    }

    Cursor --> BaseCursorImpl : _impl
    BaseCursorImpl <|-- BaseThinCursorImpl
    BaseThinCursorImpl <|-- ThinCursorImpl
```

### 4.3 네트워크 계층

```mermaid
classDiagram
    class BaseProtocol {
        <<Cython - thin_impl>>
        +_transport: Transport
        +_caps: Capabilities
        +_read_buf: ReadBuffer
        +_write_buf: WriteBuffer
        +_process_message(Message)
    }

    class Transport {
        <<Cython - thin_impl>>
        +_transport: socket/asyncio
        +_ssl_context: ssl.SSLContext
        +_max_packet_size: uint32
        +connect()
        +disconnect()
        +send_oob_break()
    }

    class ReadBuffer {
        <<Cython - thin_impl>>
        +_caps: Capabilities
        +read_uint16be()
        +read_uint32be()
        +read_raw_bytes()
        +wait_for_packets_sync()
    }

    class WriteBuffer {
        <<Cython - thin_impl>>
        +start_request()
        +end_request()
        +write_ub4()
        +write_bytes()
    }

    class Capabilities {
        <<Cython - thin_impl>>
        +sdu: uint32
        +supports_oob: bool
        +ttc_field_version: uint8
    }

    BaseProtocol --> Transport
    BaseProtocol --> ReadBuffer
    BaseProtocol --> WriteBuffer
    BaseProtocol --> Capabilities
    ReadBuffer --> Transport
    WriteBuffer --> Transport
```

### 4.4 메시지 시스템

```mermaid
classDiagram
    class Message {
        <<Cython - thin_impl>>
        +conn_impl: BaseThinConnImpl
        +message_type: uint8
        +function_code: uint8
        +error_occurred: bool
        +send(WriteBuffer)
        +process(ReadBuffer)
    }

    class MessageWithData {
        <<Cython - thin_impl>>
        +cursor_impl: BaseThinCursorImpl
        +cursor: Cursor
    }

    class ConnectMessage {
        +connect_string_bytes
        +description: Description
    }

    class AuthMessage {
        +password: bytes
        +session_data: dict
        +auth_mode: uint32
    }

    class ExecuteMessage {
        +num_execs: uint32
        +_write_execute_message()
    }

    class FetchMessage {
        +_write_message()
    }

    class CommitMessage
    class RollbackMessage
    class LogoffMessage
    class PingMessage

    Message <|-- MessageWithData
    Message <|-- ConnectMessage
    Message <|-- AuthMessage
    Message <|-- CommitMessage
    Message <|-- RollbackMessage
    Message <|-- LogoffMessage
    Message <|-- PingMessage
    MessageWithData <|-- ExecuteMessage
    MessageWithData <|-- FetchMessage
```

---

## 5. 연결(Connection) 워크플로우

### 5.1 연결 수립 과정

```mermaid
sequenceDiagram
    participant App as Python App
    participant Conn as Connection (API)
    participant Mode as DriverModeManager
    participant ThinConn as BaseThinConnImpl
    participant Proto as BaseProtocol
    participant Trans as Transport
    participant DB as Oracle DB

    App->>Conn: oracledb.connect(dsn, user, password)
    Conn->>Mode: get_manager(thin=True)
    Mode-->>Conn: mode confirmed (thin)
    Conn->>ThinConn: ThinConnImpl(dsn, params)
    
    ThinConn->>Proto: BaseProtocol()
    Proto->>Trans: Transport()
    
    Note over ThinConn,DB: Phase 1: TCP/TLS 연결
    ThinConn->>Trans: connect(host, port)
    Trans->>DB: TCP 3-way Handshake
    Trans->>DB: TLS Handshake (optional)
    DB-->>Trans: Connection Accepted
    
    Note over ThinConn,DB: Phase 2: TNS Connect 협상
    ThinConn->>Proto: send ConnectMessage
    Proto->>DB: TNS CONNECT packet
    DB-->>Proto: TNS ACCEPT packet
    Proto-->>ThinConn: Protocol negotiated (SDU, version)
    
    Note over ThinConn,DB: Phase 3: 인증
    ThinConn->>Proto: send AuthMessage (Phase 1)
    Proto->>DB: AUTH request
    DB-->>Proto: AUTH challenge (verifier)
    ThinConn->>Proto: send AuthMessage (Phase 2)
    Proto->>DB: Encrypted credentials
    DB-->>Proto: AUTH success + session data
    
    Proto-->>ThinConn: Session established
    ThinConn-->>Conn: _impl = ThinConnImpl
    Conn-->>App: Connection object
```

### 5.2 SQL 실행 워크플로우

```mermaid
sequenceDiagram
    participant App as Python App
    participant Cur as Cursor (API)
    participant CurImpl as ThinCursorImpl
    participant Stmt as Statement
    participant Proto as Protocol
    participant DB as Oracle DB

    App->>Cur: cursor.execute("SELECT * FROM emp WHERE id = :1", [100])
    Cur->>CurImpl: execute(sql, params)
    
    Note over CurImpl,Stmt: SQL 파싱 & 바인드 변수 처리
    CurImpl->>Stmt: StatementParser.parse(sql)
    Stmt-->>CurImpl: Statement (bind_info_list)
    CurImpl->>CurImpl: _bind_values(params)
    
    Note over CurImpl,DB: 실행 메시지 전송
    CurImpl->>CurImpl: _create_execute_message()
    CurImpl->>Proto: send ExecuteMessage
    Proto->>DB: TNS DATA (Execute + Define + Fetch)
    
    Note over CurImpl,DB: 결과 수신
    DB-->>Proto: TNS DATA (Column metadata + Rows)
    Proto-->>CurImpl: rows stored in fetch_vars
    CurImpl-->>Cur: execution complete
    
    App->>Cur: cursor.fetchone()
    Cur->>CurImpl: _fetch_row()
    CurImpl-->>Cur: row data (Python objects)
    Cur-->>App: (row tuple)
```

### 5.3 Connection Pool 워크플로우

```mermaid
sequenceDiagram
    participant App as Python App
    participant Pool as ConnectionPool (API)
    participant PoolImpl as ThinPoolImpl
    participant BG as Background Task
    participant DB as Oracle DB

    App->>Pool: oracledb.create_pool(min=2, max=10)
    Pool->>PoolImpl: ThinPoolImpl(dsn, params)
    PoolImpl->>BG: Start background task
    
    loop min(2)번 반복
        BG->>DB: 연결 생성
        DB-->>BG: Connection established
        BG->>PoolImpl: _free_new_conn_impls.append(conn)
    end

    App->>Pool: pool.acquire()
    Pool->>PoolImpl: acquire()
    PoolImpl->>PoolImpl: _free_used_conn_impls 또는 _free_new_conn_impls에서 꺼냄
    PoolImpl-->>Pool: conn_impl
    Pool-->>App: Connection object

    App->>Pool: pool.release(conn)
    Pool->>PoolImpl: release(conn_impl)
    PoolImpl->>PoolImpl: ping 검사 후 _free_used_conn_impls에 반환
```

---

## 6. 패킷 구조 (TNS Protocol)

```mermaid
graph TD
    subgraph "TNS Packet Structure"
        H[Packet Header<br/>8 bytes]
        D[Data Payload]
    end

    subgraph "Header Fields"
        H1[Packet Size - 4 bytes]
        H2[Packet Type - 1 byte]
        H3[Packet Flags - 1 byte]
        H4[Header Checksum - 2 bytes]
    end

    subgraph "주요 Packet Types"
        T1["CONNECT (1)"]
        T2["ACCEPT (2)"]
        T3["REFUSE (4)"]
        T4["DATA (6)"]
        T5["REDIRECT (5)"]
        T6["MARKER (12)"]
    end

    H --> H1
    H --> H2
    H --> H3
    H --> H4
```

### Data Packet 내부 구조

```mermaid
graph LR
    subgraph "DATA Packet"
        DF[Data Flags<br/>2 bytes]
        MT[Message Type<br/>1 byte]
        MB[Message Body<br/>variable]
    end

    subgraph "Message Types"
        M1["Execute (3)"]
        M2["Fetch (5)"]
        M3["Commit (14)"]
        M4["Rollback (15)"]
        M5["Auth (8)"]
        M6["Logoff (9)"]
    end
```

---

## 7. 디렉토리 구조 요약

```
src/oracledb/
├── __init__.py              # 패키지 진입점 (public exports)
├── connection.py            # Connection 클래스 (Public API)
├── cursor.py                # Cursor 클래스 (Public API)
├── pool.py                  # ConnectionPool 클래스 (Public API)
├── connect_params.py        # 연결 파라미터 (Public API)
├── driver_mode.py           # Thin/Thick 모드 관리
├── base.py                  # BaseMetaClass
├── errors.py                # 에러 코드 정의
├── exceptions.py            # Exception 클래스
│
├── base_impl.pyx            # ★ Base 구현 (Cython 엔트리포인트)
├── thin_impl.pyx            # ★ Thin 구현 (Cython 엔트리포인트)
│
├── impl/
│   ├── base/                # Base 구현 모듈들
│   │   ├── connection.pyx   #   BaseConnImpl
│   │   ├── cursor.pyx       #   BaseCursorImpl
│   │   ├── buffer.pyx       #   Buffer, GrowableBuffer
│   │   ├── converters.pyx   #   데이터 타입 변환
│   │   ├── parsers.pyx      #   SQL 파서
│   │   ├── connect_params.pyx  # ConnectParamsImpl
│   │   └── ...
│   │
│   └── thin/                # Thin 구현 모듈들
│       ├── connection.pyx   #   BaseThinConnImpl, ThinConnImpl
│       ├── cursor.pyx       #   BaseThinCursorImpl, ThinCursorImpl
│       ├── protocol.pyx     #   BaseProtocol (요청/응답 관리)
│       ├── transport.pyx    #   Transport (소켓 I/O)
│       ├── packet.pyx       #   Packet, ReadBuffer, WriteBuffer
│       ├── pool.pyx         #   BaseThinPoolImpl, ThinPoolImpl
│       ├── statement.pyx    #   Statement, BindInfo
│       ├── statement_cache.pyx  # StatementCache
│       ├── capabilities.pyx #   Capabilities (서버 기능 협상)
│       ├── crypto.pyx       #   암호화 유틸리티
│       └── messages/        #   ★ TNS 메시지 정의
│           ├── base.pyx     #     Message, MessageWithData 기본 클래스
│           ├── connect.pyx  #     ConnectMessage
│           ├── auth.pyx     #     AuthMessage (인증)
│           ├── execute.pyx  #     ExecuteMessage (SQL 실행)
│           ├── fetch.pyx    #     FetchMessage (데이터 조회)
│           ├── commit.pyx   #     CommitMessage
│           ├── rollback.pyx #     RollbackMessage
│           └── ...          #     (26개 메시지 타입)
```

---

## 8. 핵심 설계 패턴

### 8.1 Impl 패턴 (Bridge Pattern)

Public API 클래스는 구현을 `_impl` 속성에 위임합니다. 이를 통해 Thin/Thick 모드를 런타임에 선택할 수 있습니다.

```mermaid
graph LR
    subgraph "Public API"
        A[Connection]
    end

    subgraph "Implementation"
        B[BaseConnImpl]
        C[BaseThinConnImpl]
        D[ThinConnImpl]
        E[ThickConnImpl]
    end

    A -->|_impl| B
    B --> C
    B --> E
    C --> D

    style E fill:#ccc,stroke:#999,stroke-dasharray: 5 5
```

### 8.2 Message 패턴 (Command Pattern)

모든 데이터베이스 요청은 `Message` 객체로 캡슐화됩니다.

```python
# 의사 코드 (개념 설명용)
message = conn_impl._create_message(ExecuteMessage)
message.send(write_buf)      # 요청 직렬화 → 네트워크 전송
message.process(read_buf)    # 응답 수신 → 역직렬화
```

### 8.3 Statement Cache

동일 SQL을 반복 실행 시 파싱 비용을 절약합니다.

```mermaid
graph TD
    A[cursor.execute SQL] --> B{Statement Cache에 존재?}
    B -->|Yes| C[캐시된 Statement 재사용]
    B -->|No| D[새 Statement 생성 + 서버 파싱]
    D --> E[캐시에 저장]
    C --> F[Execute Message 전송]
    E --> F
```

---

## 9. 데이터 타입 변환 흐름

> **상세 문서**: 소스 코드 레벨의 IN/OUT bind 변환 전체 분석은 [DATA_TYPE_CONVERSION.md](DATA_TYPE_CONVERSION.md)를 참조하세요.

```mermaid
graph LR
    subgraph "Python → Oracle (Bind)"
        P1[Python int/str/datetime]
        P2[convert_python_to_oracle_data]
        P3[OracleData struct]
        P4[Encode to TNS bytes]
    end

    subgraph "Oracle → Python (Fetch)"
        O1[TNS bytes from server]
        O2[Decode to OracleData]
        O3[convert_oracle_data_to_python]
        O4[Python int/str/datetime]
    end

    P1 --> P2 --> P3 --> P4
    O1 --> O2 --> O3 --> O4
```

---

## 10. 개발 환경 관련

### 빌드 명령

```bash
# Cython 확장 빌드
python setup.py build_ext --inplace

# 디버그 빌드 (cygdb 디버깅용)
rm -rf cython_debug
python -m cython --gdb src/oracledb/base_impl.pyx
python -m cython --gdb src/oracledb/thin_impl.pyx
PYO_COMPILE_ARGS="-O0 -g3" python setup.py build_ext --inplace --force
```

### 주요 환경변수

| 변수 | 용도 |
|------|------|
| `PYO_DEBUG_PACKETS` | TNS 패킷 디버그 출력 |
| `PYO_SAMPLES_DRIVER_MODE` | 샘플 실행 모드 (`thin`/`thick`) |

---

## 11. 요약 다이어그램 (전체 흐름)

```mermaid
graph TB
    subgraph "Application Layer"
        APP[Python Application]
    end

    subgraph "Public API Layer (Pure Python)"
        CONN[Connection]
        CUR[Cursor]
        POOL[ConnectionPool]
        PARAMS[ConnectParams]
    end

    subgraph "Base Implementation (Cython - base_impl.so)"
        BCONN[BaseConnImpl]
        BCUR[BaseCursorImpl]
        BPOOL[BasePoolImpl]
        CONV[Converters / Encoders / Decoders]
        PARSER[SQL Parser]
        BUF[Buffer]
    end

    subgraph "Thin Implementation (Cython - thin_impl.so)"
        TCONN[BaseThinConnImpl → ThinConnImpl]
        TCUR[BaseThinCursorImpl → ThinCursorImpl]
        TPOOL[BaseThinPoolImpl → ThinPoolImpl]
        PROTO[Protocol]
        TRANS[Transport]
        MSGS[Messages<br/>Connect / Auth / Execute / Fetch / ...]
        STMTC[Statement Cache]
        PKT[Packet / ReadBuffer / WriteBuffer]
    end

    subgraph "Network"
        TCP[TCP/TLS Socket]
    end

    subgraph "Database"
        ORA[(Oracle Database)]
    end

    APP --> CONN & CUR & POOL
    CONN --> BCONN --> TCONN
    CUR --> BCUR --> TCUR
    POOL --> BPOOL --> TPOOL
    TCONN --> PROTO
    TCUR --> MSGS
    PROTO --> TRANS --> PKT --> TCP --> ORA
    TCONN --> STMTC
    BCUR --> CONV
    BCUR --> PARSER

    style APP fill:#c8e6c9
    style CONN fill:#e1f5fe
    style CUR fill:#e1f5fe
    style POOL fill:#e1f5fe
    style BCONN fill:#fff3e0
    style BCUR fill:#fff3e0
    style TCONN fill:#fce4ec
    style TCUR fill:#fce4ec
    style PROTO fill:#fce4ec
    style ORA fill:#ffecb3
```

---

## 12. 다음 단계

1. `samples/` 디렉토리의 예제를 실행하여 드라이버 동작을 체험
2. `impl/thin/messages/execute.pyx`를 읽으며 SQL 실행의 직렬화 과정 이해
3. `impl/thin/protocol.pyx`에서 요청/응답 라이프사이클 추적
4. `impl/base/converters.pyx`에서 Oracle↔Python 타입 매핑 학습
