# Rust with AI

Java, JavaScript, TypeScript에 익숙한 개발자가 Rust 기본기를 직접 코딩하며 익히는 학습 저장소다.

현재 과정은 텍스트 RPG를 지속적으로 확장하면서 문법, Ownership, Borrowing, Trait, Lifetime을 경험하고, 마지막에는 Cargo 기반 프로젝트로 전환하는 것을 목표로 한다.

## 문서

- [CURRICULUM.md](CURRICULUM.md): 1~14장 공식 학습 커리큘럼
- [PROGRESS.md](PROGRESS.md): 완료 범위와 현재 진도
- [AGENTS.md](AGENTS.md): Codex 및 공통 에이전트 작업 지침
- [CLAUDE.md](CLAUDE.md): Claude 작업 지침

커리큘럼과 진도는 채팅 기록이 아니라 위 문서를 정본으로 관리한다.

## 현재 학습 방식

각 단계는 다음 흐름으로 진행한다.

```text
개념 최소 설명
→ 짧은 예제
→ 직접 작성할 과제
→ 코드 피드백과 컴파일 오류 분석
→ 검증·커밋·푸시
```

학습 소스는 단계별 디렉터리에 저장한다.

```text
1.1/homework.rs
...
13.5/homework.rs
```

현재 위치는 [PROGRESS.md](PROGRESS.md)를 참고한다.

## 실행

Cargo 전환 전의 각 과제는 해당 디렉터리에서 `rustc`로 컴파일한다.

```bash
cd 10.2 # 예시
rustc homework.rs
./homework
```

생성된 `homework` 실행 파일은 Git에서 제외한다.

14장에서는 기존 단일 파일 RPG를 Cargo package로 전환하고 다음 개발 루프를 익힌다.

```bash
cargo check
cargo run
cargo test
cargo fmt
cargo clippy
```

## 기본기 과정 종료 기준

1~13장에서 Rust 언어 핵심을 학습하고, 14장에서 Cargo와 실제 프로젝트 구조를 익히면 기본기 과정을 종료한다.

공식 문서 정독, custom error crate, concurrency, async는 기본기 선행 과정으로 강제하지 않는다. 이후 실제 프로젝트에서 필요해질 때 공식 문서를 찾아보고 구현과 함께 학습한다.

## 이후 계획

14장 완료 후 바로 파일을 이동하거나 새 프로젝트를 만들지 않는다. 사용자와 범위를 다시 논의한 뒤 다음 구조로 전환할 예정이다.

```text
rustWithAI/
├── archive/
│   └── rust-basics/
├── projects/
│   └── network-lab/
├── AGENTS.md
├── CLAUDE.md
└── README.md
```

`projects/network-lab`에는 별도의 `README.md`, `CURRICULUM.md`, `PROGRESS.md`, `AGENTS.md`, `CLAUDE.md`를 두며 네트워크 커리큘럼은 프로젝트 시작 전에 사용자와 논의해 확정한다.

세부 전환 절차는 [CURRICULUM.md](CURRICULUM.md)의 마지막 절을 따른다.
