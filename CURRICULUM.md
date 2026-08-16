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

### 이후 후보

Network Lab 완료 후 필요와 관심도에 따라 범위를 다시 논의한다.

- OS Lab
- Data Structure Lab
- Database/Storage Lab
- Distributed Systems Lab
- ML/LLM internals Lab

프로젝트 이름과 순서는 미리 고정하지 않는다. 이전 프로젝트에서 드러난 질문과 실제 학습 목표를 바탕으로 다음 범위를 결정한다.
