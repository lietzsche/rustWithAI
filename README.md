# Rust Systems Learning Labs

Rust를 도구로 사용해 네트워크, 운영체제, 메모리, 동시성, 저장소와 분산 시스템의 내부 동작을 직접 구현하며 학습하는 저장소다.

## 학습 경로

### Rust를 처음 시작한다면

[archive/rust-basics](archive/rust-basics/README.md)에서 1장부터 시작한다. Java, JavaScript, TypeScript 경험자를 대상으로 Rust 문법, Ownership, Borrowing, Trait, Lifetime과 Cargo를 단계적으로 다룬다.

### Rust 기본기를 마쳤다면

[projects/network-lab](projects/network-lab/README.md)에서 현재 학습을 이어간다. byte와 blocking I/O부터 시작해 TCP stream의 문제를 경험하고 framing, 동시성, non-blocking I/O, async/Tokio로 확장한다.

## 저장소 구조

```text
rustWithAI/
├── archive/
│   └── rust-basics/       # 완료된 Rust 기본기 1~14장
├── projects/
│   └── network-lab/       # 현재 진행하는 네트워크 시스템 학습
├── CURRICULUM.md          # 전체 학습 로드맵
└── PROGRESS.md            # 프로젝트 간 현재 위치
```

각 학습 경로는 자체 `README.md`, `CURRICULUM.md`, `PROGRESS.md`와 작업 지침을 가진다. 실제 단계와 진도는 해당 프로젝트의 문서를 정본으로 사용한다.

## 현재 Workspace 검증

저장소 루트 Cargo workspace에는 현재 진행 중인 프로젝트만 포함한다.

```bash
cargo fmt --all -- --check
cargo check --workspace
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

완료된 기본기 Cargo 프로젝트는 독립적으로 검증할 수 있다.

```bash
cd archive/rust-basics
cargo test --workspace
```
