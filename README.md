Abbreviate or Unabbreviate numbers in rust


|     Input                   |    Output    |
|-----------------------------|--------------|
|         1_000               |     1K       |
|         1_000_000           |     1M       |
|         1_090               |     1.09K    |
|         001_090               |     1.09K    |
| 123_000_000_000_000_000_000 |     123E     |
| -123_000_000_000_000_000_000 |     -123E     |
| 1_000_000_000_000_000_000_000_000_000_000 |     1Q     |






|     Input                   |    Output    |
|-----------------------------|--------------|
|         1K               |     1000       |
|         1.09K           |     1090      |
|         1_090               |     1.09K    |
|      123E     |123_000_000_000_000_000_000|
|      1Q     |1_000_000_000_000_000_000_000_000_000_000|


Usage Example :

```rust

fn main() {
    println!("{}", 123.abbreviate_number()); // result: 123
    println!("{}", 123_000.0.abbreviate_number()); //result: 123K

    println!("{}", "1K".unabbreviate_number()); //result: 1000.0
    println!("{}", "1M".unabbreviate_number()); //result: 1000000.0
}

```

Also For Big Int Numbers: 
```rust
fn main() {
    let big_num: i128 = 999_509_999_999_999_000_123_123_000_000_123;
    println!("{}", big_num.abbreviate_number()); // result : 999.5Q
}

```
## Note
        . Symbols Are Case-insensitive 
        . Program Will Automaticly Remove Leading Zero 
        .Example: 
           001 --->1
           1.90 --->1.9


Table Of Symbols:

|     Symbol                   |    Base    |
|-----------------------------|--------------|
|         K               |     1_000       |
|         M               |      1_000_000      |
|         G               |     1_000_000_000    |
|      T                 |      1_000_000_000_000 |
|      P                 |      1_000_000_000_000_000  |
|      E                 |      1_000_000_000_000_000_000   |
|      Z                 |      1_000_000_000_000_000_000_000   |
|      Y                 |      1_000_000_000_000_000_000_000_000  |
|      R                 |      1_000_000_000_000_000_000_000_000_000   |
|      Q                 |      1_000_000_000_000_000_000_000_000_000_000   |

