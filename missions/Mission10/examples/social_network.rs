//! # Social Network Analysis with Union-Find
//!
//! This example demonstrates using Union-Find for social network analysis,
//! including friend circle detection, community formation, and network growth
//! patterns. Union-Find is particularly effective for these scenarios because
//! social connections are typically bidirectional and permanent.
//!
//! ## Use Cases
//!
//! - **Friend suggestion systems**: Find mutual connections and suggest friends
//! - **Community detection**: Identify tightly connected social groups
//! - **Social graph analysis**: Track network growth and fragmentation
//! - **Privacy controls**: Determine information visibility across friend circles
//!
//! ## Time Complexity
//!
//! - **Add friendship**: O(α(n)) amortized
//! - **Check mutual friends**: O(α(n)) amortized  
//! - **Find community size**: O(α(n)) amortized
//! - **List all communities**: O(n × α(n))

use mission10::UnionFind;
use std::collections::{HashMap, HashSet};
use std::fmt;

/// Represents a person in the social network
#[derive(Debug, Clone)]
pub struct Person {
    pub id: usize,
    pub name: String,
    pub age: u32,
    pub interests: Vec<String>,
    pub location: String,
}

impl Person {
    pub fn new(
        id: usize,
        name: String,
        age: u32,
        interests: Vec<String>,
        location: String,
    ) -> Self {
        Self {
            id,
            name,
            age,
            interests,
            location,
        }
    }
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({}, {})", self.name, self.age, self.location)
    }
}

/// Represents a friendship connection with metadata
#[derive(Debug, Clone)]
pub struct Friendship {
    pub person1: usize,
    pub person2: usize,
    pub formed_at: std::time::SystemTime,
    pub connection_type: ConnectionType,
}

#[derive(Debug, Clone)]
pub enum ConnectionType {
    Mutual,   // Through mutual friends
    Direct,   // Direct introduction
    Interest, // Common interests
    Location, // Same location
    Online,   // Met online
}

impl fmt::Display for ConnectionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConnectionType::Mutual => write!(f, "mutual friends"),
            ConnectionType::Direct => write!(f, "direct introduction"),
            ConnectionType::Interest => write!(f, "common interests"),
            ConnectionType::Location => write!(f, "same location"),
            ConnectionType::Online => write!(f, "online meeting"),
        }
    }
}

/// Social network using Union-Find for efficient community detection
pub struct SocialNetwork {
    uf: UnionFind,
    people: HashMap<usize, Person>,
    friendships: Vec<Friendship>,
    next_person_id: usize,
}

impl SocialNetwork {
    /// Create a new social network with maximum capacity
    pub fn new(max_people: usize) -> Self {
        Self {
            uf: UnionFind::new(max_people),
            people: HashMap::new(),
            friendships: Vec::new(),
            next_person_id: 0,
        }
    }

    /// Add a person to the social network
    pub fn add_person(
        &mut self,
        name: String,
        age: u32,
        interests: Vec<String>,
        location: String,
    ) -> Result<usize, String> {
        if self.next_person_id >= self.uf.len() {
            return Err("Social network at maximum capacity".to_string());
        }

        let person_id = self.next_person_id;
        let person = Person::new(person_id, name, age, interests, location);

        self.people.insert(person_id, person);
        self.next_person_id += 1;

        Ok(person_id)
    }

    /// Add a friendship between two people
    pub fn add_friendship(
        &mut self,
        person1: usize,
        person2: usize,
        connection_type: ConnectionType,
    ) -> Result<bool, String> {
        if !self.people.contains_key(&person1) || !self.people.contains_key(&person2) {
            return Err("One or both people not found in network".to_string());
        }

        let were_friends = self.uf.connected(person1, person2)?;
        let friendship_added = self.uf.union(person1, person2)?;

        if friendship_added {
            let friendship = Friendship {
                person1,
                person2,
                formed_at: std::time::SystemTime::now(),
                connection_type,
            };

            self.friendships.push(friendship);
        }

        Ok(friendship_added && !were_friends)
    }

