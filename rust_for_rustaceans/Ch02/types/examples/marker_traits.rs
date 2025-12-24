// Example 6: Marker Traits and Type-State Pattern
// Demonstrates marker traits (Send, Sync, Copy) and type-state pattern

use rustaceans_ch02_types::utils::{print_section, print_subsection};
use std::marker::PhantomData;

fn main() {
    print_section("6. MARKER TRAITS AND TYPE-STATE PATTERN");
    
    marker_traits_overview();
    type_state_pattern();
    type_state_builder();
    phantom_data_usage();
}

// --- Marker Traits Overview ---
fn marker_traits_overview() {
    print_subsection("a. Marker Traits - Send, Sync, Copy");
    
    println!("Marker traits have NO methods - they declare properties:\n");
    
    // Send: Can transfer across threads
    fn send_example<T: Send + 'static>(value: T) {
        std::thread::spawn(move || {
            drop(value);  // Can move to another thread
        }).join().unwrap();
    }
    
    // Sync: Can share references across threads
    fn sync_example<T: Sync>(value: &T) {
        std::thread::scope(|s| {
            s.spawn(|| {
                let _ref = value;  // Can share &T across threads
            });
        });
    }
    
    let number = 42;
    send_example(number);  // i32 is Send
    sync_example(&number); // i32 is Sync
    
    println!("✅ Send: Type can be transferred across thread boundaries");
    println!("   Most types are Send (except Rc, raw pointers)");
    
    println!("\n✅ Sync: &T can be shared across threads");
    println!("   T is Sync if &T is Send");
    println!("   Most types are Sync (except Cell, RefCell)");
    
    println!("\n✅ Copy: Bitwise copy creates valid instance");
    println!("   Assignment copies instead of moves");
    println!("   Stack-only types (i32, bool, etc.)");
    
    // Copy example
    let x = 42;
    let y = x;  // Copy, not move
    println!("\nCopy trait: x = {}, y = {} (both valid)", x, y);
}

// --- Type-State Pattern ---
fn type_state_pattern() {
    print_subsection("b. Type-State Pattern - Compile-Time State Machine");
    
    println!("Use zero-sized marker types to enforce state transitions:\n");
    
    // State markers (zero-sized types)
    struct Unauthenticated;
    struct Authenticated;
    
    // Connection parameterized by state
    struct SshConnection<State> {
        address: String,
        _state: PhantomData<State>,
    }
    
    // Methods available only in Unauthenticated state
    impl SshConnection<Unauthenticated> {
        fn new(address: &str) -> Self {
            println!("Creating connection to {}", address);
            SshConnection {
                address: address.to_string(),
                _state: PhantomData,
            }
        }
        
        fn authenticate(self, password: &str) -> SshConnection<Authenticated> {
            println!("Authenticating with password: {}", "*".repeat(password.len()));
            SshConnection {
                address: self.address,
                _state: PhantomData,
            }
        }
    }
    
    // Methods available only in Authenticated state
    impl SshConnection<Authenticated> {
        fn execute(&self, command: &str) {
            println!("Executing on {}: {}", self.address, command);
        }
        
        fn disconnect(self) {
            println!("Disconnecting from {}", self.address);
        }
    }
    
    // Usage
    let conn = SshConnection::new("server.example.com");
    // conn.execute("ls");  // ❌ Compile error! Must authenticate first
    
    let conn = conn.authenticate("secret123");
    conn.execute("ls -la");       // ✅ OK - authenticated
    conn.execute("whoami");       // ✅ OK
    conn.disconnect();
    
    // conn.execute("ls");  // ❌ Compile error! Connection consumed by disconnect
    
    println!("\n✅ Type system enforces:");
    println!("   - Cannot execute before authentication");
    println!("   - Cannot use connection after disconnect");
    println!("   - State transitions are one-way");
}

