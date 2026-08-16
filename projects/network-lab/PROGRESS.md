# Network Lab 진행 상황

## 완료 범위

- 프로젝트 방향과 전체 커리큘럼 설계

## 현재 단계

- 단계: N1-1 byte와 byte sequence
- 상태: 시작 준비

## 다음 단계

- `u8`, `&[u8]`, `Vec<u8>`의 관계 확인
- ASCII와 UTF-8 문자열의 byte 표현 비교
- 학습자가 직접 작성할 첫 byte buffer 과제 진행

## 최근 작업

- Network Lab의 문제 중심 학습 원칙을 확정했다.
- blocking TCP에서 시작해 framing, protocol, thread, shared state, non-blocking I/O, Tokio로 이어지는 커리큘럼을 구성했다.
- 핵심 결과물을 길이-prefix TCP Key-Value server/client로 정했다.
