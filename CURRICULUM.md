# Rust 기본기 커리큘럼

## 과정 목표

이 과정은 Java, JavaScript, TypeScript에 익숙한 개발자가 Rust의 언어 핵심을 직접 코딩하며 익히고 실제 프로젝트를 시작할 수 있는 상태에 도달하는 것을 목표로 한다.

1~13장은 언어 기본기, 14장은 Cargo와 실제 프로젝트 진입 과정이다. 14장을 완료하면 현재 Rust 기본기 과정을 종료한다.

---

## 1. Rust 기본 문법

### 1-1. 프로그램 구조

- `.rs` 파일
- `fn main`
- `println!`
- Rust 코드 실행의 기본 개념

### 1-2. 변수

- `let`
- immutable 기본값
- `mut`
- `const`

### 1-3. 기본 타입

- 타입 추론
- `i32`, `i64`
- `u32`, `u64`
- `usize`
- `f32`, `f64`
- `bool`, `char`

### 1-4. 함수

- `fn`
- parameter와 parameter type
- 함수 호출

### 1-5. Statement와 Expression

- statement
- expression
- 세미콜론의 의미

### 1-6. 반환값

- `->`
- 마지막 expression
- `return`

---

## 2. 제어문

### 2-1. `if` / `else`

### 2-2. `loop`

### 2-3. `while`

### 2-4. `for`

### 2-5. Range

- `0..10`
- `0..=10`

### 2-6. 반복 제어

- `break`
- `continue`
- `loop`에서 값 반환

---

## 3. 기본 데이터 구조

### 3-1. Array

### 3-2. Tuple

### 3-3. `Vec<T>`

- 생성
- `push`, `pop`
- 길이
- indexing

### 3-4. `usize`

- 배열과 Vec의 index
- 다른 정수 타입과의 차이

### 3-5. 컬렉션 반복

- index 기반 반복
- 값 기반 반복
- reference 기반 반복

---

## 4. 문자열

### 4-1. `&str`

### 4-2. `String`

### 4-3. 문자열 생성과 변경

- `String::from`
- `to_string`
- `push`
- `push_str`

### 4-4. `String`과 `&str` 관계

### 4-5. 문자열 처리

- 문자열 반복
- 문자 단위 처리
- byte와 Unicode 기초

---

## 5. Ownership

### 5-1. Stack과 Heap

Ownership을 이해하는 데 필요한 수준까지만 다룬다.

### 5-2. Ownership 기본 규칙

### 5-3. Move

### 5-4. Copy와 Clone

### 5-5. 함수 호출과 Ownership

### 5-6. 반환값과 Ownership

설명만으로 끝내지 않고 move 이후 원래 값을 사용하는 컴파일 오류를 직접 경험한다.

---

## 6. Reference와 Borrowing

### 6-1. Reference

- `&T`

### 6-2. Borrowing

### 6-3. Mutable Reference

- `&mut T`

### 6-4. Mutable Reference 규칙

### 6-5. Slice

- `&[T]`
- 문자열 slice

### 6-6. Ownership / Borrowing 종합 실습

Borrow checker 오류를 직접 경험하며 소유권, immutable borrow, mutable borrow, slice를 함께 사용한다.

---

## 7. Struct와 `impl`

### 7-1. Struct 정의

### 7-2. Struct 생성과 field

### 7-3. Method

### 7-4. `impl`

### 7-5. Associated Function

### 7-6. 여러 Struct를 이용한 모델링

Java class와 비교하되 Rust struct를 class와 동일한 개념으로 설명하지 않는다.

---

## 8. Enum과 Pattern Matching

### 8-1. Enum

### 8-2. Enum에 데이터 넣기

### 8-3. `match`

### 8-4. `if let`

### 8-5. Pattern과 destructuring 기초

필요한 경우 TypeScript union 또는 discriminated union과 비교한다.

---

## 9. Option과 Result

### 9-1. `Option<T>`

### 9-2. `Some` / `None`

### 9-3. `Result<T, E>`

### 9-4. `Ok` / `Err`

### 9-5. `unwrap`

### 9-6. `expect`

### 9-7. `?` 연산자

Java의 null 및 exception 방식과 비교하고 panic과 오류 전파의 차이를 직접 확인한다.

---

## 10. Closure와 Iterator

### 10-1. Closure

### 10-2. `iter`

### 10-3. `iter_mut`

### 10-4. `into_iter`

### 10-5. `map`

### 10-6. `filter`

### 10-7. `collect`

### 10-8. Iterator 체이닝

Java Stream API와 JavaScript Array API를 적극적으로 비교한다.

---

## 11. 표준 컬렉션

### 11-1. `HashMap`

### 11-2. `HashSet`

### 11-3. `VecDeque`

### 11-4. `BinaryHeap`

### 11-5. 정렬

### 11-6. 자주 사용하는 표준 라이브러리 패턴

---

## 12. Generic과 Trait

### 12-1. Generic 함수

### 12-2. Generic Struct

### 12-3. Trait

### 12-4. Trait 구현

### 12-5. Trait Bound

### 12-6. 여러 Trait 조합

Java interface와 generic을 비교하되 Rust의 static dispatch 및 trait 철학 차이를 설명한다.

---

## 13. Lifetime

### 13-1. Lifetime이 필요한 이유

### 13-2. Lifetime annotation

### 13-3. 함수와 Lifetime

### 13-4. Struct와 Lifetime

### 13-5. Lifetime elision

Lifetime 문법을 먼저 암기하지 않는다. Reference 관계에서 문제가 발생하는 실제 코드를 먼저 경험한 뒤 annotation을 학습한다.