    /// Check if two people are friends (directly or through mutual connections)
    pub fn are_friends(&mut self, person1: usize, person2: usize) -> Result<bool, String> {
        self.uf.connected(person1, person2)
    }

    /// Get the size of someone's friend circle
    pub fn friend_circle_size(&mut self, person: usize) -> Result<usize, String> {
        self.uf.size(person)
    }

    /// Get all people in the same friend circle
    pub fn get_friend_circle(&mut self, person: usize) -> Result<Vec<&Person>, String> {
        let root = self.uf.find(person)?;
        let mut circle = Vec::new();

        for (id, person_obj) in &self.people {
            if self.uf.find(*id)? == root {
                circle.push(person_obj);
            }
        }

        Ok(circle)
    }

    /// Get all distinct communities in the network
    pub fn get_communities(&mut self) -> Result<Vec<Vec<&Person>>, String> {
        let mut communities: HashMap<usize, Vec<&Person>> = HashMap::new();

        for (id, person) in &self.people {
            let root = self.uf.find(*id)?;
            communities.entry(root).or_default().push(person);
        }

        Ok(communities.into_values().collect())
    }

    /// Suggest friends based on mutual connections
    pub fn suggest_friends(&mut self, person: usize, limit: usize) -> Result<Vec<&Person>, String> {
        // Get all the info we need first to avoid borrowing conflicts
        let person_root = self.uf.find(person)?;
        let person_obj = self.people.get(&person).ok_or("Person not found")?.clone();

        // Get friend IDs
        let mut friend_ids = HashSet::new();
        for id in self.people.keys() {
            if self.uf.find(*id)? == person_root {
                friend_ids.insert(*id);
            }
        }

        let mut suggestions = Vec::new();
        let mut scores: HashMap<usize, i32> = HashMap::new();

        // Score potential friends based on various factors
        for (id, candidate) in &self.people {
            if *id == person || friend_ids.contains(id) {
                continue; // Skip self and existing friends
            }

            let mut score = 0;

            // Common interests bonus
            let common_interests: HashSet<_> = person_obj
                .interests
                .iter()
                .filter(|interest| candidate.interests.contains(interest))
                .collect();
            score += common_interests.len() as i32 * 3;

            // Same location bonus
            if person_obj.location == candidate.location {
                score += 5;
            }

            // Similar age bonus (within 5 years)
            let age_diff = (person_obj.age as i32 - candidate.age as i32).abs();
            if age_diff <= 5 {
                score += 3;
            }

            // Mutual friends bonus (check if candidate has friends in our circle)
            let candidate_root = self.uf.find(*id)?;
            let mut candidate_friend_ids = HashSet::new();
            for cand_id in self.people.keys() {
                if self.uf.find(*cand_id)? == candidate_root {
                    candidate_friend_ids.insert(*cand_id);
                }
            }

            let mutual_count = friend_ids.intersection(&candidate_friend_ids).count();
            score += mutual_count as i32 * 2;

            if score > 0 {
                scores.insert(*id, score);
            }
        }

        // Sort by score and take top suggestions
        let mut scored_people: Vec<_> = scores.iter().collect();
        scored_people.sort_by(|a, b| b.1.cmp(a.1));

        for (id, _score) in scored_people.iter().take(limit) {
            if let Some(person) = self.people.get(id) {
                suggestions.push(person);
            }
        }

        Ok(suggestions)
    }

