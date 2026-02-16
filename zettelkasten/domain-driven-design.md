# Domain-Driven Design (DDD)

**Category**: Software Architecture, Design Patterns
**Tags**: #architecture #design-patterns #domain-modeling #encapsulation #abstraction

---

## Overview

**Domain-Driven Design (DDD)** is a software development approach that emphasizes modeling the problem domain through close collaboration between technical and domain experts, creating a shared "ubiquitous language" that bridges the gap between business requirements and code implementation.

**Core Philosophy**: The code should reflect the domain's mental model, not just implement technical requirements. Business concepts become first-class citizens in the codebase.

---

## Core Principles

### 1. Ubiquitous Language

**Definition**: A shared vocabulary used by both developers and domain experts that appears consistently in code, documentation, and conversations.

**Benefits**:
- Reduces translation errors between business and code
- Makes code self-documenting
- Enables domain experts to review code structure
- Facilitates effective communication

**Rust Example** (AoC 2022 Day 15):
```rust
// ❌ BAD: Implementation details leak into interface
pub struct Sensor {
    pub x: i32,
    pub y: i32,
    pub beacon_x: i32,
    pub beacon_y: i32,
}

// Usage exposes implementation
let dist = manhattan_distance(sensor.x, sensor.y, sensor.beacon_x, sensor.beacon_y);

// ✅ GOOD: Domain concept encapsulated
impl Sensor {
    /// Returns the detection radius of this sensor (Manhattan distance to its beacon)
    pub fn radius(&self) -> i32 {
        manhattan_distance(self.x, self.y, self.beacon_x, self.beacon_y)
    }
}

// Usage reflects domain language
let detection_radius = sensor.radius();  // "radius" is the domain term
```

**Why this matters**:
- Domain experts think: "What's the sensor's detection radius?"
- NOT: "What's the Manhattan distance from sensor coords to beacon coords?"
- The code mirrors the mental model

**AUTOSAR Analogy**: Like naming a runnable `CalculateEngineSpeed` instead of `Task_10ms_Priority5` - the domain concept (engine speed) is more meaningful than the implementation detail (10ms task).

---

### 2. Bounded Contexts

**Definition**: Explicit boundaries within which a particular domain model applies. Each bounded context has its own ubiquitous language and model.

**Pattern**: Different parts of a system may use the same term to mean different things. Bounded contexts prevent confusion by making these boundaries explicit.

**Rust Example** (Hypothetical e-commerce system):
```rust
// Inventory Context: "Product" means stock tracking
mod inventory {
    pub struct Product {
        pub sku: String,
        pub quantity_on_hand: u32,
        pub reorder_point: u32,
    }

    impl Product {
        pub fn needs_reorder(&self) -> bool {
            self.quantity_on_hand <= self.reorder_point
        }
    }
}

// Pricing Context: "Product" means pricing rules
mod pricing {
    pub struct Product {
        pub sku: String,
        pub base_price: Decimal,
        pub discount_tier: DiscountTier,
    }

    impl Product {
        pub fn calculate_price(&self, quantity: u32) -> Decimal {
            // Different logic, different concerns
            self.base_price * quantity * self.discount_tier.multiplier()
        }
    }
}

// Each context has its own Product model - no confusion!
```

**AUTOSAR Analogy**: Like having separate SWCs for different ECU functions:
- `EngineMgmt_SWC`: "Temperature" means coolant temp
- `HVAC_SWC`: "Temperature" means cabin air temp
- Same term, different bounded contexts, different models

---

### 3. Entities vs Value Objects

#### Entities

**Definition**: Objects defined by their identity, not their attributes. Two entities with the same data but different IDs are distinct.

**Characteristics**:
- Has unique identifier
- Mutable over time
- Identity persists through state changes
- Equality based on ID, not data

**Rust Pattern**:
```rust
#[derive(Debug, Clone)]
pub struct User {
    id: UserId,  // Identity!
    email: String,
    name: String,
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id  // Equality by ID, not data
    }
}

// Two users with same email are DIFFERENT if IDs differ
let user1 = User { id: UserId(1), email: "test@example.com".into(), name: "Alice".into() };
let user2 = User { id: UserId(2), email: "test@example.com".into(), name: "Alice".into() };
assert_ne!(user1, user2);  // Different entities!

// Same user after name change is SAME entity
let mut user3 = User { id: UserId(1), email: "test@example.com".into(), name: "Alice".into() };
let user3_copy = user3.clone();
user3.name = "Alice Smith".into();
assert_eq!(user3.id, user3_copy.id);  // Still the same user!
```

