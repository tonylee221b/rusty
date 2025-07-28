# Cargo

- a _build system_ && a _package manager_

## Cargo.toml

`Cargo.toml` 파일은 _패키지 설정_ 및 *의존성 관리*를 담당함

## Commands

- 새로운 프로젝트 생성

```bash
cargo new {new_project_name}
```

- 프로젝트 빌드

```bash
cargo build

# 기본 빌드는 Debug
# 실행
./target/debug/{project_name}


# Release 버전 빌드
cargo build --release

# 실행
./target/release/{project_name}
```

- 컴파일 && 실행을 한번에

```bash
cargo run
```

- 컴파일만 (실행 파일 생성 X)

```bash
cargo check
```
