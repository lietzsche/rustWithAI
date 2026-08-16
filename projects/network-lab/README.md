# Network Lab

Rust 표준 라이브러리에서 시작해 네트워크 스택의 핵심 동작을 아래에서 위로 직접 경험하는 프로젝트다. 하나의 서버 제품을 빠르게 완성하는 것이 아니라, 관찰한 문제를 해결하면서 작은 TCP Key-Value protocol로 발전시킨다.

## 핵심 원칙

```text
문제 경험 → 원인 분석 → 필요한 기술 도입 → 직접 수정 → 실제 생태계와 비교
```

- 처음부터 async, Tokio, HTTP framework를 사용하지 않는다.
- byte와 buffer부터 시작한다.
- `write` 호출과 `read` 호출이 일대일 대응하지 않는 현상을 직접 확인한다.
- 표준 라이브러리를 우선하고, 외부 crate는 필요성이 드러난 뒤 도입한다.
- production-ready 서버보다 시스템 원리 이해를 우선한다.

## 목표 결과물

- 길이-prefix framing을 사용하는 TCP client/server
- `SET`, `GET`, `DELETE` 명령을 처리하는 작은 Key-Value protocol
- blocking 단일 연결, thread 기반 다중 연결, async/Tokio 구현의 비교
- codec unit test와 실제 socket integration test
- 각 구현 선택과 관찰 결과를 설명하는 문서

상세 순서는 [CURRICULUM.md](CURRICULUM.md), 현재 위치는 [PROGRESS.md](PROGRESS.md)를 따른다.

## 명령

저장소 루트에서 실행한다.

```bash
cargo fmt --all -- --check
cargo check -p network-lab
cargo test -p network-lab
cargo clippy -p network-lab --all-targets -- -D warnings
cargo run -p network-lab
```