#### Value Objects

**Definition**: Objects defined entirely by their attributes. Two value objects with the same data are considered identical.

**Characteristics**:
- No unique identifier
- Immutable (in principle)
- Equality based on all attributes
- Can be freely copied/replaced

**Rust Pattern**:
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

// Two coords with same values are IDENTICAL
let pos1 = Coord { x: 10, y: 20 };
let pos2 = Coord { x: 10, y: 20 };
assert_eq!(pos1, pos2);  // Same value object!

// Value objects are interchangeable
fn distance_from_origin(pos: Coord) -> f64 {
    ((pos.x.pow(2) + pos.y.pow(2)) as f64).sqrt()
}

// Doesn't matter WHICH (10, 20) we pass - they're identical
assert_eq!(distance_from_origin(pos1), distance_from_origin(pos2));
```

**AoC Day 15 Example**:
```rust
// Entity: Sensor (has identity - position defines it)
#[derive(Debug, Clone)]
pub struct Sensor {
    pub x: i32,
    pub y: i32,
    pub beacon_x: i32,
    pub beacon_y: i32,
}

// Value Object: (i32, i32) interval
// Two intervals (10, 20) are identical regardless of which sensor produced them
type Interval = (i32, i32);
```

**AUTOSAR Analogy**:
- **Entity**: ECU instance (each ECU has unique hardware ID, serial number)
- **Value Object**: Temperature reading (25°C from any sensor is the same value)

---

### 4. Aggregates

**Definition**: A cluster of domain objects (entities and value objects) treated as a single unit for data changes. One entity serves as the "aggregate root" - the only entry point for external access.

**Rules**:
- External objects can only reference the aggregate root
- Internal objects can reference each other
- Invariants are maintained by the root
- Transactions should not span multiple aggregates

**Rust Example** (Order management):
```rust
#[derive(Debug)]
pub struct Order {
    id: OrderId,
    customer_id: CustomerId,
    items: Vec<OrderItem>,  // Internal to aggregate
    status: OrderStatus,
    total: Decimal,
}

#[derive(Debug)]
struct OrderItem {  // Not pub! Only accessible through Order
    product_id: ProductId,
    quantity: u32,
    price_at_order: Decimal,
}

impl Order {
    // ✅ Aggregate root enforces invariants
    pub fn add_item(&mut self, product_id: ProductId, quantity: u32, price: Decimal) -> Result<(), OrderError> {
        if self.status != OrderStatus::Draft {
            return Err(OrderError::OrderAlreadySubmitted);
        }

        let item = OrderItem { product_id, quantity, price_at_order: price };
        self.total += price * Decimal::from(quantity);
        self.items.push(item);
        Ok(())
    }

    pub fn submit(&mut self) -> Result<(), OrderError> {
        if self.items.is_empty() {
            return Err(OrderError::EmptyOrder);
        }

        self.status = OrderStatus::Submitted;
        Ok(())
    }

    // ❌ No public access to modify items directly!
    // Items can only be modified through Order methods
}

// External code cannot create OrderItem directly
// External code cannot modify items Vec directly
// All changes go through Order (aggregate root)
```

**Benefits**:
- Consistency: Invariants always maintained (e.g., total = sum of items)
- Encapsulation: Implementation details hidden
- Transactional boundaries: One aggregate = one transaction

**AUTOSAR Analogy**: Like a Complex Device Driver SWC:
- Aggregate root = SWC's provided port interface (only external entry point)
- Internal entities = internal runnables, local variables
- Invariants = SWC's state machine consistency
- External components can't bypass the port to access internals

---

### 5. Domain Services

**Definition**: Operations that don't naturally belong to any entity or value object. Stateless operations that coordinate between entities or perform domain logic.

**When to use**:
- Operation involves multiple entities
- Operation doesn't fit conceptually on any single entity
- Operation is a domain concept but not a "thing"

**Rust Example**:
```rust
// ❌ BAD: Forcing operation onto an entity
impl User {
    pub fn transfer_money_to(&mut self, other: &mut User, amount: Decimal) -> Result<(), TransferError> {
        // Awkward: User shouldn't know how to transfer money
        // This is a banking operation, not a user property
    }
}

