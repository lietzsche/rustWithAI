# Network Lab 진행 상황

## 완료 범위

- 프로젝트 방향과 전체 커리큘럼 설계
- N1-1 byte와 byte sequence

## 현재 단계

- 단계: N1-1 byte와 byte sequence
- 상태: 과제 작성 및 피드백 완료

## 다음 단계

- N1-2 정수와 byte order
- big-endian과 little-endian의 byte 배열 비교
- `to_be_bytes`, `from_be_bytes`로 정수 왕복

## 최근 작업

- `u8`이 8bit unsigned 정수이며 현대 시스템의 byte 하나와 대응하는 이유를 확인했다.
- `&[u8]`로 ASCII와 UTF-8 문자열의 byte 표현을 비교했다.
- `Vec<u8>`로 소유하는 byte buffer를 만들고 byte literal을 추가했다.
- `Display`를 구현하지 않은 byte slice를 `{}`로 출력해 발생한 `E0277`을 `{:?}`로 수정했다.
- signed와 unsigned의 범위 및 동일한 bit pattern을 해석하는 차이를 확인했다.
- Network Lab의 문제 중심 학습 원칙을 확정했다.
- blocking TCP에서 시작해 framing, protocol, thread, shared state, non-blocking I/O, Tokio로 이어지는 커리큘럼을 구성했다.
- 핵심 결과물을 길이-prefix TCP Key-Value server/client로 정했다.
