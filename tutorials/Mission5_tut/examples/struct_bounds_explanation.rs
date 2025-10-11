use std::collections::HashMap;
use std::hash::Hash;

/// Explaining the Dictionary struct definition and trait bounds
fn main() {
    println!("🔍 Breaking Down the Dictionary Struct Definition");
    println!("=================================================\n");

    // Let's analyze each part of your highlighted code:

    println!("📝 Line-by-Line Analysis:");
    println!("=========================");

    println!("pub struct Dictionary<K, V>");
    println!("  ├─ `pub` = Public visibility (can be used outside this module)");
    println!("  ├─ `struct` = Defines a new data type");
    println!("  ├─ `Dictionary` = The name of our new type");
    println!("  └─ `<K, V>` = Generic type parameters (K=Key type, V=Value type)");

    println!("\nwhere");
    println!("  └─ `where` = Introduces trait bounds/constraints");

    println!("\n    K: Eq + Hash,");
    println!("      ├─ `K:` = Constraint on the K type parameter");
    println!("      ├─ `Eq` = K must implement equality comparison");
    println!("      ├─ `+` = AND operator (K must have BOTH traits)");
    println!("      ├─ `Hash` = K must be hashable");
    println!("      └─ `,` = End of constraint");

    println!("\n{{");
    println!("    inner: HashMap<K, V>,");
    println!("      ├─ `inner:` = Field name");
    println!("      ├─ `HashMap<K, V>` = Field type (using our generic parameters)");
    println!("      └─ `,` = End of field");
    println!("}}");

    println!("\n🎯 What the Trait Bounds Mean:");
    println!("==============================");

    println!("K: Eq + Hash means the key type must:");
    println!("  ✅ `Eq` - Support equality comparison (==, !=)");
    println!("      • Required because HashMap needs to check if keys are equal");
    println!("      • Example: 'apple' == 'apple' must work");

    println!("  ✅ `Hash` - Be convertible to a hash value");
    println!("      • Required because HashMap uses hashing for fast lookups");
    println!("      • Example: 'apple' → hash value → storage location");

    println!("\n🔧 impl Block Analysis:");
    println!("=======================");

    println!("impl<K, V> Dictionary<K, V>");
    println!("  ├─ `impl` = Implementation block (defines methods)");
    println!("  ├─ `<K, V>` = Same generic parameters as the struct");
    println!("  └─ `Dictionary<K, V>` = The type we're implementing for");

    println!("\nwhere");
    println!("    K: Eq + Hash,");
    println!("  └─ Same constraints repeated (required for consistency)");

    println!("\n💡 Why These Constraints Exist:");
    println!("===============================");

    println!("HashMap requires keys to be:");
    println!("  1️⃣ `Hash` - To determine where to store the key-value pair");
    println!("  2️⃣ `Eq` - To check if two keys are the same when retrieving");

    println!("\n🧪 What Types Work as Keys:");
    println!("===========================");

    println!("✅ VALID key types (implement Eq + Hash):");
    println!("  • i32, u32, i64 (all integer types)");
    println!("  • String, &str");
    println!("  • bool");
    println!("  • char");
    println!("  • (i32, i32) tuples of hashable types");
    println!("  • Most primitive types");

    println!("\n❌ INVALID key types (don't implement Hash):");
    println!("  • f32, f64 (floating point - can't hash reliably)");
    println!("  • Vec<T> (vectors are not hashable by default)");
    println!("  • HashMap<K, V> (hashmaps themselves)");

    println!("\n🎮 Real Examples:");
    println!("================");

    // These would all work:
    let _dict1: Dictionary<String, i32> = Dictionary::new(); // String keys
    let _dict2: Dictionary<i32, String> = Dictionary::new(); // Integer keys
    let _dict3: Dictionary<(i32, i32), char> = Dictionary::new(); // Tuple keys

    println!("✅ Dictionary<String, i32>     // String keys, integer values");
    println!("✅ Dictionary<i32, String>     // Integer keys, string values");
    println!("✅ Dictionary<(i32, i32), char> // Coordinate keys, character values");

    // These would NOT compile:
    // let _dict4: Dictionary<f64, i32> = Dictionary::new();  // ❌ f64 doesn't implement Hash
    // let _dict5: Dictionary<Vec<i32>, String> = Dictionary::new(); // ❌ Vec doesn't implement Hash

    println!("❌ Dictionary<f64, i32>        // f64 can't be hashed");
    println!("❌ Dictionary<Vec<i32>, String> // Vec can't be hashed");

    println!("\n🔗 The Connection to HashMap:");
    println!("=============================");

    println!("Since Dictionary contains `inner: HashMap<K, V>`,");
    println!("Dictionary must enforce the SAME constraints as HashMap!");

    println!("\nHashMap<K, V> requires:");
    println!("  K: Eq + Hash  ← Dictionary inherits this requirement");
    println!("  V: (no constraints) ← Values can be any type");

    println!("\n🚀 Summary:");
    println!("===========");
    println!("Your highlighted code is setting up the 'contract' that:");
    println!("  1. Dictionary is generic over two types: K (key) and V (value)");
    println!("  2. K must be equalizable and hashable (required by HashMap)");
    println!("  3. V can be any type (no constraints on values)");
    println!("  4. This ensures Dictionary can safely wrap HashMap!");
}

// Dummy struct to make the example compile
struct Dictionary<K, V>
where
    K: Eq + Hash,
{
    inner: HashMap<K, V>,
}

impl<K, V> Dictionary<K, V>
where
    K: Eq + Hash,
{
    fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }
}
