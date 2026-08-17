# Network Lab 진행 상황

## 완료 범위

- 프로젝트 방향과 전체 커리큘럼 설계
- N1-1 byte와 byte sequence
- N1-2 정수와 byte order

## 현재 단계

- 단계: N1-2 정수와 byte order
- 상태: 과제 작성 및 피드백 완료

## 다음 단계

- N1-3 buffer에 쓰고 읽기
- buffer의 length와 capacity 비교
- slice와 consume 위치로 복사 없이 읽는 범위 관찰

## 최근 작업

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
