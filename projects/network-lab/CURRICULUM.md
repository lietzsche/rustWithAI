# Network Lab 커리큘럼

## 과정 목표

네트워크 API 사용법을 암기하는 대신 byte가 application protocol이 되기까지의 과정을 직접 구현한다. 각 장은 이전 구현에서 실제 문제를 관찰하고 다음 기술의 필요성을 발견하도록 구성한다.

핵심 결과물은 길이-prefix framing을 사용하는 작은 TCP Key-Value server/client다. 처음에는 blocking 방식으로 만들고, 동시성과 확장성 문제를 경험한 뒤 non-blocking I/O와 async/Tokio로 확장한다.

---

## N1. Byte와 Buffer

### N1-1. byte와 byte sequence

- `u8`, `&[u8]`, `Vec<u8>`
- text와 byte의 차이
- UTF-8 encoding 관찰

### N1-2. 정수와 byte order

- 정수의 고정 크기 표현
- big-endian과 little-endian
- `to_be_bytes`, `from_be_bytes`

### N1-3. buffer에 쓰고 읽기

- append와 consume 위치
- capacity와 length
- slice로 복사 없이 읽을 수 있는 범위

### N1 종료 조건

- 정수와 문자열을 byte buffer로 바꾸고 다시 복원한다.
- byte order가 서로 다르면 값이 달라지는 이유를 설명한다.

---

## N2. Blocking I/O와 TCP 연결

### N2-1. I/O와 blocking

- `Read`, `Write`
- 입력이 없을 때 호출이 기다리는 현상
- EOF와 `read == 0`

### N2-2. socket, IP, port

- `TcpListener`, `TcpStream`
- bind, listen, accept, connect의 역할
- loopback에서 단일 client 연결

### N2-3. 첫 byte 왕복

- client가 보낸 byte를 server가 읽어 반환
- connection 종료가 양쪽에서 보이는 방식
- read/write 오류 전파

### N2 종료 조건

- loopback TCP client/server가 byte를 왕복한다.
- `accept`와 `read`가 각각 언제 기다리는지 실행 결과로 설명한다.

---

## N3. TCP Stream의 성질

### N3-1. 메시지 경계가 없는 stream

- 여러 번의 `write`와 `read` 결과 비교
- buffer 크기를 바꾼 반복 실험
- packet과 application message를 동일시하면 안 되는 이유

### N3-2. partial read와 partial write

- 읽고 쓴 byte 수를 반드시 확인하는 이유
- `read_exact`, `write_all`이 보장하는 것과 보장하지 않는 것

### N3-3. 연결 종료와 오류 상황

- 정상 EOF
- client 중도 종료
- timeout
- `ConnectionReset`, `BrokenPipe` 등 OS 오류 관찰

### N3 종료 조건

- 한 번의 `write`가 한 번의 `read`와 대응한다는 잘못된 가정을 재현하고 설명한다.
- 고정 buffer에서도 전체 데이터를 안전하게 처리한다.

---

## N4. Framing과 Codec

### N4-1. framing이 필요한 이유

- delimiter, fixed-length, length-prefix 비교
- 이 프로젝트에서 length-prefix를 선택하는 이유

### N4-2. frame encoding

- header에 payload 길이 기록
- payload 결합
- 최대 frame 크기 제한

### N4-3. incremental decoding

- header 일부만 도착한 상태
- payload 일부만 도착한 상태
- buffer에 여러 frame이 함께 있는 상태

### N4-4. codec test

- 정상 frame 왕복
- 잘린 frame
- 잘못된 길이와 과도한 길이

### N4 종료 조건

- 임의의 read 경계와 관계없이 frame을 복원한다.
- codec 순수 로직을 socket 없이 unit test한다.

---

## N5. Application Protocol

### N5-1. command와 response 모델

- `SET`, `GET`, `DELETE`
- Rust enum으로 protocol message 모델링
- 성공, 값 없음, protocol error 응답

### N5-2. message serialization

- command tag와 field encoding
- byte에서 enum으로 parsing
- 알 수 없는 command와 malformed input

### N5-3. session loop

- 한 연결에서 여러 request/response 처리
- protocol error와 connection error 구분
- keep-alive가 자연스럽게 생기는 이유

### N5 종료 조건

- 단일 client가 하나의 TCP 연결에서 여러 Key-Value 명령을 실행한다.
- transport framing과 application message의 책임을 분리한다.

---

## N6. 여러 Client와 Thread

### N6-1. 순차 server의 한계

- 한 client가 점유할 때 다른 client가 기다리는 현상 재현
- 느린 client가 전체 처리량에 미치는 영향

### N6-2. thread-per-connection

- `std::thread::spawn`
- closure ownership, `move`, `Send`, `'static`
- client별 오류 격리

### N6-3. thread 생명주기

- detached thread와 `JoinHandle`
- 종료와 자원 정리
- client 수만큼 thread가 늘어나는 비용 관찰

### N6 종료 조건