// ✅ GOOD: Domain service for cross-entity operations
pub struct MoneyTransferService {
    transaction_log: TransactionLog,
}

impl MoneyTransferService {
    pub fn transfer(
        &mut self,
        from: &mut Account,
        to: &mut Account,
        amount: Decimal,
    ) -> Result<TransactionId, TransferError> {
        // Validate business rules
        if amount <= Decimal::ZERO {
            return Err(TransferError::InvalidAmount);
        }

        if from.balance < amount {
            return Err(TransferError::InsufficientFunds);
        }

        // Perform transfer (coordinating multiple entities)
        from.withdraw(amount)?;
        to.deposit(amount)?;

        // Log transaction (service responsibility)
        let tx_id = self.transaction_log.record(from.id, to.id, amount);

        Ok(tx_id)
    }
}
```

**AoC Day 15 Example**:
```rust
// Domain service: Interval merging (operates on collections)
pub fn merge_intervals(intervals: &[(i32, i32)]) -> Vec<(i32, i32)> {
    // This doesn't belong to Sensor or Interval individually
    // It's a domain operation on collections of intervals
    // Service pattern is appropriate
}
```

**AUTOSAR Analogy**: Like a Mode Manager SWC that coordinates state transitions across multiple SWCs - it's not part of any single SWC but orchestrates them.

---

### 6. Repositories

**Definition**: Abstraction for accessing aggregates. Provides collection-like interface for querying and persisting domain objects while hiding storage details.

**Pattern**: Separate domain logic from persistence mechanism.

**Rust Example**:
```rust
// Repository trait (domain layer)
pub trait UserRepository {
    fn find_by_id(&self, id: UserId) -> Result<Option<User>, RepoError>;
    fn find_by_email(&self, email: &str) -> Result<Option<User>, RepoError>;
    fn save(&mut self, user: &User) -> Result<(), RepoError>;
    fn delete(&mut self, id: UserId) -> Result<(), RepoError>;
}

// Concrete implementation (infrastructure layer)
pub struct PostgresUserRepository {
    pool: PgPool,
}

impl UserRepository for PostgresUserRepository {
    fn find_by_id(&self, id: UserId) -> Result<Option<User>, RepoError> {
        // SQL queries here, but domain layer doesn't know about Postgres
        todo!()
    }
    // ... other methods
}

// Domain service uses trait, not concrete type
pub struct UserService<R: UserRepository> {
    repo: R,
}

impl<R: UserRepository> UserService<R> {
    pub fn change_user_email(&mut self, id: UserId, new_email: String) -> Result<(), ServiceError> {
        let mut user = self.repo.find_by_id(id)?
            .ok_or(ServiceError::UserNotFound)?;

        user.change_email(new_email)?;  // Domain logic
        self.repo.save(&user)?;  // Persistence

        Ok(())
    }
}
```

**Benefits**:
- Testability: Mock repository for unit tests
- Flexibility: Swap Postgres for MongoDB without changing domain logic
- Separation: Domain focused on business rules, not database queries

---

## Practical Rust Patterns

### Encapsulation via Methods

**AoC Day 15 Case Study**: Why `Sensor::radius()` instead of direct field access?

```rust
// Current implementation
impl Sensor {
    pub fn radius(&self) -> i32 {
        manhattan_distance(self.x, self.y, self.beacon_x, self.beacon_y)
    }
}

// Usage
let r = sensor.radius();
```

**DDD Benefits**:

1. **Semantic Clarity**: "radius" is domain language, "manhattan_distance" is implementation detail
2. **Encapsulation**: If we later cache radius, callers don't change
3. **Information Hiding**: Caller doesn't need to know radius = distance to beacon
4. **DRY**: Calculation logic in one place, not scattered across codebase
5. **Future-Proofing**: Can add validation, logging, caching without breaking callers

**Evolution Example**:
```rust
// Version 1: Simple calculation
impl Sensor {
    pub fn radius(&self) -> i32 {
        manhattan_distance(self.x, self.y, self.beacon_x, self.beacon_y)
    }
}

