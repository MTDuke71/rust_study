# Software Architecture Patterns

*Created: 2025-11-07*  
*Tags: #software-architecture #design-patterns #layer-based #feature-based #project-organization #rust #software-engineering*

## Overview

Software architecture patterns define how code is organized at the highest level. The two most common organizational patterns are **layer-based** and **feature-based** architecture, each with distinct trade-offs for maintainability, scalability, and team collaboration.

## Layer-Based Architecture

### Definition
Layer-based architecture organizes code by **technical concerns**, creating horizontal layers where each layer has a specific technical responsibility.

### Structure
```
src/
├── controllers/     # HTTP handlers, API endpoints
├── services/        # Business logic, application services  
├── repositories/    # Data access, database operations
├── models/          # Data structures, entities
├── middleware/      # Cross-cutting concerns
└── utils/           # Shared utilities
```

### Characteristics
- **Horizontal separation** by technical function
- **Dependency flow**: Controllers → Services → Repositories → Models
- **Cross-cutting concerns** handled by dedicated layers
- **Familiar pattern** from traditional enterprise development

### Advantages
- ✅ **Clear separation of concerns** - each layer has single responsibility
- ✅ **Easy to understand** - intuitive for developers familiar with MVC patterns
- ✅ **Consistent patterns** - same structure applied across all features
- ✅ **Simple dependency management** - clear layered dependencies
- ✅ **Good for CRUD operations** - natural fit for database-centric applications
- ✅ **Easy to apply middleware** - authentication, logging, validation

### Disadvantages
- ❌ **Cross-cutting feature changes are expensive** - requires touching multiple layers
- ❌ **Tight vertical coupling** - changes propagate up/down the stack
- ❌ **Poor team parallelization** - multiple teams modify same layers
- ❌ **Features scattered across directories** - hard to understand business domains
- ❌ **Promotes god objects** - services become large and generic
- ❌ **Difficult feature removal** - feature code spread across all layers

### Best Use Cases
- Small to medium applications (< 50k LOC)
- CRUD-heavy applications
- Simple business domains
- Teams new to the codebase
- Strict regulatory environments requiring clear audit trails

## Feature-Based Architecture

### Definition
Feature-based architecture organizes code by **business capabilities**, creating vertical slices where each feature contains all necessary technical layers.

### Structure
```
src/
├── user/
│   ├── user_controller.rs
│   ├── user_service.rs
│   ├── user_repository.rs
│   ├── user_model.rs
│   └── mod.rs
├── order/
│   ├── order_controller.rs
│   ├── order_service.rs
│   ├── order_repository.rs
│   └── mod.rs
├── payment/
│   └── ...
└── shared/
    ├── database/
    ├── auth/
    └── validation/
```

### Characteristics
- **Vertical separation** by business capability
- **Self-contained features** with internal layering
- **Minimal shared dependencies** - shared module for truly common code
- **Business domain alignment** - structure reflects user-facing features

### Advantages
- ✅ **High cohesion** - related code lives together
- ✅ **Low coupling** - features are independent
- ✅ **Parallel development** - teams can work on different features simultaneously
- ✅ **Clear business boundaries** - structure matches domain model
- ✅ **Easy feature modification** - contained impact radius
- ✅ **Microservices ready** - features can be extracted as services
- ✅ **Better testability** - features can be tested in isolation
- ✅ **Scalable team structure** - teams can own entire features

### Disadvantages
- ❌ **Potential code duplication** - without careful shared module design
- ❌ **Cross-cutting concerns complexity** - harder to apply consistent patterns
- ❌ **Requires discipline** - developers must avoid circular dependencies
- ❌ **Less familiar** - many developers expect layer-based organization
- ❌ **Shared code extraction** - determining what should be shared vs duplicated

### Best Use Cases
- Large applications (> 50k LOC)
- Complex business domains
- Multiple development teams
- Microservices architecture plans
- Domain-driven design approach
- Features with different lifecycles

## Hybrid Approaches

### Screaming Architecture
Combines both patterns by organizing around features but maintaining clear technical boundaries:

```
src/
├── features/
│   ├── user_management/
│   │   ├── controllers/
│   │   ├── services/
│   │   └── repositories/
│   └── order_processing/
├── shared/
│   ├── database/
│   ├── auth/
│   └── middleware/
└── infrastructure/
    ├── web/
    └── config/
```

### Package by Feature with Layers
Features as primary organization, layers within features:

