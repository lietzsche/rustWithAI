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

## 코드 구성

초기 개념 관찰 코드는 진도를 다시 확인할 수 있도록 `n1_1.rs`, `n1_2.rs` 같은 단계별 module로 분리하고 `main.rs`에서는 실행 순서만 조립한다.

각 과제는 작성 대상 module과 `main.rs`의 연결 지점을 함께 제시한다. 학습자는 핵심 구현뿐 아니라 필요한 `mod` 선언, 공개 범위와 module 경로도 직접 작성한다.

실제 네트워크 프로그램이 성장하면 학습 단계 번호를 설계 경계로 사용하지 않는다. 책임이 드러나는 시점에 다음과 같은 역할 중심 module로 전환한다.

```text
buffer.rs
codec.rs
protocol.rs
client.rs
server.rs
connection.rs
```

필요해지기 전에 구조를 모두 만들지는 않으며, framing이나 client/server 분리처럼 실제 문제에서 책임이 생길 때 하나씩 추출한다.

## 명령

저장소 루트에서 실행한다.

```bash
cargo fmt --all -- --check
cargo check -p network-lab
cargo test -p network-lab
cargo clippy -p network-lab --all-targets -- -D warnings
cargo run -p network-lab
```
