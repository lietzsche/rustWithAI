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

## Linux/WSL Vim Rust 환경

저장소 루트의 [`setup-rust-vim.sh`](setup-rust-vim.sh)는 기존 `~/.vimrc`를 덮어쓰지 않고 Vim native package 경로에 Rust 개발 환경을 설치한다.

설치 구성:

- `rust.vim`: Rust filetype, syntax, `rustfmt` 연동
- `vim-lsp`: Vim의 LSP client
- `rust-analyzer`: Rust language server
- `asyncomplete.vim`, `asyncomplete-lsp.vim`: LSP 자동완성 popup
- `rust-src`, `rustfmt`: 표준 라이브러리 분석과 formatting 지원

사전 요구사항은 `vim`, `git`, `rustup`이다. Ubuntu/Debian에서 앞의 두 명령이 없다면 먼저 설치한다.

```bash
sudo apt install vim git
```

변경될 내용을 먼저 확인하고 설치한다.

```bash
./setup-rust-vim.sh --dry-run
./setup-rust-vim.sh
```

스크립트는 다음 위치만 관리한다.

```text
~/.vim/pack/rust-with-ai/start/
~/.vim/plugin/rust-with-ai-lsp.vim
```

기존 plugin checkout은 `git pull --ff-only`로 갱신한다. 같은 경로에 스크립트가 관리하지 않는 파일이 있으면 덮어쓰지 않고 중단한다.

Cargo project를 Vim으로 연 뒤 rust-analyzer가 시작될 시간을 잠시 기다리고 다음 명령으로 상태를 확인한다.

```vim
:LspStatus
```

기본 조작:

```text
gd           정의로 이동
gr           참조 검색
gi           구현으로 이동
gt           타입 정의로 이동
K            hover 정보
<leader>rn   이름 변경
[g / ]g      이전/다음 진단
Ctrl-X Ctrl-O 수동 completion
```

자동 completion popup은 `asyncomplete` 연동으로 제공한다. 설치 후 문제가 있으면 shell에서 `rust-analyzer --version`, Vim에서 `:LspStatus` 순서로 확인한다.