```
src/
├── user/
│   ├── api/           # Controllers
│   ├── domain/        # Business logic
│   ├── data/          # Repositories
│   └── mod.rs
├── order/
│   └── ...
└── shared/
```

## Rust-Specific Considerations

### Module System Benefits
Rust's module system naturally supports feature-based architecture:
- **Clear boundaries** with `mod.rs` files
- **Explicit privacy** with `pub` visibility
- **Dependency control** through module imports
- **Compile-time enforcement** of architectural rules

### Ownership Model
Rust's ownership model encourages good architectural practices:
- **Clear interfaces** between features
- **Minimal shared mutable state**
- **Zero-cost abstractions** for shared utilities
- **Memory safety** without runtime overhead

### Cargo Workspaces
Large feature-based applications can use workspaces:
```toml
[workspace]
members = [
    "user-service",
    "order-service", 
    "payment-service",
    "shared-models"
]
```

### Example: Mission 10 Structure Analysis
The Union-Find implementation follows feature-based principles:
```
missions/Mission10/
├── src/
│   ├── union_find.rs      # Core feature implementation
│   ├── undo_union_find.rs # Extended feature
│   ├── iterators.rs       # Feature-specific utilities
│   └── lib.rs            # Public API boundary
├── examples/              # Feature demonstrations
├── tests/                 # Feature-focused testing
│   ├── unit_tests.rs      # Core functionality
│   ├── integration_tests.rs # Real-world scenarios
│   └── property_tests.rs  # Mathematical properties
└── benches/              # Performance validation
```

This structure demonstrates:
- **Feature cohesion** - all Union-Find code in one place
- **Clear boundaries** - public API through lib.rs
- **Self-contained testing** - tests specific to Union-Find
- **Domain alignment** - structure matches the data structure being implemented

## Decision Framework

### Choose Layer-Based When:
- ✅ Application size < 50k lines of code
- ✅ Simple CRUD operations dominate
- ✅ Single team development
- ✅ Traditional enterprise patterns required
- ✅ Heavy database-centric operations
- ✅ Team unfamiliar with domain-driven design

### Choose Feature-Based When:
- ✅ Application size > 50k lines of code
- ✅ Complex business domains with distinct capabilities
- ✅ Multiple teams working simultaneously
- ✅ Microservices migration planned
- ✅ Domain-driven design approach
- ✅ Features have different release cycles
- ✅ Strong business domain boundaries exist

### Migration Strategy
Moving from layer-based to feature-based:
1. **Identify feature boundaries** through domain analysis
2. **Extract one feature at a time** - gradual migration
3. **Create shared modules** for truly common code
4. **Establish dependency rules** - prevent circular references
5. **Update build/test processes** - ensure feature isolation
6. **Team training** - educate developers on new patterns

## Best Practices

### For Layer-Based Architecture:
- Keep layers thin and focused
- Avoid god objects in service layers
- Use dependency injection for testability
- Implement clear interfaces between layers
- Document cross-cutting concerns

### For Feature-Based Architecture:
- Define clear feature boundaries early
- Extract shared code judiciously - avoid premature optimization
- Use dependency inversion for shared services
- Establish conventions for inter-feature communication
- Monitor for circular dependencies

### Universal Principles:
- **Single Responsibility Principle** - whether layers or features
- **Dependency Inversion** - depend on abstractions, not concretions
- **Interface Segregation** - small, focused interfaces
- **Don't Repeat Yourself** - but prefer duplication over wrong abstraction
- **Explicit over Implicit** - clear dependencies and boundaries

## Common Anti-Patterns

### Layer-Based Anti-Patterns:
- **Leaky Abstractions** - business logic in controllers
- **God Services** - massive service classes doing everything
- **Data Transfer Object Explosion** - excessive mapping between layers
- **Circular Dependencies** - layers depending on higher layers

### Feature-Based Anti-Patterns:
- **Big Ball of Mud Features** - features that are too large
- **Shared Everything** - over-sharing defeats the purpose
- **Circular Feature Dependencies** - features depending on each other
- **Inconsistent Internal Structure** - different patterns within features

## Related Concepts

### Domain-Driven Design (DDD)
Feature-based architecture aligns well with DDD principles:
- **Bounded Contexts** map to features
- **Aggregates** define feature boundaries
- **Domain Services** live within features
- **Anti-Corruption Layers** at feature boundaries

### Hexagonal Architecture
Can be applied within features:
- **Domain core** - business logic
- **Ports** - interfaces to external systems
- **Adapters** - concrete implementations
- **Feature boundary** - hexagon boundary