- 여러 client를 동시에 처리한다.
- thread-per-connection이 단순한 이유와 확장 한계를 설명한다.

---

## N7. Shared State와 Message Passing

### N7-1. 공유 Key-Value 저장소

- `Arc<Mutex<HashMap<...>>>`
- ownership 공유와 mutable access 직렬화
- lock 범위와 contention

### N7-2. 실패와 교착 위험

- mutex poisoning
- lock을 오래 보유했을 때의 영향
- 중첩 lock을 피해야 하는 이유

### N7-3. channel 기반 owner task

- `mpsc` channel
- 저장소를 한 thread가 소유하는 구조
- 공유 메모리 방식과 message passing 비교

### N7 종료 조건

- 여러 client가 일관된 Key-Value 상태를 공유한다.
- mutex 방식과 channel 방식의 소유권 및 병목 차이를 설명한다.

---

## N8. Robustness와 Backpressure

### N8-1. error 모델

- I/O, codec, protocol error 구분
- custom error
- 필요성이 확인된 뒤 `thiserror` 또는 `anyhow` 비교

### N8-2. timeout과 retry

- connect/read/write timeout
- 안전하게 retry할 수 있는 작업
- 중복 실행과 idempotency 문제

### N8-3. bounded queue와 backpressure

- 생산 속도가 소비 속도보다 빠른 상황
- 무제한 buffering의 메모리 문제
- 요청 거부, 대기, 연결 종료 정책 비교

### N8-4. graceful shutdown

- 새 연결 수락 중단
- 처리 중인 작업 정리
- thread와 socket 종료

### N8 종료 조건

- 느리거나 잘못된 client가 서버 전체를 무제한 점유하지 못하게 한다.
- timeout, retry, backpressure 정책의 trade-off를 설명한다.

---

## N9. Test와 관찰 가능성

### N9-1. integration test

- 실제 loopback socket 사용
- 임의 port 할당
- server 준비와 test 종료 동기화

### N9-2. protocol 경계 test

- frame 분할 전송
- 여러 frame 일괄 전송
- disconnect와 timeout

### N9-3. logging과 측정

- 연결, 요청, 오류를 구조적으로 관찰
- latency와 처리량의 기초 측정
- logging이 동작을 바꾸는 경우 주의

### N9 종료 조건

- codec unit test와 socket integration test를 구분해 작성한다.
- 관찰 결과로 성능이나 동시성 주장을 검증한다.

---

## N10. Non-blocking I/O

### N10-1. blocking 모델의 비용 정리

- connection과 thread의 관계
- context switching과 stack 비용
- 동시 연결 수 증가 실험

### N10-2. non-blocking socket

- `set_nonblocking(true)`
- `WouldBlock`
- 준비되지 않은 I/O를 직접 관리할 때 생기는 복잡성

### N10-3. event loop 개념

- readiness와 polling
- 여러 connection 상태 관리
- busy loop를 피해야 하는 이유

### N10 종료 조건

- blocking과 non-blocking의 차이를 실행 결과로 설명한다.
- runtime이 대신 관리해야 할 상태와 wake-up의 필요성을 설명한다.

---

## N11. Async/Await와 Tokio

### N11-1. Future와 async/await

- Future가 아직 완료되지 않은 계산을 표현하는 방식
- `.await`에서 thread 전체를 막지 않는 의미
- blocking 함수와 async 함수 혼용 문제

### N11-2. Tokio runtime과 task

- runtime, task, scheduler
- `tokio::spawn`의 ownership, `Send`, `'static`
- async socket I/O

### N11-3. blocking server 이식

- protocol과 codec 로직 재사용
- connection 처리부를 async로 변경
- cancellation과 graceful shutdown

### N11-4. 구현 비교

- blocking, thread-per-connection, Tokio 구조 비교
- 코드 복잡도, 동시 연결, 처리량 관찰
- async가 항상 더 나은 것은 아닌 이유

### N11 종료 조건

- 동일 protocol을 blocking과 Tokio 두 방식으로 실행한다.
- async가 해결하는 문제와 새로 도입하는 복잡성을 설명한다.

---

## N12. Protocol 확장 선택지

핵심 과정을 완료한 뒤 관심과 앞선 실험 결과에 따라 하나씩 선택한다. 모두 필수로 넣지 않는다.

- UDP와 datagram 경계
- DNS 질의 message 분석
- 최소 HTTP/1.1 message와 keep-alive
- TCP proxy와 양방향 forwarding
- TLS를 적용해 평문과 암호화 구간 비교
- connection pool과 load balancing 기초

---

## 과정 완료 조건

- blocking TCP에서 stream과 framing 문제를 직접 재현하고 해결했다.
- 작은 Key-Value protocol과 여러 client의 공유 상태를 구현했다.
- partial I/O, EOF, timeout, backpressure와 shutdown을 다뤘다.
- unit test와 실제 socket integration test로 경계 상황을 검증했다.
- blocking, thread-per-connection, non-blocking, Tokio 방식의 차이를 자신의 코드와 측정 결과로 설명할 수 있다.
