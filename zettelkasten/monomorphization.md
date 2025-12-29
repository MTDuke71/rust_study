# Monomorphization - Compile-Time Code Generation

*The process where the compiler generates specialized code for each concrete type used with generic functions or types.*

---

## 🎯 **Core Concept**

Monomorphization is the process of turning generic code into specific code by filling in the concrete types at compile time. In Rust, this means for every unique type a generic function is called with, a separate copy of the machine code is generated. This results in zero-cost abstractions at runtime (static dispatch) but increases binary size.

It is the mechanism that powers Rust's **zero-cost abstractions**, allowing high-level generic code to compile down to the same assembly as manually written specific code.

## 🧠 **Mental Models**

### The Cookie Cutter

The generic function is a cookie cutter. Monomorphization is the act of pressing it into dough (types) to create specific cookies (functions). You don't eat the cutter; you eat the cookies.

### Find and Replace

Imagine the compiler doing a "Find and Replace" for the generic type parameter `T` with `i32`, then copying the function. Then doing it again for `f64`.

## 🔍 **Detailed Content**

### **Static Dispatch**

Because the specific function is known at compile time, the compiler can inline calls, leading to highly optimized code. This is in contrast to dynamic dispatch, where the function to call is determined at runtime.

### **Code Bloat**

Excessive use of generics with many different types can lead to large binary sizes, a trade-off known as "code bloat". Each unique instantiation of a generic function adds to the executable size.

### **Compilation Time**

Monomorphization happens during the compilation phase. Heavy use of generics can increase compile times because the compiler has to generate and optimize code for every concrete type used.

## 💡 **Key Takeaways**

- Generates specialized code for each concrete type.
- Enables zero-cost abstractions and aggressive inlining.
- Can increase compile times and binary size.
- Default behavior for Rust generics.
- Happens at compile time, resulting in static dispatch.

## 🔗 **Integration Points**

### **Builds On**

- [[Generic Programming]] - The syntax and usage of generics
- [[compilation-process]] - Where monomorphization fits in the pipeline

### **Enables**

- [[zero-cost-abstractions]] - High-level code with low-level performance
- [[static-dispatch]] - Direct function calls without vtables

### **Related Concepts**

- [[Trait Objects]] - The alternative: dynamic dispatch
- [[performance-patterns]] - Trade-offs between static and dynamic dispatch
- [[static-vs-dynamic-dispatch]] - Comprehensive comparison of dispatch mechanisms

---

*Tags: #concept #performance #rust-book #intermediate #compiler #generics*

*Links: [[zettel-index]] | [[rust-concepts-MOC]] | [[Trait Objects]] | [[zero-cost-abstractions]] | [[static-vs-dynamic-dispatch]]*
