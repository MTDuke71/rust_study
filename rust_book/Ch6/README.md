# Chapter 6: Enums and Pattern Matching

## 🔗 Zettelkasten Links
- **Overview**: [[zettelkasten/rust_book/rust-book-ch6]]
- **Previous**: [[zettelkasten/rust_book/rust-book-ch5]] - Structs and methods
- **Next**: [[zettelkasten/rust_book/rust-book-ch7]] - Packages, crates, and modules
- **Missions**: [[mission-3]] (LinkedList with Option) | [[mission-2]] (Queue with Result)
- **Daily Study**: [[daily-study/Day14]] - Uses Result enum extensively
- **Book MOC**: [[Rust Book MOC]]

## 📚 Overview

Chapter 6 introduces **enums** and **pattern matching**, two powerful features that enable expressing more complex data types and control flow. Enums allow you to define a type by enumerating its possible variants, while pattern matching provides an elegant way to handle different cases.

---

## 🎯 Key Concepts

### 1. **Defining Enums**

Enums allow you to define a type that can be one of several variants.

```rust
// Simple enum with variants
enum IpAddrKind {
    V4,
    V6,
}

// Enum with data in variants
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// Using the enum
let home = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1"));
```

### 2. **Enum Variants Can Hold Different Types**

Each variant can have different types and amounts of associated data:

```rust
enum Message {
    Quit,                       // No data
    Move { x: i32, y: i32 },   // Named fields (like struct)
    Write(String),              // Single value
    ChangeColor(i32, i32, i32), // Three values
}
```

### 3. **The Option Enum**

Rust doesn't have `null`. Instead, it uses `Option<T>` to represent a value that might be absent:

```rust
enum Option<T> {
    Some(T),
    None,
}

// Using Option
let some_number = Some(5);
let some_string = Some("a string");
let absent_number: Option<i32> = None;

// Must handle None case explicitly
fn add_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None,
    }
}
```

### 4. **The match Control Flow Operator**

`match` allows you to compare a value against patterns and execute code based on which pattern matches:

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

### 5. **Patterns That Bind to Values**

Match arms can bind to parts of the values that match the pattern:

```rust
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // ... etc
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
```

### 6. **Matching with Option<T>**

Common pattern for handling `Option`:

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```

### 7. **Matches Are Exhaustive**

You must handle all possible cases:

```rust
// ❌ This won't compile - missing None case
fn plus_one_bad(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
    }
}

// ✅ Use _ to catch all remaining patterns
fn describe_number(x: Option<i32>) {
    match x {
        Some(3) => println!("three"),
        Some(7) => println!("lucky seven"),
        _ => println!("some other number or none"),
    }
}
```

### 8. **Concise Control Flow with if let**

When you care about only one pattern and want to ignore the rest:

```rust
// Using match (verbose)
let some_value = Some(3);
match some_value {
    Some(3) => println!("three"),
    _ => (),
}

// Using if let (concise)
let some_value = Some(3);
if let Some(3) = some_value {
    println!("three");
}

// With else clause
let coin = Coin::Quarter(UsState::Alaska);
let mut count = 0;
if let Coin::Quarter(state) = coin {
    println!("State quarter from {:?}!", state);
} else {
    count += 1;
}
```

---

## 🔑 Key Takeaways

### Enum Benefits
- **Type Safety**: Compiler ensures all variants are handled
- **No Null**: `Option<T>` eliminates null pointer errors
- **Expressive**: Can model complex domain logic clearly
- **Pattern Matching**: Elegant control flow with exhaustiveness checking
- **Data Association**: Each variant can carry different data

### Pattern Matching Patterns
1. **Literal Matching** - Match specific values
2. **Variable Binding** - Extract data from variants
3. **Wildcard `_`** - Catch-all for remaining cases
4. **Guard Expressions** - Additional conditions with `if`

### Best Practices
- **Use `Option<T>` instead of nullability** - Forces explicit handling of absent values
- **Prefer `match` for multiple cases** - Exhaustiveness checking prevents bugs
- **Use `if let` for single case** - More concise when you only care about one variant
- **Derive Debug for enums** - Makes them easier to inspect
- **Use enums for state machines** - Model different states as variants
- **Put logic in enum methods** - Implement methods on enums like structs

---

## 🛠️ Common Patterns

### State Machine with Enums
```rust
enum State {
    Idle,
    Processing(String),
    Completed(u32),
    Failed(String),
}

impl State {
    fn process(&mut self, input: String) {
        *self = match self {
            State::Idle => State::Processing(input),
            State::Processing(data) => {
                // Do processing
                State::Completed(data.len() as u32)
            }
            _ => State::Failed("Invalid state transition".to_string()),
        }
    }
}
```

### Result Error Handling
```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("division by zero"))
    } else {
        Ok(a / b)
    }
}

