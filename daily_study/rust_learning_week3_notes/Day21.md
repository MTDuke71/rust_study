# Day 21 · Generics + Traits Practice (integrated mastery project)

> **Learning Context**: Day 21 integrates all Week 3 concepts (traits, generics, lifetimes, trait objects) through Mission5's complete API design, demonstrating real-world application of advanced type system features.

**Cross-Track Integration:**
- **Mission Focus**: Culminates Mission5's advanced features using all Week 3 concepts in a complete, flexible HashMap API - see [[mission-5]]
- **Daily Study**: Week 3 mastery project combining traits → generics → lifetimes → trait objects
- **Rust Book**: Chapters 10 (Generics, Traits, Lifetimes) and 17 (Object-Oriented Features) integration

**Related Zettelkasten Notes:**
- [[Collections MOC]] - Complete type system patterns across collection designs
- [[mission-5]] - REQ-8 advanced API design using all Week 3 concepts
- [[HashMap Internals]] - Implementation patterns for flexible, performant APIs
- [[zettel-index]] - Main learning hub

## Integration Project Overview

### Mission5 Complete: Advanced HashMap with Full Type System
This day creates a comprehensive HashMap implementation showcasing:

1. **Generic Parameters**: Multiple type parameters with bounds
2. **Trait Definitions**: Custom traits for extensibility
3. **Trait Implementations**: Both manual and derived implementations
4. **Associated Types**: Advanced trait patterns
5. **Lifetimes**: Zero-copy operations and complex relationships
6. **Trait Objects**: Dynamic dispatch for plugins
7. **Where Clauses**: Complex constraint expressions
8. **Default Implementations**: Flexible API design

## Core Integration Patterns

### 1. Generic HashMap with Trait Bounds
```rust
use std::hash::{Hash, Hasher, BuildHasher, RandomState};
use std::collections::HashMap;
use std::marker::PhantomData;

// Mission5 Complete: Advanced generic HashMap
pub struct AdvancedHashMap<K, V, S = RandomState>
where
    K: Hash + Eq,
    S: BuildHasher,
{
    inner: HashMap<K, V, S>,
    metadata: MapMetadata<K, V>,
    plugins: Vec<Box<dyn MapPlugin<K, V>>>,
}

// Metadata tracking with generic parameters
#[derive(Debug, Clone)]
pub struct MapMetadata<K, V> {
    access_count: u64,
    modification_count: u64,
    _phantom: PhantomData<(K, V)>,
}

impl<K, V> MapMetadata<K, V> {
    pub fn new() -> Self {
        MapMetadata {
            access_count: 0,
            modification_count: 0,
            _phantom: PhantomData,
        }
    }
    
    pub fn record_access(&mut self) {
        self.access_count += 1;
    }
    
    pub fn record_modification(&mut self) {
        self.modification_count += 1;
    }
    
    pub fn stats(&self) -> (u64, u64) {
        (self.access_count, self.modification_count)
    }
}
```

### 2. Advanced Trait System
```rust
// Core traits for HashMap extensibility
pub trait Keyable {
    type Key: Hash + Eq + Clone;
    
    fn key(&self) -> Self::Key;
    fn matches_key(&self, key: &Self::Key) -> bool {
        &self.key() == key
    }
}

pub trait Valuable {
    type Value: Clone;
    
    fn value(&self) -> Self::Value;
}

// Combined trait with associated types
pub trait Entry: Keyable + Valuable {
    fn into_pair(self) -> (Self::Key, Self::Value);
    
    // Default implementation using associated functions
    fn from_pair(key: Self::Key, value: Self::Value) -> Self
    where
        Self: Sized;
}

// Plugin trait with generic parameters
pub trait MapPlugin<K, V>: Send + Sync {
    fn name(&self) -> &str;
    fn on_insert(&self, key: &K, value: &V) -> Result<(), PluginError>;
    fn on_get(&self, key: &K, value: Option<&V>) -> Result<(), PluginError>;
    fn on_remove(&self, key: &K, value: Option<V>) -> Result<(), PluginError>;
    
    // Default implementations
    fn priority(&self) -> u8 { 0 }
    fn is_enabled(&self) -> bool { true }
}

#[derive(Debug, Clone)]
pub struct PluginError {
    pub message: String,
    pub plugin_name: String,
}

impl std::fmt::Display for PluginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Plugin '{}': {}", self.plugin_name, self.message)
    }
}

impl std::error::Error for PluginError {}
```

