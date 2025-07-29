# Data Types

## 기본 타입

- 너무나도 쉽기 때문에 생략

## 복합 타입

### Tuple

- 다양한 타입의 여러 값을 묶어 하나의 복합 타입으로 만든 것임

```rust
fn main() {
  let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```

- 패턴 매칭해서 튜플 값을 해체할 수 도 있음

```rust
fn main() {
  let tup = (500, 6.4, 1);
  let (x, y, z)  = tup;
  println!("The value of y is: {y}");
}
```

- 마침표(`.`) 뒤에 인덱스를 써서 튜플 요소에 접근 가능함

```rust
fn main() {
  let x: (i32, f64, u8) = (500, 6.4, 1);
  let five_hundred = x.0;
  let six_point_four = x.1;
  let one = x.2;
}
```

### 배열 타입

```rust
fn main() {
  let a = [1, 2, 3, 4, 5];
  let b: [i32; 5] = [1, 2, 3, 4, 5];
  let c: [3; 5]; // [3, 3, 3, 3, 3]
}
```
