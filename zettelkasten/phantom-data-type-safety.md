# 🎭 PhantomData Type Safety Patterns

**Using PhantomData for zero-cost type safety and compile-time guarantees**

## 🎯 Core Concept

`PhantomData<T>` is a zero-sized type that "pretends" to hold a type `T` without actually storing it. It enables type-level programming and compile-time type safety without runtime overhead.

## 🔍 The Problem PhantomData Solves

### **Without Type Safety:**

```rust
// Easy to mix up different types of IDs
fn process_user(id: u64) { ... }
fn process_product(id: u64) { ... }

let user_id = 123;
let product_id = 456;

// Oops! Bug that compiles fine
process_user(product_id);  // ❌ Wrong ID type, but no error
```

### **With PhantomData:**

```rust
struct Id<T> {
    value: u64,
    _phantom: PhantomData<T>,
}

type UserId = Id<User>;
type ProductId = Id<Product>;

// Compiler prevents mixing up ID types
process_user(user_id);     // ✅ Correct
process_user(product_id);  // ❌ Compile error! Type mismatch
```

## 🛠️ Basic PhantomData Pattern

### **Generic Struct with PhantomData:**

```rust
use std::marker::PhantomData;

struct Id<T> {
    value: u64,
    _phantom: PhantomData<T>,  // Type marker (zero-sized)
}

impl<T> Id<T> {
    fn new(value: u64) -> Self {
        Id {
            value,
            _phantom: PhantomData,  // Type inferred automatically
        }
    }
    
    fn get(&self) -> u64 {
        self.value
    }
}
```

### **Type Aliases for Safety:**

```rust
type UserId = Id<User>;
type ProductId = Id<Product>;
type OrderId = Id<Order>;

struct User { name: String }
struct Product { name: String }
struct Order { id: u64 }
```

## 🎯 How PhantomData Creates Type Uniqueness

### **The Magic of Generic Type Substitution:**

```rust
// Generic template
Id<T> {
    value: u64,
    _phantom: PhantomData<T>,
}

// Becomes unique concrete types:
Id<User> {
    value: u64,
    _phantom: PhantomData<User>,    // User-specific marker
}

Id<Product> {
    value: u64,
    _phantom: PhantomData<Product>, // Product-specific marker
}
```

**Key Insight:** Each `T` creates a completely different type, even with identical structure!

## 🚀 Real-World Applications

### **1. Database Entity IDs**

```rust
type UserId = Id<User>;
type OrderId = Id<Order>;
type ProductId = Id<Product>;

fn get_user(id: UserId) -> Option<User> { ... }
fn get_order(id: OrderId) -> Option<Order> { ... }
fn get_product(id: ProductId) -> Option<Product> { ... }

// Type safety prevents wrong entity lookups
let user = get_user(order_id);  // ❌ Compile error!
```

### **2. API Endpoint Safety**

```rust
fn delete_user(id: UserId) -> Result<(), Error> { ... }
fn delete_product(id: ProductId) -> Result<(), Error> { ... }

// Can't accidentally call wrong endpoint
delete_user(product_id);  // ❌ Compile error!
```

### **3. Resource Handles**

```rust
struct FileHandle<T> {
    fd: i32,
    _phantom: PhantomData<T>,
}

type ReadHandle = FileHandle<ReadMode>;
type WriteHandle = FileHandle<WriteMode>;

fn read_data(handle: ReadHandle) -> Vec<u8> { ... }
fn write_data(handle: WriteHandle, data: &[u8]) { ... }

// Prevents using read handle for writing
write_data(read_handle, &data);  // ❌ Compile error!
```

### **4. Unit Type Safety**

```rust
struct Length<T> {
    value: f64,
    _phantom: PhantomData<T>,
}

type Meters = Length<MeterUnit>;
type Feet = Length<FootUnit>;

fn add_lengths(a: Meters, b: Meters) -> Meters {
    Length::new(a.value + b.value)
}

// Can't add meters and feet
let total = add_lengths(meters, feet);  // ❌ Compile error!
```

## 🔧 Advanced Patterns

### **PhantomData with Lifetime Parameters**

```rust
struct Cursor<'a, T> {
    data: &'a [T],
    position: usize,
    _phantom: PhantomData<&'a T>,
}

impl<'a, T> Cursor<'a, T> {
    fn new(data: &'a [T]) -> Self {
        Cursor {
            data,
            position: 0,
            _phantom: PhantomData,
        }
    }
}
```