### 3. Complex Implementations with Where Clauses
```rust
impl<K, V, S> AdvancedHashMap<K, V, S>
where
    K: Hash + Eq + Clone,
    V: Clone,
    S: BuildHasher,
{
    pub fn new() -> Self
    where
        S: Default,
    {
        AdvancedHashMap {
            inner: HashMap::default(),
            metadata: MapMetadata::new(),
            plugins: Vec::new(),
        }
    }
    
    pub fn with_hasher(hasher: S) -> Self {
        AdvancedHashMap {
            inner: HashMap::with_hasher(hasher),
            metadata: MapMetadata::new(),
            plugins: Vec::new(),
        }
    }
    
    // Generic insertion with plugin notifications
    pub fn insert(&mut self, key: K, value: V) -> Result<Option<V>, PluginError>
    where
        K: std::fmt::Debug,
        V: std::fmt::Debug,
    {
        // Notify plugins before insertion
        for plugin in &self.plugins {
            if plugin.is_enabled() {
                plugin.on_insert(&key, &value)?;
            }
        }
        
        self.metadata.record_modification();
        let old_value = self.inner.insert(key, value);
        
        Ok(old_value)
    }
    
    // Zero-copy lookup with lifetime management
    pub fn get<'a, Q>(&'a self, key: &Q) -> Option<&'a V>
    where
        K: std::borrow::Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.metadata.record_access();
        let result = self.inner.get(key);
        
        // Notify plugins (ignoring errors for read operations)
        for plugin in &self.plugins {
            if plugin.is_enabled() {
                let _ = plugin.on_get(unsafe { 
                    std::mem::transmute(key) 
                }, result.map(|v| unsafe { 
                    std::mem::transmute(v) 
                }));
            }
        }
        
        result
    }
    
    // Advanced entry API with generic bounds
    pub fn entry_with_default<F>(&mut self, key: K, default_fn: F) -> &mut V
    where
        F: FnOnce() -> V,
    {
        self.metadata.record_access();
        self.inner.entry(key).or_insert_with(default_fn)
    }
}

// Specialized implementations for common types
impl<S> AdvancedHashMap<String, String, S>
where
    S: BuildHasher,
{
    // String-specific operations
    pub fn insert_str(&mut self, key: &str, value: &str) -> Result<Option<String>, PluginError> {
        self.insert(key.to_string(), value.to_string())
    }
    
    pub fn get_str(&self, key: &str) -> Option<&String> {
        self.get(key)
    }
    
    // Zero-copy string operations with Cow
    pub fn get_cow<'a>(&'a self, key: &str) -> std::borrow::Cow<'a, str> {
        match self.get(key) {
            Some(value) => std::borrow::Cow::Borrowed(value),
            None => std::borrow::Cow::Borrowed(""),
        }
    }
}
```

### 4. Iterator Integration with Generic Traits
```rust
// Custom iterator with generic bounds
pub struct MapIterator<'a, K, V, F>
where
    F: Fn(&K, &V) -> bool,
{
    inner: std::collections::hash_map::Iter<'a, K, V>,
    filter: F,
}

impl<'a, K, V, F> Iterator for MapIterator<'a, K, V, F>
where
    F: Fn(&K, &V) -> bool,
{
    type Item = (&'a K, &'a V);
    
    fn next(&mut self) -> Option<Self::Item> {
        while let Some((k, v)) = self.inner.next() {
            if (self.filter)(k, v) {
                return Some((k, v));
            }
        }
        None
    }
}

impl<K, V, S> AdvancedHashMap<K, V, S>
where
    K: Hash + Eq,
    S: BuildHasher,
{
    // Generic filtered iteration
    pub fn iter_filtered<F>(&self, filter: F) -> MapIterator<K, V, F>
    where
        F: Fn(&K, &V) -> bool,
    {
        MapIterator {
            inner: self.inner.iter(),
            filter,
        }
    }
    
    // Collect with generic transformation
    pub fn collect_transformed<T, F>(&self, transform: F) -> Vec<T>
    where
        F: Fn(&K, &V) -> T,
    {
        self.inner.iter().map(|(k, v)| transform(k, v)).collect()
    }
    
    // Generic reduce operation
    pub fn reduce<T, F>(&self, initial: T, reducer: F) -> T
    where
        F: Fn(T, &K, &V) -> T,
    {
        self.inner.iter().fold(initial, |acc, (k, v)| reducer(acc, k, v))
    }
}
```

