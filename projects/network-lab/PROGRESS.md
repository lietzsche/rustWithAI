# Network Lab 진행 상황

## 완료 범위

- 프로젝트 방향과 전체 커리큘럼 설계
- N1-1 byte와 byte sequence
- N1-2 정수와 byte order
- N1-3 buffer에 쓰고 읽기
- N1 Byte와 Buffer
- N2-1 I/O와 blocking
- N2-2 socket, IP, port

## 현재 단계

- 단계: N2-2 socket, IP, port
- 상태: 과제 작성 및 피드백 완료

## 다음 단계

- N2-3 첫 byte 왕복
- client가 보낸 byte를 server가 읽어 반환한다.
- connection 종료와 read/write 오류 전파를 관찰한다.

## 최근 작업

- `TcpListener`를 `127.0.0.1:7878`에 bind하고 client가 없을 때 `accept`가 blocking되는 것을 확인했다.
- `TcpStream::connect`로 loopback server에 연결해 `accept`가 반환되는 것을 확인했다.
- server와 client의 `local_addr`, `peer_addr`를 비교해 양쪽 endpoint가 서로 반대로 대응함을 확인했다.
- server port는 명시적으로 bind한 `7878`이고 client port는 OS가 고른 임시 port임을 확인했다.
- 연결을 기다리는 `TcpListener`와 연결 후 통신에 사용하는 `TcpStream`의 역할을 구분했다.
- `Vec<u8>`에 `Write::write_all`로 전체 byte sequence를 기록했다.
- `Cursor<Vec<u8>>`를 작은 buffer로 반복해서 읽어 `Read::read`의 실제 반환 길이를 확인했다.
- non-empty buffer의 `read == 0`을 EOF로 판단하고 반복을 종료했다.
- terminal 입력 전에는 `read`가 대기하고 입력 후 반환되는 blocking 동작을 관찰했다.
- 빈 입력에서 `Ctrl+D`로 EOF를 전달해 표준 입력의 `read == 0`을 확인했다.
- interactive 실험을 `blocking_stdin_demo`로 분리하고 `--stdin-demo`에서만 실행되도록 정리했다.
- `.read()` method syntax가 `Read` trait의 `read` 호출임을 확인했다.
- 앞으로의 과제에서 작성 대상 module과 `main.rs` 연결을 명시하고 학습자가 직접 모듈화하도록 하는 진행 원칙을 추가했다.
- N1의 완료된 예제를 `n1_1`, `n1_2`, `n1_3` module로 분리하고 `main.rs`를 실행 조립 역할로 정리했다.
- 개념 예제는 단계별로 보관하되 실제 구현은 `buffer`, `codec`, `client`, `server` 등 역할 중심 module로 전환하는 원칙을 문서화했다.
- `Vec::with_capacity`와 append 전후의 length, capacity를 비교했다.
- `extend_from_slice`로 byte sequence를 buffer 뒤에 추가하고 재할당을 관찰했다.
- 별도 `read_position`과 `&[u8]` slice로 원본을 삭제하거나 복사하지 않고 논리적으로 소비했다.
- `[u8]`는 unsized slice 본체이고 `&[u8]`는 주소와 길이를 가진 고정 크기 reference임을 확인했다.
- raw byte와 UTF-8 text 해석을 분리하고 `from_utf8`의 `Result`를 확인했다.
- 문자열과 정수를 byte로 변환하고 원래 값으로 복원해 N1 종료 조건을 충족했다.
- `u32`를 big-endian과 little-endian의 `[u8; 4]`로 변환했다.
- 각 byte 배열을 올바른 byte order로 복원해 원래 정수와 일치함을 확인했다.
- big-endian 배열을 little-endian으로 잘못 해석해 값이 달라지는 현상을 확인했다.
- `from_be_bytes`를 배열의 메서드로 호출해 발생한 `E0599`를 associated function 호출로 수정했다.
- 16진수 literal과 `{:#010x}` formatting의 의미를 확인했다.
- format specifier 앞의 `:`를 생략해 발생한 format string 문법 오류를 수정했다.
- `u8`이 8bit unsigned 정수이며 현대 시스템의 byte 하나와 대응하는 이유를 확인했다.
- `&[u8]`로 ASCII와 UTF-8 문자열의 byte 표현을 비교했다.
- `Vec<u8>`로 소유하는 byte buffer를 만들고 byte literal을 추가했다.
- `Display`를 구현하지 않은 byte slice를 `{}`로 출력해 발생한 `E0277`을 `{:?}`로 수정했다.
- signed와 unsigned의 범위 및 동일한 bit pattern을 해석하는 차이를 확인했다.
- Network Lab의 문제 중심 학습 원칙을 확정했다.
- blocking TCP에서 시작해 framing, protocol, thread, shared state, non-blocking I/O, Tokio로 이어지는 커리큘럼을 구성했다.
- 핵심 결과물을 길이-prefix TCP Key-Value server/client로 정했다.
