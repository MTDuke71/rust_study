# Procedural Macros

*Navigation: [[zettel-index]] | [[macros-introduction]] | [[declarative-macros]] | [[rust_book/rust-book-ch20]]*

---

## Overview

**Procedural macros** (proc macros) are functions that take Rust code as input and produce Rust code as output. Unlike declarative macros (`macro_rules!`), they operate on the abstract syntax tree (AST) and can perform arbitrary computation.

**Key Insight**: Procedural macros are like compiler plugins - they run during compilation and can generate, modify, or analyze code using the full power of Rust.

---

## Three Types of Procedural Macros

| Type | Syntax | Purpose | Example |
|------|--------|---------|---------|
| **Derive Macros** | `#[derive(Trait)]` | Auto-implement traits | `#[derive(Debug, Clone)]` |
| **Attribute-like Macros** | `#[custom_attr]` | Custom attributes on items | `#[route(GET, "/")]` |
| **Function-like Macros** | `custom!()` | Like declarative but more powerful | `sql!(SELECT * FROM users)` |

---

## Architecture

### Separate Crate Requirement

**CRITICAL**: Procedural macros **must** be defined in a separate crate with `proc-macro = true`:

```toml
# my_macros/Cargo.toml
[package]
name = "my_macros"
version = "0.1.0"

[lib]
proc-macro = true

[dependencies]
syn = "2.0"      # Parse Rust code
quote = "1.0"    # Generate Rust code
proc-macro2 = "1.0"  # Wrapper around proc_macro
```

### Key Dependencies

**`syn`** - Parse Rust syntax into AST:
```rust
use syn::{parse_macro_input, DeriveInput, ItemStruct, Expr};
```

**`quote`** - Generate Rust code from templates:
```rust
use quote::quote;
let output = quote! {
    impl MyTrait for #name { }
};
```

**`proc_macro2`** - Wrapper types that work in and outside proc macros:
```rust
use proc_macro2::TokenStream;
```

---

## 1. Derive Macros

**Purpose**: Automatically implement traits for types.

### Basic Template

```rust
// my_macros/src/lib.rs
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(MyTrait)]
pub fn my_trait_derive(input: TokenStream) -> TokenStream {
    // Parse input tokens as a Rust item (struct/enum)
    let input = parse_macro_input!(input as DeriveInput);
    
    // Extract name and generics
    let name = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    
    // Generate implementation
    let expanded = quote! {
        impl #impl_generics MyTrait for #name #ty_generics #where_clause {
            fn my_method(&self) {
                println!("Called on {}", stringify!(#name));
            }
        }
    };
    
    TokenStream::from(expanded)
}
```

### Usage

```rust
// In user code (different crate)
use my_macros::MyTrait;

#[derive(MyTrait)]
struct MyStruct {
    field: i32,
}

// Now MyStruct automatically implements MyTrait
let s = MyStruct { field: 42 };
s.my_method();  // "Called on MyStruct"
```

### Handling Data Structures

**Match on `input.data`** to handle structs vs enums:

```rust
use syn::{Data, Fields};

match &input.data {
    Data::Struct(data_struct) => {
        match &data_struct.fields {
            Fields::Named(fields) => {
                // struct Foo { x: i32, y: i32 }
                let field_names = fields.named.iter().map(|f| &f.ident);
                // ...
            }
            Fields::Unnamed(fields) => {
                // struct Foo(i32, i32)
                let field_count = fields.unnamed.len();
                // ...
            }
            Fields::Unit => {
                // struct Foo;
                // ...
            }
        }
    }
    Data::Enum(data_enum) => {
        // enum Foo { A, B, C }
        let variants = &data_enum.variants;
        // ...
    }
    Data::Union(_) => {
        panic!("Unions not supported");
    }
}
```

### Real-World Example: Builder Pattern