### 5. Plugin System with Trait Objects
```rust
// Concrete plugin implementations
pub struct LoggingPlugin {
    name: String,
}

impl LoggingPlugin {
    pub fn new(name: String) -> Self {
        LoggingPlugin { name }
    }
}

impl<K, V> MapPlugin<K, V> for LoggingPlugin
where
    K: std::fmt::Debug,
    V: std::fmt::Debug,
{
    fn name(&self) -> &str {
        &self.name
    }
    
    fn on_insert(&self, key: &K, value: &V) -> Result<(), PluginError> {
        println!("INSERT: {:?} -> {:?}", key, value);
        Ok(())
    }
    
    fn on_get(&self, key: &K, value: Option<&V>) -> Result<(), PluginError> {
        match value {
            Some(v) => println!("GET: {:?} -> {:?}", key, v),
            None => println!("GET: {:?} -> NOT_FOUND", key),
        }
        Ok(())
    }
    
    fn on_remove(&self, key: &K, value: Option<V>) -> Result<(), PluginError> {
        match value {
            Some(v) => println!("REMOVE: {:?} -> {:?}", key, v),
            None => println!("REMOVE: {:?} -> NOT_FOUND", key),
        }
        Ok(())
    }
}

pub struct ValidationPlugin<F>
where
    F: Fn(&str, &str) -> bool + Send + Sync,
{
    validator: F,
}

impl<F> ValidationPlugin<F>
where
    F: Fn(&str, &str) -> bool + Send + Sync,
{
    pub fn new(validator: F) -> Self {
        ValidationPlugin { validator }
    }
}

impl<F> MapPlugin<String, String> for ValidationPlugin<F>
where
    F: Fn(&str, &str) -> bool + Send + Sync,
{
    fn name(&self) -> &str {
        "ValidationPlugin"
    }
    
    fn on_insert(&self, key: &String, value: &String) -> Result<(), PluginError> {
        if (self.validator)(key, value) {
            Ok(())
        } else {
            Err(PluginError {
                message: format!("Validation failed for key '{}' with value '{}'", key, value),
                plugin_name: self.name().to_string(),
            })
        }
    }
    
    fn on_get(&self, _key: &String, _value: Option<&String>) -> Result<(), PluginError> {
        Ok(())  // No validation needed for reads
    }
    
    fn on_remove(&self, _key: &String, _value: Option<String>) -> Result<(), PluginError> {
        Ok(())  // No validation needed for removal
    }
    
    fn priority(&self) -> u8 {
        10  // High priority - validate before other operations
    }
}

// Plugin manager with trait objects
impl<K, V, S> AdvancedHashMap<K, V, S>
where
    K: Hash + Eq,
    S: BuildHasher,
{
    pub fn add_plugin(&mut self, plugin: Box<dyn MapPlugin<K, V>>) {
        self.plugins.push(plugin);
        // Sort by priority (high to low)
        self.plugins.sort_by_key(|p| std::cmp::Reverse(p.priority()));
    }
    
    pub fn remove_plugin(&mut self, name: &str) {
        self.plugins.retain(|p| p.name() != name);
    }
    
    pub fn list_plugins(&self) -> Vec<(&str, u8)> {
        self.plugins.iter()
                   .map(|p| (p.name(), p.priority()))
                   .collect()
    }
}
```

### 6. Builder Pattern with Generic Constraints
```rust
// Builder pattern combining all concepts
pub struct HashMapBuilder<K, V, S = RandomState>
where
    K: Hash + Eq,
    S: BuildHasher,
{
    hasher: Option<S>,
    capacity: Option<usize>,
    plugins: Vec<Box<dyn MapPlugin<K, V>>>,
    _phantom: PhantomData<(K, V)>,
}

impl<K, V> HashMapBuilder<K, V, RandomState>
where
    K: Hash + Eq,
{
    pub fn new() -> Self {
        HashMapBuilder {
            hasher: None,
            capacity: None,
            plugins: Vec::new(),
            _phantom: PhantomData,
        }
    }
}

impl<K, V, S> HashMapBuilder<K, V, S>
where
    K: Hash + Eq,
    S: BuildHasher,
{
    pub fn with_hasher<NewS>(self, hasher: NewS) -> HashMapBuilder<K, V, NewS>
    where
        NewS: BuildHasher,
    {
        HashMapBuilder {
            hasher: Some(hasher),
            capacity: self.capacity,
            plugins: self.plugins,
            _phantom: PhantomData,
        }
    }
    
    pub fn with_capacity(mut self, capacity: usize) -> Self {
        self.capacity = Some(capacity);
        self
    }
    
    pub fn add_plugin(mut self, plugin: Box<dyn MapPlugin<K, V>>) -> Self {
        self.plugins.push(plugin);
        self
    }
    
    pub fn build(mut self) -> AdvancedHashMap<K, V, S>
    where
        S: Default,
    {
        let hasher = self.hasher.unwrap_or_default();
        let mut map = if let Some(capacity) = self.capacity {
            AdvancedHashMap {
                inner: HashMap::with_capacity_and_hasher(capacity, hasher),
                metadata: MapMetadata::new(),
                plugins: Vec::new(),
            }
        } else {
            AdvancedHashMap {
                inner: HashMap::with_hasher(hasher),
                metadata: MapMetadata::new(),
                plugins: Vec::new(),
            }
        };
        
        // Sort plugins by priority and add them
        self.plugins.sort_by_key(|p| std::cmp::Reverse(p.priority()));
        map.plugins = self.plugins;
        
        map
    }
}
```

## Real-World Integration Example