// Version 2: Add caching (no caller changes!)
pub struct Sensor {
    pub x: i32,
    pub y: i32,
    pub beacon_x: i32,
    pub beacon_y: i32,
    cached_radius: Option<i32>,  // New field
}

impl Sensor {
    pub fn radius(&mut self) -> i32 {
        *self.cached_radius.get_or_insert_with(|| {
            manhattan_distance(self.x, self.y, self.beacon_x, self.beacon_y)
        })
    }
}

// Version 3: Add validation (still no caller changes!)
impl Sensor {
    pub fn radius(&self) -> i32 {
        let r = manhattan_distance(self.x, self.y, self.beacon_x, self.beacon_y);
        assert!(r >= 0, "Radius cannot be negative");
        r
    }
}
```

**Caller code never changes** - this is the power of encapsulation!

### Type-Driven Design

**Pattern**: Use Rust's type system to enforce domain invariants at compile time.

```rust
// ❌ BAD: Primitives expose invariant violations
fn create_user(email: String, age: i32) -> Result<User, UserError> {
    if !email.contains('@') {
        return Err(UserError::InvalidEmail);
    }
    if age < 0 || age > 150 {
        return Err(UserError::InvalidAge);
    }
    // ...
}

// ✅ GOOD: Types enforce invariants
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Email(String);

