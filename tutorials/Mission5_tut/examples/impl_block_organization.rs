/// Demonstration of different ways to organize impl blocks
/// Shows the tradeoffs between grouping methods by constraints vs functionality
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug)]
pub struct Counter<K>
where
    K: Eq + Hash,
{
    counts: HashMap<K, usize>,
}

// =====================================
// CURRENT APPROACH: Group by Functionality
// =====================================
impl<K> Default for Counter<K>
where
    K: Eq + Hash,
 {
    fn default() -> Self {
        Self::new()
    }
}

impl<K> Counter<K>
where
    K: Eq + Hash,
{
    pub fn new() -> Self {
        Self {
            counts: HashMap::new(),
        }
    }

    pub fn increment(&mut self, key: K) {
        *self.counts.entry(key).or_insert(0) += 1;
    }

    pub fn get(&self, key: &K) -> usize {
        self.counts.get(key).copied().unwrap_or(0)
    }
}

impl<K> Counter<K>
where
    K: Eq + Hash + Clone, // More restrictive bounds
{
    // This method doesn't actually need Clone!
    pub fn count_multiple<I>(&mut self, items: I)
    where
        I: IntoIterator<Item = K>,
    {
        for item in items {
            self.increment(item); // Just calls increment - no cloning!
        }
    }

    // This method DOES need Clone
    pub fn most_common(&self, n: usize) -> Vec<(K, usize)> {
        let mut items: Vec<_> = self
            .counts
            .iter()
            .map(|(k, &v)| (k.clone(), v)) // <-- CLONE USED HERE
            .collect();

        items.sort_by(|a, b| b.1.cmp(&a.1));
        items.into_iter().take(n).collect()
    }
}

// =====================================
// ALTERNATIVE APPROACH: Group by Constraints
// =====================================
pub struct AlternativeCounter<K>
where
    K: Eq + Hash,
{
    counts: HashMap<K, usize>,
}

// Block 1: Basic operations (Eq + Hash only)
impl<K> Default for AlternativeCounter<K>
where
    K: Eq + Hash,
 {
    fn default() -> Self {
        Self::new()
    }
}

impl<K> AlternativeCounter<K>
where
    K: Eq + Hash,
{
    pub fn new() -> Self {
        Self {
            counts: HashMap::new(),
        }
    }

    pub fn increment(&mut self, key: K) {
        *self.counts.entry(key).or_insert(0) += 1;
    }

    pub fn get(&self, key: &K) -> usize {
        self.counts.get(key).copied().unwrap_or(0)
    }

    // count_multiple could go here since it doesn't actually need Clone
    pub fn count_multiple<I>(&mut self, items: I)
    where
        I: IntoIterator<Item = K>,
    {
        for item in items {
            self.increment(item); // No cloning needed
        }
    }
}

// Block 2: Analysis operations (Eq + Hash + Clone)
impl<K> AlternativeCounter<K>
where
    K: Eq + Hash + Clone,
{
    // Only methods that actually need Clone go here
    pub fn most_common(&self, n: usize) -> Vec<(K, usize)> {
        let mut items: Vec<_> = self
            .counts
            .iter()
            .map(|(k, &v)| (k.clone(), v)) // Clone is actually needed
            .collect();

        items.sort_by(|a, b| b.1.cmp(&a.1));
        items.into_iter().take(n).collect()
    }
}

fn main() {
    println!("🔍 Impl Block Organization Comparison");
    println!("=====================================");

    // Both approaches work the same from the user's perspective
    let mut counter1 = Counter::new();
    let mut counter2 = AlternativeCounter::new();

    // Basic operations work on both
    counter1.increment("a");
    counter2.increment("a");

    let items = vec!["b", "a", "c", "a"];
    counter1.count_multiple(items.clone());
    counter2.count_multiple(items);

    println!("Counter1 'a' count: {}", counter1.get(&"a"));
    println!("Counter2 'a' count: {}", counter2.get(&"a"));

    // Analysis operations work on both (since &str implements Clone)
    let common1 = counter1.most_common(2);
    let common2 = counter2.most_common(2);

    println!("Most common (approach 1): {:?}", common1);
    println!("Most common (approach 2): {:?}", common2);

    println!("\n💡 Key Insight:");
    println!("count_multiple() could be in the first impl block since it doesn't clone!");
    println!("The current design groups it with analysis methods for logical organization.");
}