// --- Type-State Builder Pattern ---
fn type_state_builder() {
    print_subsection("c. Type-State Builder Pattern");
    
    println!("Enforce required fields at compile time:\n");
    
    // State markers
    struct NoName;
    struct HasName;
    struct NoEmail;
    struct HasEmail;
    
    // Builder parameterized by state
    struct UserBuilder<NameState, EmailState> {
        name: Option<String>,
        email: Option<String>,
        age: Option<u32>,
        _name_state: PhantomData<NameState>,
        _email_state: PhantomData<EmailState>,
    }
    
    #[allow(dead_code)]
    struct User {
        name: String,
        email: String,
        age: Option<u32>,
    }
    
    impl UserBuilder<NoName, NoEmail> {
        fn new() -> Self {
            UserBuilder {
                name: None,
                email: None,
                age: None,
                _name_state: PhantomData,
                _email_state: PhantomData,
            }
        }
    }
    
    impl<EmailState> UserBuilder<NoName, EmailState> {
        fn name(self, name: &str) -> UserBuilder<HasName, EmailState> {
            UserBuilder {
                name: Some(name.to_string()),
                email: self.email,
                age: self.age,
                _name_state: PhantomData,
                _email_state: PhantomData,
            }
        }
    }
    
    impl<NameState> UserBuilder<NameState, NoEmail> {
        fn email(self, email: &str) -> UserBuilder<NameState, HasEmail> {
            UserBuilder {
                name: self.name,
                email: Some(email.to_string()),
                age: self.age,
                _name_state: PhantomData,
                _email_state: PhantomData,
            }
        }
    }
    
    impl<NameState, EmailState> UserBuilder<NameState, EmailState> {
        fn age(mut self, age: u32) -> Self {
            self.age = Some(age);
            self
        }
    }
    
    impl UserBuilder<HasName, HasEmail> {
        fn build(self) -> User {
            User {
                name: self.name.unwrap(),
                email: self.email.unwrap(),
                age: self.age,
            }
        }
    }
    
    // Usage
    let user = UserBuilder::new()
        .name("Alice")
        .email("alice@example.com")
        .age(30)
        .build();
    
    println!("Built user: {} <{}>", user.name, user.email);
    
    // These won't compile:
    // UserBuilder::new().build();                    // ❌ Missing name and email
    // UserBuilder::new().name("Bob").build();        // ❌ Missing email
    // UserBuilder::new().email("bob@test.com").build(); // ❌ Missing name
    
    println!("\n✅ Compiler enforces required fields");
    println!("   Cannot call build() without name and email");
}

// --- PhantomData Usage ---
fn phantom_data_usage() {
    print_subsection("d. PhantomData - Marker for Unused Type Parameters");
    
    println!("PhantomData<T> indicates type parameter T is used conceptually:\n");
    
    struct Container<T> {
        #[allow(dead_code)]
        data: Vec<u8>,
        _marker: PhantomData<T>,  // "Acts like" it owns T
    }
    
    impl<T> Container<T> {
        fn new() -> Self {
            Container {
                data: Vec::new(),
                _marker: PhantomData,
            }
        }
    }
    
    let _int_container: Container<i32> = Container::new();
    let _str_container: Container<String> = Container::new();
    
    println!("Container<i32>: size = {} bytes", std::mem::size_of::<Container<i32>>());
    println!("Container<String>: size = {} bytes", std::mem::size_of::<Container<String>>());
    println!("PhantomData: size = {} bytes (zero!)", std::mem::size_of::<PhantomData<String>>());
    
    println!("\n✅ PhantomData<T> is zero-sized");
    println!("✅ Tells compiler to treat struct as if it owns T");
    println!("✅ Affects variance and drop checking");
    
    println!("\n📊 When to use marker types:\n");
    println!("┌────────────────────────────────────────────────────┐");
    println!("│ Type-State Pattern                                 │");
    println!("├────────────────────────────────────────────────────┤");
    println!("│ • Enforce valid state transitions                  │");
    println!("│ • Make invalid states unrepresentable             │");
    println!("│ • Zero runtime cost                                │");
    println!("│ Examples: Authenticated/Unauthenticated, Open/Close│");
    println!("└────────────────────────────────────────────────────┘");
}