impl Email {
    pub fn new(s: String) -> Result<Self, EmailError> {
        if !s.contains('@') {
            return Err(EmailError::MissingAtSign);
        }
        Ok(Email(s))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Age(u8);  // 0-255, closer to valid range

impl Age {
    pub fn new(value: u8) -> Result<Self, AgeError> {
        if value > 150 {
            return Err(AgeError::TooOld);
        }
        Ok(Age(value))
    }
}

// Now the function signature GUARANTEES valid data
fn create_user(email: Email, age: Age) -> User {
    User { email, age }  // No validation needed!
}
```

**Benefit**: Invalid states are unrepresentable. Compiler enforces domain rules.

### Builder Pattern for Complex Entities

**Pattern**: Use builder to construct entities with many optional fields while maintaining invariants.

```rust
pub struct Sensor {
    x: i32,
    y: i32,
    beacon_x: i32,
    beacon_y: i32,
    name: Option<String>,
    calibration: Option<CalibrationData>,
}

pub struct SensorBuilder {
    x: Option<i32>,
    y: Option<i32>,
    beacon_x: Option<i32>,
    beacon_y: Option<i32>,
    name: Option<String>,
    calibration: Option<CalibrationData>,
}

impl SensorBuilder {
    pub fn new() -> Self {
        Self {
            x: None,
            y: None,
            beacon_x: None,
            beacon_y: None,
            name: None,
            calibration: None,
        }
    }

    pub fn position(mut self, x: i32, y: i32) -> Self {
        self.x = Some(x);
        self.y = Some(y);
        self
    }

    pub fn beacon(mut self, x: i32, y: i32) -> Self {
        self.beacon_x = Some(x);
        self.beacon_y = Some(y);
        self
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn build(self) -> Result<Sensor, BuildError> {
        Ok(Sensor {
            x: self.x.ok_or(BuildError::MissingPosition)?,
            y: self.y.ok_or(BuildError::MissingPosition)?,
            beacon_x: self.beacon_x.ok_or(BuildError::MissingBeacon)?,
            beacon_y: self.beacon_y.ok_or(BuildError::MissingBeacon)?,
            name: self.name,
            calibration: self.calibration,
        })
    }
}

// Usage
let sensor = SensorBuilder::new()
    .position(2, 18)
    .beacon(-2, 15)
    .name("Sensor-Alpha".into())
    .build()?;
```

---

## AUTOSAR Parallels

| DDD Concept | AUTOSAR Equivalent | Example |
|-------------|-------------------|---------|
| **Ubiquitous Language** | SWC naming conventions | `BMS_CalcStateOfCharge` (domain: battery management) |
| **Bounded Context** | ECU/SWC boundaries | `EngineMgmt_SWC` vs `Transmission_SWC` |
| **Entity** | SWC instance with unique ID | Each ECU instance has unique network address |
| **Value Object** | Signal data type | `Temperature` (25°C is same from any sensor) |
| **Aggregate** | Complex Device Driver | Only port provides access, internals hidden |
| **Domain Service** | Mode Manager / Coordinator | Orchestrates state transitions across SWCs |
| **Repository** | NvM (Non-Volatile Memory) abstraction | Read/write data without knowing EEPROM vs Flash |
| **Encapsulation** | Port/Interface pattern | External access only through provided/required ports |

**Key Insight**: AUTOSAR already applies many DDD principles! The component-based architecture with strict interfaces mirrors DDD's emphasis on bounded contexts and encapsulation.

---

## When to Use DDD

### ✅ Good Fit

- **Complex domain logic**: Business rules are complicated and evolving
- **Long-lived systems**: Code will be maintained for years
- **Collaborative teams**: Developers + domain experts working together
- **Unclear requirements**: Domain model emerges through exploration

### ❌ Poor Fit

- **Simple CRUD apps**: Just mapping database to UI (no complex logic)
- **Short-term projects**: Overhead not justified for 2-week prototype
- **No domain experts**: Can't establish ubiquitous language without them
- **Purely technical problems**: Data processing pipelines, compilers, etc.

---

## Common Pitfalls

### Anemic Domain Model

**Anti-pattern**: Entities are just data containers, all logic in services.

```rust
// ❌ ANEMIC: User is just data
pub struct User {
    pub id: UserId,
    pub email: String,
    pub status: AccountStatus,
}

pub struct UserService {
    // All logic here!
}

impl UserService {
    pub fn suspend_user(&self, user: &mut User, reason: String) {
        user.status = AccountStatus::Suspended;
        // Bypasses any business rules User might enforce
    }
}

// ✅ RICH: User enforces its own invariants
impl User {
    pub fn suspend(&mut self, reason: SuspensionReason) -> Result<(), UserError> {
        match self.status {
            AccountStatus::Active => {
                self.status = AccountStatus::Suspended { reason };
                Ok(())
            }
            AccountStatus::Suspended { .. } => Err(UserError::AlreadySuspended),
            AccountStatus::Deleted => Err(UserError::AccountDeleted),
        }
    }
}
```

### Over-Engineering

**Anti-pattern**: Applying DDD patterns to simple problems.

```rust
// ❌ OVERKILL: Builder for 2 fields
let coord = CoordBuilder::new()
    .x(10)
    .y(20)
    .build()?;

// ✅ SIMPLE: Just use struct literal
let coord = Coord { x: 10, y: 20 };
```

**Rule**: Start simple, add patterns when complexity demands them.

---

## Related Patterns

- [[philosophy-of-software-design]] - **Complementary approach** - How to design modules (deep interfaces, information hiding). DDD provides *what* to model, PoSD provides *how* to structure modules.
- [[software-architecture-patterns]] - Feature-based architecture aligns with bounded contexts
- [[Clean Code Principles]] - Rich domain models example
- [[common-traits-pattern]] - Implementing traits for domain types
- [[mission-4]] - Rc<RefCell<T>> for shared mutable domain objects
- [[mission-10]] - Union-Find as domain service pattern

---

## Summary

**Domain-Driven Design** bridges the gap between business requirements and code by:

1. **Ubiquitous Language**: Shared vocabulary in code and conversations
2. **Strategic Design**: Bounded contexts define model boundaries
3. **Tactical Patterns**: Entities, value objects, aggregates, services, repositories
4. **Encapsulation**: Hide implementation, expose domain concepts
5. **Type Safety**: Use Rust's type system to enforce invariants

**Key Insight from AoC Day 15**: The simple `Sensor::radius()` method demonstrates DDD in action - domain language (radius) over implementation detail (Manhattan distance calculation).

**AUTOSAR Connection**: Component-based architecture already embodies many DDD principles - bounded contexts (SWCs), encapsulation (ports), and domain-driven naming.

---

**References**:
- Eric Evans, *Domain-Driven Design: Tackling Complexity in the Heart of Software* (2003)
- Vaughn Vernon, *Implementing Domain-Driven Design* (2013)

**Last Updated**: 2026-02-15