    /// Print network statistics
    pub fn print_statistics(&mut self) -> Result<(), String> {
        println!("📊 Social Network Statistics:");
        println!("   Total people: {}", self.people.len());
        println!("   Total friendships: {}", self.friendships.len());
        println!("   Communities: {}", self.uf.count());

        let communities = self.get_communities()?;

        if communities.len() > 1 {
            println!("   Community sizes:");
            let mut sizes: Vec<usize> = communities.iter().map(|c| c.len()).collect();
            sizes.sort_by(|a, b| b.cmp(a));

            for (i, size) in sizes.iter().enumerate() {
                println!("     Community {}: {} people", i + 1, size);
            }

            let largest_community = communities.iter().max_by_key(|c| c.len()).unwrap();
            println!(
                "   Largest community has {} people",
                largest_community.len()
            );
        }

        Ok(())
    }

    /// Print detailed community information
    pub fn print_communities(&mut self) -> Result<(), String> {
        let communities = self.get_communities()?;

        println!("🏘️  Community Details:");

        for (i, community) in communities.iter().enumerate() {
            if community.len() == 1 {
                println!("   👤 Isolated: {}", community[0]);
            } else {
                println!("   🏘️  Community {} ({} people):", i + 1, community.len());
                for person in community {
                    println!("      • {}", person);
                }

                // Analyze community characteristics
                let avg_age: f32 =
                    community.iter().map(|p| p.age as f32).sum::<f32>() / community.len() as f32;
                let locations: HashSet<_> = community.iter().map(|p| &p.location).collect();
                let all_interests: Vec<_> = community.iter().flat_map(|p| &p.interests).collect();
                let mut interest_counts: HashMap<&String, usize> = HashMap::new();

                for interest in &all_interests {
                    *interest_counts.entry(interest).or_insert(0) += 1;
                }

                println!("      📈 Average age: {:.1}", avg_age);
                println!(
                    "      📍 Locations: {}",
                    locations
                        .iter()
                        .map(|s| s.as_str())
                        .collect::<Vec<_>>()
                        .join(", ")
                );

                if let Some((popular_interest, count)) =
                    interest_counts.iter().max_by_key(|(_, v)| **v)
                {
                    if *count > 1 {
                        println!(
                            "      ⭐ Popular interest: {} ({} people)",
                            popular_interest, count
                        );
                    }
                }
            }
            println!();
        }

        Ok(())
    }

    /// Simulate viral spread through the network
    pub fn simulate_viral_spread(
        &mut self,
        starting_person: usize,
        spread_probability: f64,
    ) -> Result<Vec<usize>, String> {
        if !self.people.contains_key(&starting_person) {
            return Err("Starting person not found".to_string());
        }

        println!(
            "🦠 Simulating viral spread starting from {}",
            self.people[&starting_person].name
        );

        let friend_circle = self.get_friend_circle(starting_person)?;
        let friend_circle_size = friend_circle.len();
        let mut infected = vec![starting_person];

        // Simple simulation: each friend has spread_probability chance of getting infected
        for person in friend_circle {
            if person.id != starting_person && rand::random::<f64>() < spread_probability {
                infected.push(person.id);
            }
        }

        println!(
            "   📊 Infected {} out of {} people in the community",
            infected.len(),
            friend_circle_size
        );

        for id in &infected[1..] {
            // Skip the starting person
            println!("   • {}", self.people[id].name);
        }

        Ok(infected)
    }

    /// Get person by ID
    pub fn get_person(&self, id: usize) -> Option<&Person> {
        self.people.get(&id)
    }
}

// Use a simple random number generator for demo purposes
mod rand {
    use std::cell::Cell;

    thread_local! {
        static SEED: Cell<u64> = const { Cell::new(1) };
    }

    pub fn random<T>() -> T
    where
        T: From<f64>,
    {
        SEED.with(|seed| {
            let current = seed.get();
            // Simple LCG
            let next = current.wrapping_mul(1664525).wrapping_add(1013904223);
            seed.set(next);
            T::from((next as f64) / (u64::MAX as f64))
        })
    }
}