```rust
#[proc_macro_derive(Builder)]
pub fn builder_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let builder_name = format!("{}Builder", name);
    let builder_ident = syn::Ident::new(&builder_name, name.span());
    
    // Extract fields from struct
    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => panic!("Only named fields supported"),
        },
        _ => panic!("Only structs supported"),
    };
    
    // Generate builder fields (all Option<T>)
    let builder_fields = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        quote! { #name: Option<#ty> }
    });
    
    // Generate setter methods
    let setters = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        quote! {
            pub fn #name(mut self, #name: #ty) -> Self {
                self.#name = Some(#name);
                self
            }
        }
    });
    
    // Generate build() method
    let build_fields = fields.iter().map(|f| {
        let name = &f.ident;
        quote! {
            #name: self.#name.ok_or(concat!("Field not set: ", stringify!(#name)))?
        }
    });
    
    let expanded = quote! {
        pub struct #builder_ident {
            #(#builder_fields),*
        }
        
        impl #builder_ident {
            #(#setters)*
            
            pub fn build(self) -> Result<#name, &'static str> {
                Ok(#name {
                    #(#build_fields),*
                })
            }
        }
        
        impl #name {
            pub fn builder() -> #builder_ident {
                #builder_ident {
                    #(#name: None),*
                }
            }
        }
    };
    
    TokenStream::from(expanded)
}
```

**Usage**:
```rust
#[derive(Builder)]
struct User {
    name: String,
    age: u32,
}

let user = User::builder()
    .name("Alice".to_string())
    .age(30)
    .build()
    .unwrap();
```

---

## 2. Attribute-like Macros

**Purpose**: Custom attributes that can be applied to items.

### Basic Template

```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn my_attribute(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse attribute arguments
    let attr_args = parse_macro_input!(attr as AttributeArgs);
    
    // Parse the item being annotated (e.g., a function)
    let input_fn = parse_macro_input!(item as ItemFn);
    
    // Generate modified code
    let fn_name = &input_fn.sig.ident;
    let fn_body = &input_fn.block;
    
    let expanded = quote! {
        fn #fn_name() {
            println!("Entering {}", stringify!(#fn_name));
            #fn_body
            println!("Exiting {}", stringify!(#fn_name));
        }
    };
    
    TokenStream::from(expanded)
}
```

### Usage

```rust
#[my_attribute]
fn my_function() {
    println!("Function body");
}

// Expands to:
// fn my_function() {
//     println!("Entering my_function");
//     println!("Function body");
//     println!("Exiting my_function");
// }
```

### Real-World Example: Route Attribute (Web Framework)

```rust
#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse attribute: #[route(GET, "/users")]
    let route_config = parse_macro_input!(attr as RouteConfig);
    let input_fn = parse_macro_input!(item as ItemFn);
    
    let method = &route_config.method;
    let path = &route_config.path;
    let fn_name = &input_fn.sig.ident;
    let fn_vis = &input_fn.vis;
    
    let expanded = quote! {
        // Keep original function
        #input_fn
        
        // Generate route registration
        inventory::submit! {
            Route {
                method: Method::#method,
                path: #path,
                handler: #fn_name,
            }
        }
    };
    
    TokenStream::from(expanded)
}
```

**Usage**:
```rust
#[route(GET, "/users/:id")]
async fn get_user(id: u32) -> Json<User> {
    // Handler implementation
}
```

---

## 3. Function-like Macros

**Purpose**: Custom macro invocations that look like function calls but can have custom syntax.

### Basic Template

```rust
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn my_macro(input: TokenStream) -> TokenStream {
    // Parse custom syntax
    let input_str = input.to_string();
    
    // Generate code based on input
    let expanded = quote! {
        // Generated code here
    };
    
    TokenStream::from(expanded)
}
```

### Usage

```rust
my_macro!(custom syntax here);
```

### Real-World Example: SQL Query Macro

```rust
#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
    // Parse SQL string
    let sql_query = input.to_string();
    
    // Validate SQL at compile time (could use SQL parser)
    validate_sql(&sql_query).expect("Invalid SQL");
    
    // Generate code
    let expanded = quote! {
        {
            let query = #sql_query;
            // Execute query with type-safe parameters
            db.query(query)
        }
    };
    
    TokenStream::from(expanded)
}
```

