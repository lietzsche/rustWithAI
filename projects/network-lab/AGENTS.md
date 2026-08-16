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

## 검증과 Git

- 단계 시작과 완료 시 이 프로젝트의 `PROGRESS.md`를 갱신한다.
- formatter, check, test, Clippy를 위험과 단계에 맞게 실행한다.
- 소단원 완료 후 관련 소스와 진도 문서를 같은 커밋에 포함하고 origin에 푸시한다.
- 커밋 메시지는 `N1-1 byte와 byte sequence 학습`처럼 단계와 내용을 한국어로 기록한다.