### Complete Application: User Management System
```rust
use std::collections::hash_map::RandomState;

// Domain types showcasing all concepts
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct UserId(pub u64);

#[derive(Debug, Clone)]
pub struct User {
    pub id: UserId,
    pub name: String,
    pub email: String,
    pub role: UserRole,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UserRole {
    Admin,
    User,
    Guest,
}

// Trait implementations
impl Keyable for User {
    type Key = UserId;
    
    fn key(&self) -> Self::Key {
        self.id.clone()
    }
}

impl Valuable for User {
    type Value = User;
    
    fn value(&self) -> Self::Value {
        self.clone()
    }
}

impl Entry for User {
    fn into_pair(self) -> (Self::Key, Self::Value) {
        let key = self.key();
        (key, self)
    }
    
    fn from_pair(key: Self::Key, value: Self::Value) -> Self {
        // Ensure consistency
        assert_eq!(key, value.id);
        value
    }
}

// Specialized user management HashMap
pub type UserMap = AdvancedHashMap<UserId, User>;

// User-specific plugins
pub struct UserAuditPlugin {
    audit_log: Vec<String>,
}

impl UserAuditPlugin {
    pub fn new() -> Self {
        UserAuditPlugin {
            audit_log: Vec::new(),
        }
    }
    
    pub fn get_audit_log(&self) -> &[String] {
        &self.audit_log
    }
}

impl MapPlugin<UserId, User> for UserAuditPlugin {
    fn name(&self) -> &str {
        "UserAuditPlugin"
    }
    
    fn on_insert(&self, key: &UserId, value: &User) -> Result<(), PluginError> {
        // Note: In real implementation, you'd need interior mutability
        println!("AUDIT: User {} ({}) added", value.name, key.0);
        Ok(())
    }
    
    fn on_get(&self, key: &UserId, value: Option<&User>) -> Result<(), PluginError> {
        match value {
            Some(user) => println!("AUDIT: User {} accessed", user.name),
            None => println!("AUDIT: Access attempt for unknown user {}", key.0),
        }
        Ok(())
    }
    
    fn on_remove(&self, key: &UserId, value: Option<User>) -> Result<(), PluginError> {
        match value {
            Some(user) => println!("AUDIT: User {} ({}) removed", user.name, key.0),
            None => println!("AUDIT: Removal attempt for unknown user {}", key.0),
        }
        Ok(())
    }
    
    fn priority(&self) -> u8 {
        5  // Medium priority
    }
}

// Complete user management system
pub struct UserManager {
    users: UserMap,
}

impl UserManager {
    pub fn new() -> Self {
        let mut map = HashMapBuilder::new()
            .with_capacity(100)
            .add_plugin(Box::new(LoggingPlugin::new("UserLog".to_string())))
            .add_plugin(Box::new(UserAuditPlugin::new()))
            .build();
        
        UserManager { users: map }
    }
    
    // Generic operations with full type system integration
    pub fn add_user(&mut self, user: User) -> Result<(), PluginError> {
        let id = user.id.clone();
        self.users.insert(id, user)?;
        Ok(())
    }
    
    pub fn get_user(&self, id: &UserId) -> Option<&User> {
        self.users.get(id)
    }
    
    pub fn find_users_by_role(&self, role: &UserRole) -> Vec<&User> {
        self.users
            .iter_filtered(|_id, user| &user.role == role)
            .map(|(_id, user)| user)
            .collect()
    }
    
    pub fn get_user_names(&self) -> Vec<String> {
        self.users.collect_transformed(|_id, user| user.name.clone())
    }
    
    pub fn count_by_role(&self, role: &UserRole) -> usize {
        self.users.reduce(0, |count, _id, user| {
            if &user.role == role {
                count + 1
            } else {
                count
            }
        })
    }
    
    pub fn stats(&self) -> (u64, u64) {
        self.users.metadata.stats()
    }
}
```

## Performance Optimization with Generic Specialization

### Zero-Cost Abstractions in Practice
```rust
// Demonstrating zero-cost abstractions
use std::hash::{Hash, Hasher};

// Generic hash computation with specialization
pub trait FastHash {
    fn fast_hash<H: Hasher>(&self, hasher: &mut H);
}

// Optimized for primitive types
impl FastHash for u64 {
    fn fast_hash<H: Hasher>(&self, hasher: &mut H) {
        hasher.write_u64(*self);  // Direct write, no allocations
    }
}

impl FastHash for String {
    fn fast_hash<H: Hasher>(&self, hasher: &mut H) {
        hasher.write(self.as_bytes());  // Direct byte access
    }
}

// Performance-critical HashMap operations
impl<K, V, S> AdvancedHashMap<K, V, S>
where
    K: Hash + Eq + FastHash,
    S: BuildHasher,
{
    // Optimized bulk insert
    pub fn bulk_insert<I>(&mut self, items: I) -> Result<usize, PluginError>
    where
        I: IntoIterator<Item = (K, V)>,
        V: Clone,
    {
        let mut count = 0;
        for (key, value) in items {
            self.insert(key, value)?;
            count += 1;
        }
        Ok(count)
    }
    
    // Zero-allocation key existence check
    pub fn contains_key_fast<Q>(&self, key: &Q) -> bool
    where
        K: std::borrow::Borrow<Q>,
        Q: Hash + Eq + ?Sized + FastHash,
    {
        self.inner.contains_key(key)
    }
}
```

