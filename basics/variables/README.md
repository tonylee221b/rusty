# Variable

## 일반 변수

- Rust에서는 항상 **Immutable** 임

```rust
let x = 5;
x = 6; // << ERROR!
```

- 따라서 _mut_ 이라는 키워드로 선언하여 변경될 수 있도록 함

```rust
let mut x = 5;
x = 6; // << GOOD!
```

## 상수

- 절대 바뀌지 않는 값
- _mut_ 이랑 같이 못씀
- _const_ 키워드 사용해서 선언함
- 값의 타입은 **반드시** 명시되어야 함

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

## 섀도잉

- 새 변수를 이전 변수명과 같은 이름으로 선언 가능함
- 컴파일러는 두 번째 변수를 보게 됨

```rust
fn main() {
  let x = 5;
  let x = x + 1; // << shadowing

  {
    let x = x * 2; // << shadowing
    println!("The value of x in the inner scope is: {x}");
  }

  println!("The value of x is: {x}")
}
```