/// Example: Building a university social network
fn example_university_network() -> Result<(), String> {
    println!("=== University Social Network Example ===");

    let mut network = SocialNetwork::new(50);

    // Add students from different departments
    let alice = network.add_person(
        "Alice".to_string(),
        20,
        vec![
            "Computer Science".to_string(),
            "Gaming".to_string(),
            "Anime".to_string(),
        ],
        "Dorm A".to_string(),
    )?;

    let bob = network.add_person(
        "Bob".to_string(),
        21,
        vec![
            "Computer Science".to_string(),
            "Basketball".to_string(),
            "Music".to_string(),
        ],
        "Dorm A".to_string(),
    )?;

    let charlie = network.add_person(
        "Charlie".to_string(),
        19,
        vec![
            "Mathematics".to_string(),
            "Gaming".to_string(),
            "Chess".to_string(),
        ],
        "Dorm B".to_string(),
    )?;

    let diana = network.add_person(
        "Diana".to_string(),
        22,
        vec![
            "Biology".to_string(),
            "Photography".to_string(),
            "Hiking".to_string(),
        ],
        "Off-campus".to_string(),
    )?;

    let eve = network.add_person(
        "Eve".to_string(),
        20,
        vec![
            "Physics".to_string(),
            "Anime".to_string(),
            "Chess".to_string(),
        ],
        "Dorm B".to_string(),
    )?;

    let frank = network.add_person(
        "Frank".to_string(),
        23,
        vec![
            "Computer Science".to_string(),
            "Photography".to_string(),
            "Basketball".to_string(),
        ],
        "Off-campus".to_string(),
    )?;

    println!("🎓 Added 6 university students");
    network.print_statistics()?;

    println!("\n🤝 Forming friendships...");

    // Friendships based on different factors
    println!("• Alice and Bob become friends (same dorm + CS major)");
    network.add_friendship(alice, bob, ConnectionType::Location)?;

    println!("• Alice and Charlie become friends (common gaming interest)");
    network.add_friendship(alice, charlie, ConnectionType::Interest)?;

    println!("• Charlie and Eve become friends (same dorm + both like chess)");
    network.add_friendship(charlie, eve, ConnectionType::Location)?;

    println!("• Diana and Frank meet through photography club");
    network.add_friendship(diana, frank, ConnectionType::Interest)?;

    println!("\n📊 After initial friendships:");
    network.print_statistics()?;
    network.print_communities()?;

    // Test friend suggestions
    println!("🔍 Friend suggestions for Alice:");
    let suggestions = network.suggest_friends(alice, 3)?;
    for (i, person) in suggestions.iter().enumerate() {
        println!(
            "   {}. {} (interests: {})",
            i + 1,
            person,
            person.interests.join(", ")
        );
    }

    // Connect the communities
    println!("\nBob introduces Alice to Eve at a campus event...");
    network.add_friendship(alice, eve, ConnectionType::Mutual)?;

    println!("Frank meets Charlie through mutual friend Alice...");
    network.add_friendship(frank, charlie, ConnectionType::Mutual)?;

    println!("\n📊 After community bridging:");
    network.print_statistics()?;
    network.print_communities()?;

    println!();
    Ok(())
}