## Testing Integration Patterns

### Generic Test Framework
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    // Generic test helper
    fn create_test_map<K, V>() -> AdvancedHashMap<K, V>
    where
        K: Hash + Eq + Clone,
        V: Clone,
    {
        HashMapBuilder::new()
            .with_capacity(10)
            .build()
    }
    
    // Test trait bounds
    #[test]
    fn test_generic_operations() {
        let mut map: AdvancedHashMap<String, i32> = create_test_map();
        
        // Test insertion
        assert!(map.insert("key1".to_string(), 42).unwrap().is_none());
        assert_eq!(map.insert("key1".to_string(), 43).unwrap(), Some(42));
        
        // Test retrieval
        assert_eq!(map.get("key1"), Some(&43));
        assert_eq!(map.get("nonexistent"), None);
        
        // Test stats
        let (access_count, modification_count) = map.metadata.stats();
        assert_eq!(access_count, 2);
        assert_eq!(modification_count, 2);
    }
    
    // Test plugin system
    #[test]
    fn test_plugin_integration() {
        let mut map: AdvancedHashMap<String, String> = HashMapBuilder::new()
            .add_plugin(Box::new(LoggingPlugin::new("TestLog".to_string())))
            .build();
        
        assert!(map.insert("test".to_string(), "value".to_string()).is_ok());
        assert_eq!(map.get("test"), Some(&"value".to_string()));
    }
    
    // Test complex trait combinations
    #[test]
    fn test_user_management() {
        let mut manager = UserManager::new();
        
        let user = User {
            id: UserId(1),
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
            role: UserRole::Admin,
        };
        
        assert!(manager.add_user(user.clone()).is_ok());
        assert_eq!(manager.get_user(&UserId(1)), Some(&user));
        
        let admins = manager.find_users_by_role(&UserRole::Admin);
        assert_eq!(admins.len(), 1);
        assert_eq!(admins[0].name, "Alice");
        
        let admin_count = manager.count_by_role(&UserRole::Admin);
        assert_eq!(admin_count, 1);
    }
}
```

## Best Practices Summary

### Design Guidelines for Generic + Trait Systems
```rust
// ✅ Good: Minimal, focused trait definitions
trait Displayable {
    fn display(&self) -> String;
}

// ✅ Good: Generic bounds only where needed
fn process_items<T>(items: Vec<T>) -> Vec<String>
where
    T: Displayable,  // Only the constraint we actually need
{
    items.into_iter().map(|item| item.display()).collect()
}

// ❌ Avoid: Over-constrained generics
// fn bad_process<T>(items: Vec<T>) -> Vec<String>
// where
//     T: Clone + Send + Sync + Debug + Display + Default,  // Too many constraints!
// {
//     // Implementation only uses Display
// }

// ✅ Good: Associated types for fixed relationships
trait Iterator {
    type Item;  // Each iterator type has one Item type
    fn next(&mut self) -> Option<Self::Item>;
}

// ✅ Good: Default implementations for common cases
trait Plugin {
    fn name(&self) -> &str;
    fn execute(&self) -> Result<(), String>;
    
    fn is_enabled(&self) -> bool { true }  // Reasonable default
    fn priority(&self) -> u8 { 0 }         // Reasonable default
}
```

## Learning Progression Summary

From Day 21, you should master:
1. **Integration Patterns**: Combining traits, generics, lifetimes, and trait objects
2. **Advanced Constraints**: Complex where clauses and bound combinations  
3. **Performance Awareness**: Zero-cost abstractions and specialization
4. **Plugin Architecture**: Extensible systems using trait objects
5. **Builder Patterns**: Flexible construction with generic parameters
6. **Real-World Design**: Complete API design using all Week 3 concepts

**Week 3 Complete Mastery:**
- **Days 15-18**: Foundation concepts (traits → generics → lifetimes → advanced traits)
- **Days 19-20**: Dynamic dispatch and advanced lifetime patterns
- **Day 21**: Integration project demonstrating real-world application

**Mission5 Requirements Fulfilled:**
- **REQ-8**: Advanced API design using complete type system
- **REQ-7**: Zero-copy operations with lifetime management
- **REQ-6**: Flexible APIs using trait objects for extensibility
- **REQ-5**: Generic data structures for type safety

**Cross-References:**
- [[Collections MOC]] - Complete type system patterns across all collection designs
- [[mission-5]] - REQ-8 advanced HashMap implementation with all Week 3 concepts
- [[HashMap Internals]] - Implementation details for performance-critical generic code

**Next Week**: Week 4 will focus on **Applied Problem Solving** with a emphasis on AoC-Style Problems

---
**Zettelkasten Integration:**
*Links: [[Collections MOC]] | [[mission-5]] | [[HashMap Internals]] | [[zettel-index]]*

*Tags: #generics-traits-practice #integration-project #mission5 #advanced-api-design #type-system-mastery #daily-study #week3 #real-world-application*

## 🚀 **Complete Runnable Example**

```rust
// Copy this entire block to Rust Playground or save as a .rs file
use std::collections::HashMap;
use std::hash::{Hash, BuildHasher, RandomState};
use std::marker::PhantomData;

