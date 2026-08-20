# Rust 시스템 학습 로드맵

## 목표

Rust 숙련 자체보다 컴퓨터 내부 동작을 코드로 확인하는 것을 우선한다. 개념을 먼저 모두 암기하지 않고 작은 구현에서 문제를 경험한 뒤 그 문제를 해결하는 기술과 추상화를 도입한다.

```text
개념 이해 → 작은 구현 → 실행 → 문제 재현 → 원인 분석 → 직접 수정 → 상위 추상화 → 실무 기술과 연결
```

## 프로젝트

### 1. Rust Basics — 완료

- 위치: [archive/rust-basics](archive/rust-basics/README.md)
- 범위: Rust 문법, Ownership, Borrowing, Struct, Enum, Error, Iterator, Collection, Generic, Trait, Lifetime, Cargo
- 완료 지점: Git tag `rust-basics-complete`

### 2. Network Lab — 현재

- 위치: [projects/network-lab](projects/network-lab/README.md)
- 정본: [projects/network-lab/CURRICULUM.md](projects/network-lab/CURRICULUM.md)
- 핵심 흐름: byte → buffer → blocking I/O → TCP stream → framing → protocol → concurrency → non-blocking I/O → async/Tokio

### 3. OS Foundations Lab — 예정

- Network Lab에서 사용한 socket, thread, blocking I/O를 OS 관점으로 확장한다.
- 핵심 범위: syscall, file descriptor, process와 thread, virtual memory, file I/O, scheduling, synchronization, signal과 process lifecycle
- 핵심 질문: user space와 kernel space 사이에서 데이터와 제어가 어떻게 이동하는가?
- 다음 연결: file I/O, page cache, memory mapping 이해를 Storage Engine Lab의 기반으로 사용한다.

### 4. Storage Engine Lab — 예정

- Network Lab의 in-memory Key-Value 결과물을 disk에 영속화한다.
- 핵심 흐름: byte와 file → fixed-size page → buffering/page cache → append-only log → partial write와 crash → recovery/WAL → index와 compaction
- 핵심 질문: write가 성공했을 때 데이터는 application buffer, kernel page cache, storage device 중 어디까지 도달했는가?
- 결과물: 작은 persistent Key-Value storage engine

### 5. Database Lab — 예정

- Storage Engine Lab 위에 record, index, query와 transaction을 추가한다.
- 핵심 범위: B-Tree 또는 LSM Tree, primary/secondary index, transaction, isolation, concurrency control, recovery
- 핵심 질문: 여러 작업이 동시에 실행되고 장애가 발생해도 데이터의 일관성과 지속성을 어떻게 보장하는가?
- 결과물: index와 transaction을 가진 작은 database

### 6. Distributed Systems Lab — 예정

- Network, concurrency, persistent storage와 failure handling을 여러 node로 확장한다.
- 핵심 흐름: timeout과 partial failure → retry와 duplicate execution → idempotency → replication → leader와 consistency → consensus 기초
- 핵심 질문: 일부 node와 network가 실패하는 상황에서 여러 복제본이 어떤 상태를 신뢰해야 하는가?
- 결과물: 앞선 database 또는 Key-Value engine을 이용한 작은 replicated system

### 7. 선택 심화 — 예정

앞선 프로젝트에서 생긴 관심과 질문에 따라 하나를 선택한다.

- Runtime/Concurrency Lab: executor, scheduler, Future/Waker, work stealing
- Compiler/Language Lab: lexer, parser, AST, type checking, bytecode와 VM
- Performance Lab: benchmark, profiling, allocation, cache locality, SIMD
- ML/LLM Systems Lab: tensor layout, matrix multiplication, batching, attention, inference와 quantization

## 프로젝트 연결 원칙

큰 프로젝트 순서는 미리 정하되 세부 단원은 직전 프로젝트를 완료한 시점에 확정한다. 다음 프로젝트는 가능하면 이전 결과물과 남은 질문을 이어받는다.

```text
TCP in-memory Key-Value server
    ↓
OS와 I/O 기반 이해
    ↓
persistent Key-Value storage
    ↓
index와 transaction을 가진 database
    ↓
replicated distributed database
```

각 프로젝트는 다음 학습 흐름을 따른다.

```text
낮은 수준의 기본 요소
→ 작은 구현
→ 실패와 한계 재현
→ OS/하드웨어 동작 관찰
→ 학습자의 수정
→ 상위 추상화 도입
→ 실제 생태계와 비교
```

각 단계에서는 다음 질문을 반복한다.

1. 데이터는 지금 어디에 있는가?
2. 상태는 누가 소유하고 변경하는가?
3. 작업은 어떤 thread, process, runtime 또는 OS가 실행하거나 기다리는가?

완료 여부는 특정 API나 framework 사용 여부가 아니라 자신의 구현과 관찰 결과로 시스템 원리와 trade-off를 설명할 수 있는지를 기준으로 판단한다. 프로젝트를 완료하면 다음 프로젝트를 자동으로 시작하지 않고, 학습자가 진행을 요청한 뒤 상세 커리큘럼과 진도 문서를 만든다.