**Usage**:
```rust
let users = sql!(SELECT * FROM users WHERE age > 18);
```

### Advanced: Custom DSL

```rust
#[proc_macro]
pub fn html(input: TokenStream) -> TokenStream {
    // Parse HTML-like syntax: html! { <div class="foo">Hello</div> }
    let html_tree = parse_html(input);
    
    // Generate HTML string construction
    let expanded = generate_html_code(&html_tree);
    
    TokenStream::from(expanded)
}
```

---

## Working with `syn` and `quote`

### Parsing with `syn`

**Common parsing functions**:

```rust
use syn::{
    parse_macro_input,
    DeriveInput,    // Struct/enum definition
    ItemFn,         // Function definition
    ItemStruct,     // Struct only
    ItemEnum,       // Enum only
    Expr,           // Expression
    Type,           // Type
    Pat,            // Pattern
};

// Parse as specific type
let input = parse_macro_input!(input as DeriveInput);

// Parse with custom parser
let input = parse_macro_input!(input with custom_parser);
```

**Extracting information**:

```rust
let input: DeriveInput = parse_macro_input!(input);

// Basic info
let name = &input.ident;           // Type name
let generics = &input.generics;    // <T, U>
let attrs = &input.attrs;          // #[attributes]

// Split generics for impl
let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

// Data inspection
match &input.data {
    Data::Struct(s) => { /* ... */ }
    Data::Enum(e) => { /* ... */ }
    Data::Union(u) => { /* ... */ }
}
```

### Generating with `quote`

**Basic usage**:

```rust
use quote::quote;

let tokens = quote! {
    fn hello() {
        println!("Hello, world!");
    }
};
```

**Interpolation with `#`**:

```rust
let name = "Alice";
let age = 30;

let tokens = quote! {
    struct Person {
        name: &'static str,
        age: u32,
    }
    
    let person = Person {
        name: #name,
        age: #age,
    };
};
```

**Repetition with `#(...)*`**:

```rust
let fields = vec!["field1", "field2", "field3"];

let tokens = quote! {
    struct MyStruct {
        #(
            #fields: i32,
        )*
    }
};
// Expands to:
// struct MyStruct {
//     field1: i32,
//     field2: i32,
//     field3: i32,
// }
```

**Nested repetition**:

```rust
let variants = vec![
    ("Red", vec!["r", "red"]),
    ("Green", vec!["g", "green"]),
    ("Blue", vec!["b", "blue"]),
];

let tokens = quote! {
    impl Color {
        fn parse(s: &str) -> Option<Self> {
            match s {
                #(
                    #( #aliases )|* => Some(Color::#names),
                )*
                _ => None,
            }
        }
    }
};
```

---

## Error Handling

### Compile Errors with `syn::Error`

```rust
use syn::Error;

let error = Error::new(
    name.span(),  // Where the error occurred
    "Invalid name: must start with uppercase letter"
);

return error.to_compile_error().into();
```

### Multiple Errors

```rust
let mut errors = Vec::new();

for field in fields {
    if !is_valid(&field) {
        errors.push(Error::new(field.span(), "Invalid field"));
    }
}

if !errors.is_empty() {
    let combined = errors.into_iter()
        .map(|e| e.to_compile_error())
        .collect::<proc_macro2::TokenStream>();
    return combined.into();
}
```

### Panicking (Simple but Less User-Friendly)

```rust
if fields.is_empty() {
    panic!("Struct must have at least one field");
}
```

---

## Advanced Techniques

### Helper Attributes

**Derive macros can define helper attributes**:

```rust
#[proc_macro_derive(MyTrait, attributes(my_helper))]
pub fn my_trait_derive(input: TokenStream) -> TokenStream {
    // Now users can write:
    // #[derive(MyTrait)]
    // struct Foo {
    //     #[my_helper(skip)]
    //     field: i32,
    // }
    
    // Parse helper attributes
    for attr in &field.attrs {
        if attr.path().is_ident("my_helper") {
            // Process helper attribute
        }
    }
    // ...
}
```