### Clean Architecture
Similar principles but different organization:
- **Entities** - business objects
- **Use Cases** - application services
- **Interface Adapters** - controllers/presenters
- **Frameworks** - external concerns

## Measuring Success

### Metrics for Good Architecture:
- **Change Impact Radius** - how many files need modification for typical changes
- **Team Parallelization** - can teams work without conflicts
- **Build Time** - does structure support incremental compilation
- **Test Isolation** - can features be tested independently
- **Code Ownership** - clear responsibility boundaries

### Warning Signs:
- **Shotgun Surgery** - small changes require many file modifications
- **Merge Conflicts** - frequent conflicts between team members
- **Integration Complexity** - difficult to combine feature work
- **Technical Debt Growth** - shortcuts accumulate due to structure

## Tools and Techniques

### Architecture Decision Records (ADRs)
Document architectural choices:
```markdown
# ADR 001: Feature-Based Architecture

## Status: Accepted

## Context
We need to support multiple teams working on different business capabilities...

## Decision
We will organize code by business features rather than technical layers...

## Consequences
Positive: Better team autonomy, clearer business alignment
Negative: Need to establish shared module conventions
```

### Dependency Analysis Tools
For Rust projects:
- `cargo-deps` - visualize dependency graphs
- `cargo-modules` - analyze module structure  
- `cargo-audit` - security dependency analysis
- Custom linting rules for architectural boundaries

### Code Organization Scripts
Automate structure validation:
```bash
# Check for circular dependencies
cargo deps --all-deps | grep -E "cycle|circular"

# Validate feature isolation
find src/features -name "*.rs" -exec grep -l "use.*features::" {} \;
```

## Real-World Examples

### Layer-Based Success Stories:
- **Ruby on Rails** - convention over configuration
- **Spring Boot** - enterprise Java applications
- **ASP.NET MVC** - Microsoft web applications

### Feature-Based Success Stories:
- **Netflix** - microservices architecture
- **Shopify** - e-commerce platform modules
- **GitHub** - feature-based development teams

### Rust Examples:
- **Cargo** - feature-based with shared utilities
- **Rustc** - compiler phases with shared infrastructure
- **Tokio** - runtime components as features

## Future Considerations

### Emerging Patterns:
- **Micro-frontends** - feature-based UI architecture
- **Event-driven architecture** - features communicate via events  
- **CQRS/Event Sourcing** - separate read/write feature models
- **Serverless functions** - ultimate feature isolation

### Technology Trends:
- **Containerization** - features as deployable units
- **Service mesh** - infrastructure for feature communication
- **GraphQL** - unified API over feature-based services
- **WebAssembly** - features as portable modules

## Conclusion

The choice between layer-based and feature-based architecture fundamentally comes down to **optimizing for change**. Layer-based architecture optimizes for consistency and simplicity in small applications. Feature-based architecture optimizes for team autonomy and business alignment in complex applications.

**Modern recommendation**: Start with feature-based architecture for new projects, especially in Rust where the module system provides excellent support for this pattern. The benefits of business alignment, team autonomy, and maintainability typically outweigh the initial complexity.

The key insight is that **architecture is not about the code - it's about the people**. Choose the pattern that best supports how your team works and how your business evolves.

---

## Links

**Architecture Patterns**:
- [[Clean Code Principles]] - Code-level design principles
- [[V-Cycle Methodology]] - Software development process
- [[TDD (Test-Driven Development)]] - Testing strategies for different architectures

**Rust-Specific**:
- [[rust-best-practices]] - Language-specific recommendations
- [[mission-10]] - Example of feature-based organization in practice
- [[Cargo Workspaces]] - Multi-crate project organization

**Domain Modeling**:
- [[Domain-Driven Design]] - Business-aligned architecture
- [[Event-Driven Architecture]] - Decoupled feature communication
- [[Microservices Patterns]] - Service-based architecture

**Project Organization**:
- [[project-structure-patterns]] - Directory organization strategies
- [[code-ownership-models]] - Team and code responsibility patterns
- [[refactoring-techniques]] - Evolving architecture over time

**Quality Assurance**:
- [[test-pyramid]] - Testing strategies for different architectures
- [[code-quality-metrics]] - Measuring architectural success
- [[technical-debt-management]] - Managing architectural evolution

*Links: [[zettel-index]] | [[Clean Code Principles]] | [[rust-best-practices]] | [[V-Cycle Methodology]] | [[test-pyramid]] | [[TDD (Test-Driven Development)]] | [[mission-10]]*