---

## 14. Cargo와 실제 프로젝트 진입

이 장은 단일 `.rs` 파일과 `rustc` 중심 학습에서 실제 Cargo 프로젝트로 전환하는 과정이다. 명령어 암기보다 기존 RPG를 실제 package로 옮기며 익힌다.

### 14-1. `cargo new`와 기본 디렉터리 구조

- `Cargo.toml`
- `src/main.rs`
- `target/`

### 14-2. 기본 개발 명령

- `cargo run`
- `cargo build`
- `cargo check`

### 14-3. Manifest와 Edition

- `Cargo.toml`
- `Cargo.lock`
- package metadata
- Rust edition

### 14-4. Dependency

- dependency 추가
- crates.io dependency와 version
- Cargo가 dependency를 해석하고 빌드하는 기본 흐름

### 14-5. Package와 Crate

- package
- crate
- crate root
- binary crate
- library crate

### 14-6. Binary와 Library 분리

- `src/main.rs`
- `src/lib.rs`
- 실행 진입점과 재사용 가능한 로직 분리

### 14-7. Module과 공개 범위

- `mod`
- `use`
- path
- `pub`
- 기본 private 규칙

### 14-8. 여러 파일로 코드 분리

- 관련 타입과 함수를 module로 이동
- 파일과 module 관계
- 과도하게 잘게 나누지 않는 기준

### 14-9. 기본 개발 도구

- `cargo fmt`
- `cargo clippy`
- formatter와 lint의 역할

### 14-10. `cargo test`와 최소 Unit Test

- `#[test]`
- 기본 assertion
- 순수 로직 하나를 unit test로 검증
- 상세 테스트 설계는 후속 프로젝트에서 필요할 때 확장

### 14-11. Workspace 기초

- 여러 package를 묶는 workspace의 목적
- 필요하기 전에는 단일 package를 유지하는 판단 기준

### 14-12. 기존 RPG의 Cargo 프로젝트 전환

- 기존 누적 RPG 코드를 Cargo package로 이동
- binary와 library/module 경계 정리
- 최소 unit test 추가
- `cargo check`, `test`, `fmt`, `clippy`, `run` 검증
- 기본기 과정 완료 기록

---

## 지속적으로 확장하는 예제

1~13장에서는 하나의 텍스트 RPG/던전 게임을 가능한 범위에서 계속 확장한다.

- 변수와 제어문: HP, Gold, 공격, 회복
- 함수: 공격과 상태 출력
- Vec와 String: inventory, Player/Monster/Item 이름
- Ownership과 Borrowing: inventory와 전투 데이터의 안전한 전달
- Struct와 Enum: Player, Monster, Item, Action
- Option과 Result: 선택 장비와 실패 가능한 작업
- Iterator와 Closure: inventory와 전투 데이터 처리
- Generic과 Trait: 공통 행동 추상화
- Lifetime: reference 관계가 필요한 모델

게임 구현 자체가 Rust 학습보다 어려워지지 않게 한다. 아직 배우지 않은 개념을 억지로 사용하지 않는다.

---

## 기본기 과정에서 제외한 주제

다음 주제는 중요하지 않아서 삭제한 것이 아니라 실제 프로젝트에서 필요해질 때 학습하도록 이동했다.

- 공식 문서 순차 정독
- 상세 unit/integration test 설계
- custom error, `thiserror`, `anyhow`
- thread, channel, `Arc`, `Mutex`
- async/await, Future, Tokio
- domain별 고급 표준 라이브러리와 외부 crate

공식 문서는 구현 중 정확한 규칙이나 API가 필요할 때 해당 부분을 찾아본다. Effective Rust 같은 후속 자료는 실제 프로젝트 코드를 작성한 뒤 항목별로 적용한다.

---

## 14장 완료 후 전환 절차

14장을 완료해도 에이전트가 파일 이동이나 새 프로젝트 생성을 자동으로 수행하지 않는다. 반드시 사용자와 다음 프로젝트의 범위를 다시 논의한다.

합의 후 다음 순서로 전환한다.

1. 모든 기본기 단계와 `PROGRESS.md`가 완료 상태인지 확인한다.
2. 전체 소스를 검증하고 기본기 완료 커밋을 푸시한다.
3. `rust-basics-complete` 같은 완료 tag 생성 여부를 사용자와 결정한다.
4. 기존 학습 자료를 `archive/rust-basics/`로 이동한다.
5. `projects/network-lab/`을 생성한다.
6. 네트워크 프로젝트 범위와 커리큘럼을 사용자와 논의한다.
7. `projects/network-lab/`에 다음 문서를 만든다.
   - `README.md`
   - `CURRICULUM.md`
   - `PROGRESS.md`
   - `AGENTS.md`
   - `CLAUDE.md`
8. 루트 `README.md`, `AGENTS.md`, `CLAUDE.md`를 다중 프로젝트 저장소 구조에 맞게 갱신한다.

예상 구조:

```text
rustWithAI/
├── archive/
│   └── rust-basics/
├── projects/
│   └── network-lab/
│       ├── AGENTS.md
│       ├── CLAUDE.md
│       ├── README.md
│       ├── CURRICULUM.md
│       ├── PROGRESS.md
│       ├── Cargo.toml
│       ├── src/
│       └── tests/
├── AGENTS.md
├── CLAUDE.md
└── README.md
```

네트워크 프로젝트에서는 처음부터 async를 사용하지 않는다. byte, framing, blocking TCP를 먼저 구현하고 concurrency와 async가 필요한 이유를 경험한 뒤 thread, channel, async/await, Tokio로 확장하는 방향을 우선 검토한다.