### **PhantomData with Multiple Type Parameters**

```rust
struct Container<K, V> {
    data: Vec<u8>,
    _phantom: PhantomData<(K, V)>,
}

type UserMap = Container<String, User>;
type ProductMap = Container<u64, Product>;
```

### **PhantomData with Trait Bounds**

```rust
struct Processor<T: Processable> {
    config: Config,
    _phantom: PhantomData<T>,
}

trait Processable {
    fn process(&self) -> Result<(), Error>;
}

impl<T: Processable> Processor<T> {
    fn new(config: Config) -> Self {
        Processor {
            config,
            _phantom: PhantomData,
        }
    }
}
```

## 🎯 PhantomData Variants

### **PhantomData<T> - Owns T**

```rust
struct Owns<T> {
    data: u32,
    _phantom: PhantomData<T>,  // Indicates ownership of T
}
```

### **PhantomData<&T> - Borrows T**

```rust
struct Borrows<T> {
    data: u32,
    _phantom: PhantomData<&T>,  // Indicates borrowing of T
}
```

### **PhantomData<&mut T> - Mutably Borrows T**

```rust
struct MutBorrows<T> {
    data: u32,
    _phantom: PhantomData<&mut T>,  // Indicates mutable borrowing
}
```

## 💡 Best Practices

### **1. Use Underscore Prefix**

```rust
struct Id<T> {
    value: u64,
    _phantom: PhantomData<T>,  // ✅ _phantom indicates unused
}
```

### **2. Document the Purpose**

```rust
struct Id<T> {
    value: u64,
    /// PhantomData ensures type safety without runtime cost
    _phantom: PhantomData<T>,
}
```

### **3. Consider Type Aliases**

```rust
// Make intent clear with type aliases
type UserId = Id<User>;
type ProductId = Id<Product>;
```

### **4. Add Convenience Methods**

```rust
impl<T> Id<T> {
    fn cast<U>(self) -> Id<U> {
        Id::new(self.value)
    }
    
    fn eq(&self, other: &Id<T>) -> bool {
        self.value == other.value
    }
}
```

## 🧪 Testing PhantomData Patterns

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_safety() {
        let user_id = UserId::new(123);
        let product_id = ProductId::new(123);
        
        // Same value, different types
        assert_eq!(user_id.get(), product_id.get());
        
        // But they're different types
        // assert_eq!(user_id, product_id);  // ❌ Compile error!
    }

    #[test]
    fn test_casting() {
        let user_id = UserId::new(123);
        let product_id = user_id.cast::<Product>();
        
        assert_eq!(user_id.get(), product_id.get());
    }
}
```

## 🔗 Integration with Learning Tracks

### **Mission Integration**

- **Mission 5**: Type-safe HashMap keys with PhantomData
- **Mission 6**: Grid coordinates with unit type safety
- **Advanced Missions**: Resource management with type safety

### **Daily Study Integration**

- **Day 15**: Generics and type parameters
- **Day 16**: Advanced type system patterns
- **Day 17**: Zero-cost abstractions

### **AoC Applications**

- Type-safe coordinate systems
- Entity ID management
- Resource handle safety

## 📚 Further Reading

- [[daily-study/Day15]] - Foundation for PhantomData
- [[daily-study/Day16]] - Complex type system usage
- [[zero-cost-abstractions]] - Performance characteristics
- [[Type Safety Patterns]] - Compile-time guarantees
- [[variance]] - Using PhantomData to control variance in custom types

## 🎓 Key Takeaways

1. **Zero Runtime Cost** - PhantomData is zero-sized
2. **Compile-Time Safety** - Prevents type confusion
3. **Type Uniqueness** - Each generic instantiation is unique
4. **Clear Intent** - Self-documenting code
5. **Refactoring Safety** - Changes caught at compile time

---

*PhantomData enables powerful type-level programming patterns that provide compile-time safety without runtime overhead. Essential for building robust, type-safe systems.*

*Tags: #phantomdata #type-safety #generics #zero-cost #compile-time #patterns #advanced*
*Links: [[zettel-index]] | [[rust-concepts-MOC]] | [[daily-study/Day15]] | [[zero-cost-abstractions]]*