fn main() {
    println!("=== Generics + Traits Integration from Day 21 ===\n");
    
    basic_integration_demo();
    plugin_system_demo();
    user_management_demo();
    iterator_patterns_demo();
    builder_pattern_demo();
}

// 1. Basic integration of generics and traits
trait Keyable {
    type Key: Hash + Eq + Clone;
    fn key(&self) -> Self::Key;
}

trait Valuable {
    type Value: Clone;
    fn value(&self) -> Self::Value;
}

trait Entry: Keyable + Valuable {
    fn into_pair(self) -> (Self::Key, Self::Value);
}

#[derive(Debug, Clone)]
struct Person {
    id: u32,
    name: String,
    age: u32,
}

impl Keyable for Person {
    type Key = u32;
    fn key(&self) -> Self::Key {
        self.id
    }
}

impl Valuable for Person {
    type Value = Person;
    fn value(&self) -> Self::Value {
        self.clone()
    }
}

impl Entry for Person {
    fn into_pair(self) -> (Self::Key, Self::Value) {
        let key = self.key();
        (key, self)
    }
}

// Generic storage with trait bounds
struct Storage<T>
where
    T: Entry,
{
    items: HashMap<T::Key, T::Value>,
    count: usize,
}

impl<T> Storage<T>
where
    T: Entry,
{
    fn new() -> Self {
        Storage {
            items: HashMap::new(),
            count: 0,
        }
    }
    
    fn add(&mut self, item: T) {
        let (key, value) = item.into_pair();
        self.items.insert(key, value);
        self.count += 1;
    }
    
    fn get(&self, key: &T::Key) -> Option<&T::Value> {
        self.items.get(key)
    }
    
    fn len(&self) -> usize {
        self.count
    }
}

fn basic_integration_demo() {
    println!("1. Basic Generics + Traits Integration:");
    
    let mut storage = Storage::new();
    
    storage.add(Person {
        id: 1,
        name: "Alice".to_string(),
        age: 30,
    });
    
    storage.add(Person {
        id: 2,
        name: "Bob".to_string(),
        age: 25,
    });
    
    println!("   Storage contains {} people", storage.len());
    
    if let Some(person) = storage.get(&1) {
        println!("   Found person: {} (age {})", person.name, person.age);
    }
    
    if let Some(person) = storage.get(&2) {
        println!("   Found person: {} (age {})", person.name, person.age);
    }
    
    println!();
}

// 2. Plugin system with trait objects
trait Plugin<T>: Send + Sync {
    fn name(&self) -> &str;
    fn process(&self, item: &T) -> Result<(), String>;
    fn priority(&self) -> u8 { 0 }
}

struct LoggingPlugin {
    prefix: String,
}

impl LoggingPlugin {
    fn new(prefix: String) -> Self {
        LoggingPlugin { prefix }
    }
}

impl<T> Plugin<T> for LoggingPlugin
where
    T: std::fmt::Debug,
{
    fn name(&self) -> &str {
        "LoggingPlugin"
    }
    
    fn process(&self, item: &T) -> Result<(), String> {
        println!("   {}: Processing {:?}", self.prefix, item);
        Ok(())
    }
    
    fn priority(&self) -> u8 {
        1
    }
}

struct ValidationPlugin<F>
where
    F: Fn(&Person) -> bool + Send + Sync,
{
    validator: F,
}

impl<F> ValidationPlugin<F>
where
    F: Fn(&Person) -> bool + Send + Sync,
{
    fn new(validator: F) -> Self {
        ValidationPlugin { validator }
    }
}

impl<F> Plugin<Person> for ValidationPlugin<F>
where
    F: Fn(&Person) -> bool + Send + Sync,
{
    fn name(&self) -> &str {
        "ValidationPlugin"
    }
    
    fn process(&self, item: &Person) -> Result<(), String> {
        if (self.validator)(item) {
            println!("   Validation passed for {}", item.name);
            Ok(())
        } else {
            Err(format!("Validation failed for {}", item.name))
        }
    }
    
    fn priority(&self) -> u8 {
        10  // High priority - validate first
    }
}

struct PluginManager<T> {
    plugins: Vec<Box<dyn Plugin<T>>>,
}