/// Example: Professional networking
fn example_professional_network() -> Result<(), String> {
    println!("=== Professional Networking Example ===");

    let mut network = SocialNetwork::new(30);

    // Add professionals from different companies and roles
    let people_data = vec![
        (
            "Sarah",
            28,
            vec![
                "Software Engineering".to_string(),
                "Machine Learning".to_string(),
            ],
            "Tech Corp",
        ),
        (
            "Mike",
            32,
            vec!["Product Management".to_string(), "Startups".to_string()],
            "StartupX",
        ),
        (
            "Lisa",
            25,
            vec!["UI/UX Design".to_string(), "User Research".to_string()],
            "Design Studio",
        ),
        (
            "James",
            30,
            vec!["Software Engineering".to_string(), "Blockchain".to_string()],
            "Tech Corp",
        ),
        (
            "Anna",
            27,
            vec!["Data Science".to_string(), "Machine Learning".to_string()],
            "AI Labs",
        ),
        (
            "David",
            35,
            vec!["Product Management".to_string(), "Enterprise".to_string()],
            "Big Corp",
        ),
        (
            "Emma",
            26,
            vec!["UI/UX Design".to_string(), "Mobile Apps".to_string()],
            "StartupX",
        ),
        (
            "Ryan",
            29,
            vec!["Data Science".to_string(), "Analytics".to_string()],
            "Analytics Co",
        ),
    ];

    let mut people_ids = Vec::new();

    for (name, age, interests, company) in people_data {
        let id = network.add_person(name.to_string(), age, interests, company.to_string())?;
        people_ids.push(id);
    }

    println!("💼 Professional network with {} people", people_ids.len());

    // Form professional connections
    println!("\n🤝 Forming professional connections...");

    // Same company connections
    network.add_friendship(people_ids[0], people_ids[3], ConnectionType::Location)?; // Sarah-James (Tech Corp)
    network.add_friendship(people_ids[1], people_ids[6], ConnectionType::Location)?; // Mike-Emma (StartupX)

    // Same interest connections
    network.add_friendship(people_ids[0], people_ids[4], ConnectionType::Interest)?; // Sarah-Anna (ML)
    network.add_friendship(people_ids[2], people_ids[6], ConnectionType::Interest)?; // Lisa-Emma (Design)
    network.add_friendship(people_ids[4], people_ids[7], ConnectionType::Interest)?; // Anna-Ryan (Data Science)
    network.add_friendship(people_ids[1], people_ids[5], ConnectionType::Interest)?; // Mike-David (Product)

    // Conference/event connections
    network.add_friendship(people_ids[0], people_ids[2], ConnectionType::Online)?; // Sarah-Lisa (Tech conference)
    network.add_friendship(people_ids[3], people_ids[7], ConnectionType::Online)?; // James-Ryan (Meetup)

    println!("📊 Professional network statistics:");
    network.print_statistics()?;
    network.print_communities()?;

    // Demonstrate professional networking value
    println!("🔍 Professional opportunities through network:");

    // Check if people in different companies are connected
    println!(
        "• Can Sarah (Tech Corp) reach Emma (StartupX)? {}",
        network.are_friends(people_ids[0], people_ids[6])?
    );

    if network.are_friends(people_ids[0], people_ids[6])? {
        println!("  → Yes! Through their professional network connections");
        let sarah_circle = network.get_friend_circle(people_ids[0])?;
        println!(
            "  → Sarah's professional network size: {}",
            sarah_circle.len()
        );
    }

    // Friend suggestions for career growth
    println!("\n💡 Professional suggestions for Sarah:");
    let suggestions = network.suggest_friends(people_ids[0], 3)?;
    for person in suggestions {
        println!(
            "   • {} at {} (interests: {})",
            person.name,
            person.location,
            person.interests.join(", ")
        );
    }

    println!();
    Ok(())
}

