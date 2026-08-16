# Rust with AI

Java, JavaScript, TypeScript에 익숙한 개발자가 Rust 기본기를 직접 코딩하며 익히는 학습 저장소다.

텍스트 RPG를 지속적으로 확장하면서 문법, Ownership, Borrowing, Trait, Lifetime을 경험하고 Cargo 기반 프로젝트로 전환하는 Rust 기본기 과정을 담고 있다.

## 문서

- [CURRICULUM.md](CURRICULUM.md): 1~14장 공식 학습 커리큘럼
- [PROGRESS.md](PROGRESS.md): 완료 범위와 현재 진도
- [AGENTS.md](AGENTS.md): Codex 및 공통 에이전트 작업 지침
- [CLAUDE.md](CLAUDE.md): Claude 작업 지침

커리큘럼과 진도는 채팅 기록이 아니라 위 문서를 정본으로 관리한다.

## 현재 상태

1~14장의 Rust 기본기 과정과 기존 RPG의 Cargo package 전환을 완료했다. 다음 프로젝트를 만들거나 기존 자료를 이동하기 전, 프로젝트 범위와 저장소 전환 절차를 논의하는 단계다.

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

14장에서 전환한 Cargo workspace와 RPG package는 저장소 루트에서 다음과 같이 검증·실행한다.

```bash
cargo fmt --all -- --check
cargo check --workspace
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo run -p rust-rpg
```

Cargo package는 `14.1/`에 있고 루트 `Cargo.toml`의 workspace member로 등록되어 있다.

## 기본기 과정 종료 기준

1~13장에서 Rust 언어 핵심을 학습하고, 14장에서 Cargo와 실제 프로젝트 구조를 익혀 기본기 과정을 완료했다.

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