impl<T> PluginManager<T> {
    fn new() -> Self {
        PluginManager {
            plugins: Vec::new(),
        }
    }
    
    fn add_plugin(&mut self, plugin: Box<dyn Plugin<T>>) {
        self.plugins.push(plugin);
        // Sort by priority (high to low)
        self.plugins.sort_by_key(|p| std::cmp::Reverse(p.priority()));
    }
    
    fn process_item(&self, item: &T) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        
        for plugin in &self.plugins {
            if let Err(e) = plugin.process(item) {
                errors.push(format!("{}: {}", plugin.name(), e));
            }
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

fn plugin_system_demo() {
    println!("2. Plugin System with Trait Objects:");
    
    let mut manager = PluginManager::new();
    
    manager.add_plugin(Box::new(LoggingPlugin::new("AUDIT".to_string())));
    manager.add_plugin(Box::new(ValidationPlugin::new(|person: &Person| {
        person.age >= 18  // Must be adult
    })));
    
    let alice = Person { id: 1, name: "Alice".to_string(), age: 30 };
    let child = Person { id: 2, name: "Child".to_string(), age: 10 };
    
    println!("   Processing Alice:");
    match manager.process_item(&alice) {
        Ok(()) => println!("   ✓ Processing completed successfully"),
        Err(errors) => println!("   ✗ Errors: {:?}", errors),
    }
    
    println!("   Processing Child:");
    match manager.process_item(&child) {
        Ok(()) => println!("   ✓ Processing completed successfully"),
        Err(errors) => println!("   ✗ Errors: {:?}", errors),
    }
    
    println!();
}

// 3. Advanced user management system
#[derive(Debug, Clone, PartialEq)]
enum Role {
    Admin,
    User,
    Guest,
}

struct User {
    id: u32,
    name: String,
    role: Role,
}

// Generic HashMap wrapper with metadata
struct AdvancedMap<K, V>
where
    K: Hash + Eq,
{
    data: HashMap<K, V>,
    access_count: u64,
    modification_count: u64,
}

impl<K, V> AdvancedMap<K, V>
where
    K: Hash + Eq,
{
    fn new() -> Self {
        AdvancedMap {
            data: HashMap::new(),
            access_count: 0,
            modification_count: 0,
        }
    }
    
    fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.modification_count += 1;
        self.data.insert(key, value)
    }
    
    fn get(&mut self, key: &K) -> Option<&V> {
        self.access_count += 1;
        self.data.get(key)
    }
    
    fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.data.iter()
    }
    
    fn stats(&self) -> (u64, u64) {
        (self.access_count, self.modification_count)
    }
    
    // Generic filtering with closures
    fn find_all<F>(&self, predicate: F) -> Vec<&V>
    where
        F: Fn(&K, &V) -> bool,
    {
        self.data
            .iter()
            .filter(|(k, v)| predicate(k, v))
            .map(|(_, v)| v)
            .collect()
    }
}

struct UserManager {
    users: AdvancedMap<u32, User>,
}

impl UserManager {
    fn new() -> Self {
        UserManager {
            users: AdvancedMap::new(),
        }
    }
    
    fn add_user(&mut self, user: User) {
        let id = user.id;
        self.users.insert(id, user);
    }
    
    fn get_user(&mut self, id: u32) -> Option<&User> {
        self.users.get(&id)
    }
    
    fn find_by_role(&self, role: &Role) -> Vec<&User> {
        self.users.find_all(|_id, user| &user.role == role)
    }
    
    fn list_all(&self) -> Vec<&User> {
        self.users.iter().map(|(_, user)| user).collect()
    }
    
    fn stats(&self) -> (u64, u64) {
        self.users.stats()
    }
}

fn user_management_demo() {
    println!("3. Advanced User Management System:");
    
    let mut manager = UserManager::new();
    
    manager.add_user(User {
        id: 1,
        name: "Alice".to_string(),
        role: Role::Admin,
    });
    
    manager.add_user(User {
        id: 2,
        name: "Bob".to_string(),
        role: Role::User,
    });
    
    manager.add_user(User {
        id: 3,
        name: "Charlie".to_string(),
        role: Role::User,
    });
    
    manager.add_user(User {
        id: 4,
        name: "Guest".to_string(),
        role: Role::Guest,
    });
    
    // Find users by role
    let admins = manager.find_by_role(&Role::Admin);
    println!("   Admins: {:?}", admins.iter().map(|u| &u.name).collect::<Vec<_>>());
    
    let users = manager.find_by_role(&Role::User);
    println!("   Regular users: {:?}", users.iter().map(|u| &u.name).collect::<Vec<_>>());
    
    // Access some users
    if let Some(user) = manager.get_user(1) {
        println!("   Found user 1: {} (role: {:?})", user.name, user.role);
    }
    
    if let Some(user) = manager.get_user(2) {
        println!("   Found user 2: {} (role: {:?})", user.name, user.role);
    }
    
    let (access_count, modification_count) = manager.stats();
    println!("   Stats: {} accesses, {} modifications", access_count, modification_count);
    
    println!();
}

