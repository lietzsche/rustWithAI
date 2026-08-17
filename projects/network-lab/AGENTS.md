# Network Lab 작업 지침

## 필수 확인 문서

작업 전에 이 디렉터리의 `CURRICULUM.md`, `PROGRESS.md`, `README.md`와 저장소 루트 `AGENTS.md`를 확인한다. 단계는 이 프로젝트의 `CURRICULUM.md`, 진도는 `PROGRESS.md`를 정본으로 사용한다.

## 학습 목적

Network Lab의 1차 목적은 idiomatic Rust나 production-ready 서버가 아니라 네트워크와 동시성의 내부 원리를 코드로 경험하는 것이다.

```text
개념 최소 설명
→ 작은 구현 과제
→ 실행과 문제 재현
→ 원인 분석
→ 학습자가 수정
→ 필요한 추상화 도입
→ 실무 기술과 비교
```

## 진행 원칙

- 처음부터 async, Tokio, HTTP framework를 사용하지 않는다.
- byte, buffer, blocking I/O와 TCP stream부터 시작한다.
- framing, concurrency, shared state, async는 앞 단계에서 필요성이 드러난 뒤 도입한다.
- 표준 라이브러리를 우선한다.
- 핵심 codec, connection 처리, concurrency 코드는 학습자가 직접 작성한다.
- 정답 코드를 먼저 보여주지 않는다. 첫 번째에는 방향 힌트, 두 번째에는 구체적인 힌트, 요청 시 완성 코드와 해설을 제공한다.
- 의도적 문제 실험은 관찰한 뒤 정상 상태로 정리한다.
- 추상화는 현재 문제를 해결하는 데 필요한 만큼만 추가한다.

## 설명 기준

- `TcpStream`은 Java `Socket`, blocking thread는 Java Thread/Executor, async task는 CompletableFuture나 Node.js event loop와 비교할 수 있다.
- 비슷하다는 설명보다 ownership, scheduling, error 처리와 runtime 모델의 차이를 우선한다.
- OS socket buffer, syscall, kernel 대기처럼 관찰 결과에 영향을 주는 하위 계층을 필요한 깊이까지 설명한다.

## 코드 구성 원칙

- `main.rs`는 실행 순서와 최상위 조립만 담당하고, 완료된 독립 학습 예제는 `n1_1.rs`, `n1_2.rs`처럼 단계별 module로 보관할 수 있다.
- 과제를 제시할 때 학습자가 작성할 대상 파일과 module 연결 위치를 함께 명시한다. 독립적인 개념 실습은 처음부터 해당 단계 module에 작성하도록 유도하고, `main.rs`에는 `mod` 선언과 실행 호출만 추가하게 한다.
- 이미 역할 중심 module이 생긴 영역의 과제는 새 단계 module을 계속 만들지 않고 해당 `buffer`, `codec`, `client`, `server` 등의 module에 기능을 추가하도록 안내한다.
- module 선언, `pub` 공개 범위, `use`와 경로 선택도 학습자가 직접 작성하고 오류를 경험하도록 과제 범위에 포함한다. 단, 모듈화 자체가 현재 핵심 개념을 가릴 정도라면 필요한 최소 boilerplate만 제공할 수 있다.
- 단계별 module은 개념 관찰용 예제를 구분하기 위한 구조이며, 실제 Network Lab 구현의 영구적인 설계 단위로 사용하지 않는다.
- TCP client/server와 재사용 로직이 성장하기 시작하면 단원 번호가 아니라 `buffer`, `codec`, `protocol`, `client`, `server`, `connection`처럼 책임과 역할을 기준으로 module을 나눈다.
- 아직 역할이 드러나지 않은 코드를 미리 세분화하지 않는다. 현재 문제를 해결하면서 책임이 명확해지는 시점에 module을 추출한다.
- 기존 단계별 예제를 보존할지 역할 module의 test로 옮길지는 전환 시점에 학습 가치와 중복을 기준으로 결정한다.

## 검증과 Git

- 단계 시작과 완료 시 이 프로젝트의 `PROGRESS.md`를 갱신한다.
- formatter, check, test, Clippy를 위험과 단계에 맞게 실행한다.
- 소단원 완료 후 관련 소스와 진도 문서를 같은 커밋에 포함하고 origin에 푸시한다.
- 커밋 메시지는 `N1-1 byte와 byte sequence 학습`처럼 단계와 내용을 한국어로 기록한다.
