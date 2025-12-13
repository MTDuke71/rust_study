//! Chapter 20.5: Macros
//!
//! Demonstrates:
//! - Declarative macros (macro_rules!)
//! - Procedural macros (concept)

fn main() {
    println!("=== Chapter 20.5: Macros ===\n");

    demonstrate_declarative_macros();
    demonstrate_macro_concepts();

    println!("\n✅ All macro concepts demonstrated!");
}

/// Custom vec-like macro
#[macro_export]
macro_rules! my_vec {
    ( $( $x:expr ),* ) => {
        {
            vec![$($x),*]
        }
    };
}

/// HashMap macro
#[macro_export]
macro_rules! hashmap {
    ( $( $key:expr => $value:expr ),* $(,)? ) => {
        {
            let mut map = ::std::collections::HashMap::new();
            $(
                map.insert($key, $value);
            )*
            map
        }
    };
}

/// 1. Declarative Macros
fn demonstrate_declarative_macros() {
    println!("--- 1. Declarative Macros (macro_rules!) ---");

    let v = my_vec![1, 2, 3];
    println!("  my_vec![1, 2, 3] = {:?}", v);

    let map = hashmap! {
        "key1" => "value1",
        "key2" => "value2",
    };
    println!("  hashmap! = {:?}", map);

    println!("💡 Declarative macros: pattern matching on code");
    println!();
}

/// 2. Macro Concepts
fn demonstrate_macro_concepts() {
    println!("--- 2. Procedural Macro Concepts ---");

    println!("  Procedural macros require separate crates:");
    println!("    - Custom derive: #[derive(MyTrait)]");
    println!("    - Attribute-like: #[route(GET, \"/\")]");
    println!("    - Function-like: sql!(SELECT * FROM users)");

    println!("\n  Key crates:");
    println!("    - syn: Parse Rust code");
    println!("    - quote: Generate Rust code");
    println!("    - proc-macro: Compiler API");

    println!("\n💡 Procedural macros: code that generates code");
    println!();
}