// 4. Advanced iterator patterns
trait CollectExt<T> {
    fn collect_mapped<U, F>(self, f: F) -> Vec<U>
    where
        F: Fn(T) -> U;
        
    fn collect_filtered<F>(self, predicate: F) -> Vec<T>
    where
        F: Fn(&T) -> bool;
}

impl<I, T> CollectExt<T> for I
where
    I: Iterator<Item = T>,
{
    fn collect_mapped<U, F>(self, f: F) -> Vec<U>
    where
        F: Fn(T) -> U,
    {
        self.map(f).collect()
    }
    
    fn collect_filtered<F>(self, predicate: F) -> Vec<T>
    where
        F: Fn(&T) -> bool,
    {
        self.filter(predicate).collect()
    }
}

// Generic data processor
struct DataProcessor<T> {
    items: Vec<T>,
}

impl<T> DataProcessor<T>
where
    T: Clone,
{
    fn new(items: Vec<T>) -> Self {
        DataProcessor { items }
    }
    
    fn process_with<F, U>(&self, processor: F) -> Vec<U>
    where
        F: Fn(&T) -> U,
    {
        self.items.iter().collect_mapped(processor)
    }
    
    fn filter_items<F>(&self, predicate: F) -> Vec<T>
    where
        F: Fn(&T) -> bool,
    {
        self.items.iter().cloned().collect_filtered(predicate)
    }
    
    fn reduce_to<U, F>(&self, initial: U, reducer: F) -> U
    where
        F: Fn(U, &T) -> U,
    {
        self.items.iter().fold(initial, reducer)
    }
}

fn iterator_patterns_demo() {
    println!("4. Advanced Iterator Patterns:");
    
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let processor = DataProcessor::new(numbers);
    
    // Transform numbers to strings
    let strings = processor.process_with(|&n| format!("Number: {}", n));
    println!("   Transformed: {:?}", &strings[..3]); // Show first 3
    
    // Filter even numbers
    let evens = processor.filter_items(|&n| n % 2 == 0);
    println!("   Even numbers: {:?}", evens);
    
    // Sum all numbers
    let sum = processor.reduce_to(0, |acc, &n| acc + n);
    println!("   Sum: {}", sum);
    
    // Product of odd numbers
    let odd_product = processor
        .filter_items(|&n| n % 2 == 1)
        .iter()
        .fold(1, |acc, &n| acc * n);
    println!("   Product of odds: {}", odd_product);
    
    println!();
}

// 5. Builder pattern with generics
struct ConfigBuilder<T> {
    items: Vec<T>,
    capacity: Option<usize>,
    name: Option<String>,
}

impl<T> ConfigBuilder<T> {
    fn new() -> Self {
        ConfigBuilder {
            items: Vec::new(),
            capacity: None,
            name: None,
        }
    }
    
    fn add_item(mut self, item: T) -> Self {
        self.items.push(item);
        self
    }
    
    fn with_capacity(mut self, capacity: usize) -> Self {
        self.capacity = Some(capacity);
        self
    }
    
    fn with_name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = Some(name.into());
        self
    }
    
    fn build(self) -> Config<T> {
        Config {
            items: self.items,
            capacity: self.capacity.unwrap_or(10),
            name: self.name.unwrap_or_else(|| "Default".to_string()),
        }
    }
}

struct Config<T> {
    items: Vec<T>,
    capacity: usize,
    name: String,
}

impl<T> Config<T>
where
    T: std::fmt::Debug,
{
    fn info(&self) -> String {
        format!(
            "Config '{}': {} items, capacity {}, items: {:?}",
            self.name,
            self.items.len(),
            self.capacity,
            &self.items[..self.items.len().min(3)] // Show first 3
        )
    }
}

fn builder_pattern_demo() {
    println!("5. Builder Pattern with Generics:");
    
    let config = ConfigBuilder::new()
        .with_name("NumberConfig")
        .with_capacity(20)
        .add_item(42)
        .add_item(84)
        .add_item(126)
        .build();
    
    println!("   {}", config.info());
    
    let string_config = ConfigBuilder::new()
        .with_name("StringConfig")
        .add_item("hello".to_string())
        .add_item("world".to_string())
        .build();
    
    println!("   {}", string_config.info());
    
    println!();
}
```

### **🛠️ How to Run This Code:**

1. **Online**: Copy to [Rust Playground](https://play.rust-lang.org/)
2. **Local file**: Save as `day21_demo.rs` and run `rustc day21_demo.rs && ./day21_demo`
3. **In this workspace**: `.\scripts\run_md.bat daily_study\rust_learning_week3_notes\Day21.md`
4. **As Cargo example**: `cargo run --example day21_integration_demo` (if you add it to Mission5_tut)