### Type Analysis

```rust
use syn::Type;

fn analyze_type(ty: &Type) -> String {
    match ty {
        Type::Path(type_path) => {
            // Regular type like String, Vec<T>, etc.
            type_path.path.segments.last().unwrap().ident.to_string()
        }
        Type::Reference(type_ref) => {
            // &T or &mut T
            format!("Reference to {}", analyze_type(&type_ref.elem))
        }
        Type::Tuple(type_tuple) => {
            // (T1, T2, ...)
            "Tuple".to_string()
        }
        _ => "Unknown".to_string(),
    }
}
```

### Span Manipulation

**Spans track source location** for better error messages:

```rust
use proc_macro2::Span;

// Create identifier with span
let ident = syn::Ident::new("my_name", Span::call_site());

// Use original span for errors
let error = Error::new(original_ident.span(), "Error message");
```

---

## Common Patterns

### Pattern 1: Newtype Wrapper

```rust
#[proc_macro_derive(NewType)]
pub fn newtype_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    
    // Extract inner type (assumes single unnamed field)
    let inner_type = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                &fields.unnamed[0].ty
            }
            _ => panic!("NewType requires single unnamed field"),
        },
        _ => panic!("NewType only works on structs"),
    };
    
    let expanded = quote! {
        impl #name {
            pub fn new(value: #inner_type) -> Self {
                Self(value)
            }
            
            pub fn into_inner(self) -> #inner_type {
                self.0
            }
            
            pub fn as_inner(&self) -> &#inner_type {
                &self.0
            }
        }
        
        impl From<#inner_type> for #name {
            fn from(value: #inner_type) -> Self {
                Self(value)
            }
        }
        
        impl AsRef<#inner_type> for #name {
            fn as_ref(&self) -> &#inner_type {
                &self.0
            }
        }
    };
    
    TokenStream::from(expanded)
}
```

### Pattern 2: Enum Utilities

```rust
#[proc_macro_derive(EnumUtils)]
pub fn enum_utils_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    
    let variants = match &input.data {
        Data::Enum(data) => &data.variants,
        _ => panic!("EnumUtils only works on enums"),
    };
    
    let variant_names = variants.iter().map(|v| &v.ident);
    let variant_strings = variants.iter().map(|v| v.ident.to_string());
    
    let expanded = quote! {
        impl #name {
            pub fn all() -> Vec<Self> {
                vec![#(Self::#variant_names),*]
            }
            
            pub fn name(&self) -> &'static str {
                match self {
                    #(Self::#variant_names => #variant_strings,)*
                }
            }
        }
    };
    
    TokenStream::from(expanded)
}
```

---

## Testing Procedural Macros

### Unit Testing with `trybuild`

```rust
// tests/test_derive.rs
#[test]
fn test_successful_derive() {
    let t = trybuild::TestCases::new();
    t.pass("tests/ui/pass/*.rs");
}

#[test]
fn test_compile_failures() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/fail/*.rs");
}
```

### Integration Testing

```rust
// tests/integration.rs
use my_macros::MyTrait;

#[derive(MyTrait)]
struct TestStruct {
    field: i32,
}

#[test]
fn test_derived_functionality() {
    let s = TestStruct { field: 42 };
    s.my_method();
}
```

### Debug Output

```rust
// In your proc macro
eprintln!("Generated code: {}", expanded.to_string());
```

---

## Performance Considerations

- **Compile time**: Proc macros increase compile time (they run during compilation)
- **Complexity**: Complex macros can significantly slow builds
- **Caching**: Cargo caches proc macro results when inputs don't change
- **Best practices**:
  - Keep generated code minimal
  - Cache expensive computations
  - Use conditional compilation for debug/release builds

---

## Common Pitfalls

### 1. Hygiene Issues

**Problem**: Generated code might not have expected visibility:

```rust
// ❌ Generated code might not see private items
quote! {
    fn helper() { ... }  // Might conflict or be inaccessible
}
```

**Solution**: Use fully qualified paths:

```rust
quote! {
    fn helper() {
        ::std::println!("Using fully qualified path");
    }
}
```

### 2. Span Information Loss

**Problem**: Using wrong spans makes errors confusing:

```rust
// ❌ Bad: creates new span
let ident = syn::Ident::new("name", Span::call_site());
```

**Solution**: Preserve original spans:

```rust
// ✅ Good: uses original span
let ident = syn::Ident::new("name", original.span());
```

### 3. Type Parameter Handling

**Problem**: Forgetting to handle generics:

```rust
// ❌ Bad: ignores generic parameters
impl MyTrait for #name { ... }
```

**Solution**: Use `split_for_impl()`:

```rust
// ✅ Good: handles generics correctly
let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
impl #impl_generics MyTrait for #name #ty_generics #where_clause { ... }
```

---

## When to Use Procedural Macros

**✅ Use When**:
- Auto-implementing traits with custom logic
- Need to inspect type structure (fields, variants)
- Creating custom DSLs with validation
- Complex code generation based on attributes
- Want better error messages with spans

**❌ Avoid When**:
- Simple pattern matching suffices (use `macro_rules!`)
- Runtime code generation (use regular functions)
- Can use generic functions or traits instead
- Compile time is critical (proc macros are slower)

---

## Comparison Table

| Feature | Declarative | Derive | Attribute-like | Function-like |
|---------|-------------|--------|----------------|---------------|
| **Syntax** | `macro_rules!` | `#[derive()]` | `#[attr]` | `macro!()` |
| **Complexity** | Simple | Medium | Medium | High |
| **AST Access** | No | Yes | Yes | Yes |
| **Custom Syntax** | Limited | No | No | Yes |
| **Separate Crate** | No | Yes | Yes | Yes |
| **Dependencies** | None | syn, quote | syn, quote | syn, quote |
| **Compile Time** | Fast | Slower | Slower | Slower |

---

## Integration with Rust Book Ch20.5

From the Rust Book:

**Key Concepts**:
1. Procedural macros are functions that manipulate Rust code
2. Must be in separate crate with `proc-macro = true`
3. Three types: derive, attribute-like, function-like
4. Use `syn` to parse, `quote` to generate

**Example from Book**:
```rust
use proc_macro::TokenStream;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Implementation would parse input and generate impl
    TokenStream::new()
}
```

---

## Real-World Crates Using Proc Macros

| Crate | Macro Type | Purpose |
|-------|------------|---------|
| **serde** | Derive | `#[derive(Serialize, Deserialize)]` |
| **tokio** | Attribute | `#[tokio::main]` for async runtime |
| **actix-web** | Attribute | `#[get("/path")]` route handlers |
| **thiserror** | Derive | `#[derive(Error)]` for error types |
| **clap** | Derive | `#[derive(Parser)]` for CLI parsing |
| **diesel** | Macro | `table!` for database schema DSL |

---

## Related Concepts

### Macro Fundamentals
- [[macros-introduction]] - Overview of all macro types
- [[declarative-macros]] - Pattern-based macros with `macro_rules!`

### Code Generation
- [[rust-metaprogramming]] - Compile-time programming techniques
- [[build-scripts]] - Alternative code generation at build time
- [[const-evaluation]] - Compile-time computation without macros

### Advanced Rust
- [[Rust Traits]] - Often better than macros for polymorphism
- [[rust_book/rust-book-ch20]] - Advanced features including macros

---

## Further Reading

- **Rust Book Ch20.5**: Introduction to procedural macros
- **`syn` documentation**: Parsing Rust syntax
- **`quote` documentation**: Generating Rust code
- **The Rust Reference**: Procedural Macros chapter
- **dtolnay's proc-macro-workshop**: Hands-on exercises

---

*Tags: #macros #procedural-macros #proc-macro #metaprogramming #derive-macros #code-generation #syn #quote #rust-advanced*

*Links: [[zettel-index]] | [[macros-introduction]] | [[declarative-macros]] | [[rust_book/rust-book-ch20]] | [[../../rust_book/Ch20/examples/ch20_5_macros]]*