// Using the result
match divide(10, 2) {
    Ok(result) => println!("Result: {}", result),
    Err(e) => println!("Error: {}", e),
}
```

### Option Chaining
```rust
// Extract nested Option values
fn get_user_city(user_id: i32) -> Option<String> {
    let user = find_user(user_id)?;  // Early return if None
    let address = user.address?;
    let city = address.city?;
    Some(city)
}
```

### Pattern Guards
```rust
let num = Some(4);

match num {
    Some(x) if x < 5 => println!("less than five: {}", x),
    Some(x) => println!("{}", x),
    None => (),
}
```

### Matching Multiple Patterns
```rust
let x = 1;

match x {
    1 | 2 => println!("one or two"),
    3..=5 => println!("three through five"),
    _ => println!("anything else"),
}
```

---

## 🧠 Mental Model

Think of enums as:
- **Enum Definition** = A menu of possible variants
- **Enum Instance** = An order from that menu
- **Match Expression** = A waiter checking which order was made
- **Pattern Binding** = Unpacking the order to see what's inside

**Option vs Null:**
- Traditional null: "This might not exist, but good luck remembering to check!"
- Rust Option: "This might not exist, and the compiler FORCES you to handle both cases"

**Match vs If/Else Chain:**
- If/else: Manual checks, easy to miss cases
- Match: Compiler ensures exhaustiveness, impossible to miss cases

---

## 🔍 Why Enums Matter

### Solving the Billion Dollar Mistake
Tony Hoare (inventor of null references) called null his "billion-dollar mistake" because null pointer errors have caused countless bugs and crashes. Rust solves this with `Option<T>`:

```rust
// ❌ In languages with null, this can crash
String name = user.getName(); // Might be null!
int length = name.length();   // CRASH if null

// ✅ In Rust, compiler forces you to handle None
let name: Option<String> = user.get_name();
match name {
    Some(n) => println!("Length: {}", n.len()),
    None => println!("No name provided"),
}
```

### Modeling Domain Logic
Enums perfectly model "one of these things" logic:

```rust
// Payment methods in an e-commerce system
enum PaymentMethod {
    CreditCard { number: String, cvv: String },
    PayPal { email: String },
    Bitcoin { address: String },
    Cash,
}

// Game character actions
enum Action {
    Move { x: i32, y: i32 },
    Attack { target: String, damage: u32 },
    Heal { amount: u32 },
    Wait,
}
```

---

## 🎯 Real-World Applications

### AoC Pattern: Parsing Instructions
```rust
enum Instruction {
    Forward(i32),
    Down(i32),
    Up(i32),
}

fn parse_line(line: &str) -> Option<Instruction> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    match parts.as_slice() {
        ["forward", n] => Some(Instruction::Forward(n.parse().ok()?)),
        ["down", n] => Some(Instruction::Down(n.parse().ok()?)),
        ["up", n] => Some(Instruction::Up(n.parse().ok()?)),
        _ => None,
    }
}
```

### API Response Modeling
```rust
enum ApiResponse {
    Success { data: String },
    Error { code: u16, message: String },
    Loading,
}

fn handle_response(response: ApiResponse) {
    match response {
        ApiResponse::Success { data } => println!("Data: {}", data),
        ApiResponse::Error { code, message } => {
            println!("Error {}: {}", code, message)
        }
        ApiResponse::Loading => println!("Loading..."),
    }
}
```

---

## 📖 Further Reading
- [The Rust Book Chapter 6](https://doc.rust-lang.org/book/ch06-00-enums.html)
- [Rust Reference - Enums](https://doc.rust-lang.org/reference/items/enumerations.html)
- [Rust by Example - Enums](https://doc.rust-lang.org/rust-by-example/custom_types/enum.html)
- [Rust Reference - Pattern Matching](https://doc.rust-lang.org/reference/patterns.html)

---

## 🔗 Related Content

**Missions:**
- [[mission-3]] - LinkedList uses `Option<Box<Node<T>>>` for next pointers
- [[mission-2]] - Queue operations return `Option<T>` for empty cases
- [[mission-5]] - HashMap uses `Option<&V>` for get operations

**Daily Study:**
- [[daily-study/Day14]] - Uses `Result<T, E>` enum extensively
- [[daily-study/Day10]] - HashMap methods return `Option<T>`
- [[daily-study/Day11]] - Set methods use `Option` for lookups

**Next Steps:**
- Complete exercises in `Ch6/defining_enums/`, `Ch6/match_operator/`, `Ch6/if_let/`
- Review [[zettelkasten/rust_book/rust-book-ch7]] - Organizing code with modules
- Practice: Implement a state machine for a game or application

---

*This chapter introduces Rust's approach to null safety and algebraic data types, which are fundamental to writing safe, expressive Rust code. Enums combined with pattern matching eliminate entire classes of bugs common in other languages.*

*Links: [[Rust Book MOC]] | [[zettelkasten/rust_book/rust-book-ch5]] | [[zettelkasten/rust_book/rust-book-ch7]] | [[3-Track Integration]]*
*Tags: #rust-book #chapter6 #enums #pattern-matching #option #result #match #if-let #foundation*