/// Example: Community growth simulation
fn example_community_growth() -> Result<(), String> {
    println!("=== Community Growth Simulation ===");

    let mut network = SocialNetwork::new(100);

    // Start with a few early adopters
    println!("🌱 Starting with early adopters...");

    let mut people = Vec::new();
    let interests = [
        vec!["Technology".to_string(), "Startups".to_string()],
        vec!["Technology".to_string(), "Gaming".to_string()],
        vec!["Art".to_string(), "Photography".to_string()],
        vec!["Fitness".to_string(), "Nutrition".to_string()],
        vec!["Music".to_string(), "Concerts".to_string()],
    ];

    for (i, interest_list) in interests.iter().enumerate() {
        let id = network.add_person(
            format!("Person-{}", i + 1),
            25 + i as u32,
            interest_list.clone(),
            if i < 3 {
                "City A".to_string()
            } else {
                "City B".to_string()
            },
        )?;
        people.push(id);
    }

    // Initial small connections
    network.add_friendship(people[0], people[1], ConnectionType::Interest)?;
    network.add_friendship(people[2], people[3], ConnectionType::Location)?;

    println!("Phase 1 - Early adopters:");
    network.print_statistics()?;

    // Phase 2: Friends of friends join
    println!("\n🚀 Phase 2: Network effects kick in...");

    for i in 5..15 {
        let similar_interests = if i % 2 == 0 {
            vec!["Technology".to_string(), "Innovation".to_string()]
        } else {
            vec!["Art".to_string(), "Creative".to_string()]
        };

        let id = network.add_person(
            format!("Person-{}", i + 1),
            20 + (i % 10) as u32,
            similar_interests,
            format!("City {}", if i % 3 == 0 { "A" } else { "B" }),
        )?;

        // New people connect to existing network based on interests
        for existing_id in &people {
            if let (Some(new_person), Some(existing_person)) =
                (network.get_person(id), network.get_person(*existing_id))
            {
                let common_interests: Vec<_> = new_person
                    .interests
                    .iter()
                    .filter(|interest| existing_person.interests.contains(interest))
                    .collect();

                if !common_interests.is_empty() && rand::random::<f64>() < 0.4 {
                    network.add_friendship(id, *existing_id, ConnectionType::Interest)?;
                }
            }
        }

        people.push(id);
    }

    println!("Phase 2 - Network growth:");
    network.print_statistics()?;

    // Phase 3: Viral growth
    println!("\n📈 Phase 3: Viral growth phase...");

    for i in 15..30 {
        let id = network.add_person(
            format!("Person-{}", i + 1),
            18 + (i % 15) as u32,
            vec!["Social".to_string(), "Networking".to_string()],
            format!("City {}", char::from(65 + (i % 3) as u8)), // City A, B, C
        )?;

        // New joiners connect to multiple existing people
        let mut connections = 0;
        for existing_id in &people {
            if connections >= 3 {
                break;
            } // Limit connections per person

            if rand::random::<f64>() < 0.3 {
                network.add_friendship(id, *existing_id, ConnectionType::Online)?;
                connections += 1;
            }
        }

        people.push(id);
    }

    println!("Phase 3 - Viral growth:");
    network.print_statistics()?;

    // Analyze final network
    println!("\n📊 Final network analysis:");
    let communities = network.get_communities()?;

    if communities.len() == 1 {
        println!("🎉 Success! Everyone is connected in one large community");
        println!("   Community size: {}", communities[0].len());

        // Simulate information spread
        println!("\n🗞️  Testing information spread...");
        let infected = network.simulate_viral_spread(people[0], 0.6)?;
        println!(
            "   Information reached {:.1}% of the network",
            (infected.len() as f64 / people.len() as f64) * 100.0
        );
    } else {
        println!(
            "📊 Network formed {} separate communities",
            communities.len()
        );
        for (i, community) in communities.iter().enumerate() {
            println!("   Community {}: {} people", i + 1, community.len());
        }
    }

    println!();
    Ok(())
}

fn main() -> Result<(), String> {
    println!("👥 Social Network Analysis with Union-Find\n");

    example_university_network()?;
    example_professional_network()?;
    example_community_growth()?;

    println!("✅ All social network examples completed!");

    println!("\n💡 Key Benefits of Union-Find for Social Networks:");
    println!("• Efficient friend circle detection and community analysis");
    println!("• Fast mutual friend checking for recommendations");
    println!("• Scalable to millions of users with near-constant operations");
    println!("• Perfect for modeling permanent, bidirectional relationships");
    println!("• Enables real-time network analysis and viral spread simulation");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_friendship() {
        let mut network = SocialNetwork::new(10);

        let alice = network
            .add_person(
                "Alice".to_string(),
                25,
                vec!["CS".to_string()],
                "NYC".to_string(),
            )
            .unwrap();

        let bob = network
            .add_person(
                "Bob".to_string(),
                26,
                vec!["Math".to_string()],
                "NYC".to_string(),
            )
            .unwrap();

        // Initially not friends
        assert!(!network.are_friends(alice, bob).unwrap());
        assert_eq!(network.friend_circle_size(alice).unwrap(), 1);

        // Add friendship
        assert!(network
            .add_friendship(alice, bob, ConnectionType::Direct)
            .unwrap());

        // Now they are friends
        assert!(network.are_friends(alice, bob).unwrap());
        assert_eq!(network.friend_circle_size(alice).unwrap(), 2);
        assert_eq!(network.friend_circle_size(bob).unwrap(), 2);
    }

    #[test]
    fn test_friend_circles() {
        let mut network = SocialNetwork::new(10);

        let alice = network
            .add_person("Alice".to_string(), 25, vec![], "NYC".to_string())
            .unwrap();
        let bob = network
            .add_person("Bob".to_string(), 26, vec![], "NYC".to_string())
            .unwrap();
        let charlie = network
            .add_person("Charlie".to_string(), 27, vec![], "LA".to_string())
            .unwrap();

        // Create friendship: Alice <-> Bob
        network
            .add_friendship(alice, bob, ConnectionType::Direct)
            .unwrap();

        // Alice and Bob should be in same circle, Charlie separate
        let alice_circle = network.get_friend_circle(alice).unwrap();
        assert_eq!(alice_circle.len(), 2); // Alice and Bob

        let charlie_circle = network.get_friend_circle(charlie).unwrap();
        assert_eq!(charlie_circle.len(), 1); // Just Charlie

        // Connect Charlie to Alice
        network
            .add_friendship(alice, charlie, ConnectionType::Online)
            .unwrap();

        // Now all should be in same circle
        let alice_circle = network.get_friend_circle(alice).unwrap();
        assert_eq!(alice_circle.len(), 3); // Alice, Bob, and Charlie
    }

    #[test]
    fn test_communities() {
        let mut network = SocialNetwork::new(10);

        // Create two separate friend groups
        let group1_a = network
            .add_person("A".to_string(), 25, vec![], "NYC".to_string())
            .unwrap();
        let group1_b = network
            .add_person("B".to_string(), 26, vec![], "NYC".to_string())
            .unwrap();

        let group2_c = network
            .add_person("C".to_string(), 27, vec![], "LA".to_string())
            .unwrap();
        let group2_d = network
            .add_person("D".to_string(), 28, vec![], "LA".to_string())
            .unwrap();

        // Connect within groups
        network
            .add_friendship(group1_a, group1_b, ConnectionType::Location)
            .unwrap();
        network
            .add_friendship(group2_c, group2_d, ConnectionType::Location)
            .unwrap();

        let communities = network.get_communities().unwrap();

        // Should have 2 communities of size 2 each, plus isolated nodes
        assert!(communities.len() >= 2);

        let community_sizes: Vec<usize> = communities.iter().map(|c| c.len()).collect();
        assert!(community_sizes.contains(&2)); // Both groups should have size 2
    }

    #[test]
    fn test_duplicate_friendship() {
        let mut network = SocialNetwork::new(5);

        let alice = network
            .add_person("Alice".to_string(), 25, vec![], "NYC".to_string())
            .unwrap();
        let bob = network
            .add_person("Bob".to_string(), 26, vec![], "NYC".to_string())
            .unwrap();

        // First friendship should succeed
        assert!(network
            .add_friendship(alice, bob, ConnectionType::Direct)
            .unwrap());

        // Second friendship should return false (no change)
        assert!(!network
            .add_friendship(alice, bob, ConnectionType::Interest)
            .unwrap());
        assert!(!network
            .add_friendship(bob, alice, ConnectionType::Location)
            .unwrap());

        // Should still be friends
        assert!(network.are_friends(alice, bob).unwrap());
    }
}
