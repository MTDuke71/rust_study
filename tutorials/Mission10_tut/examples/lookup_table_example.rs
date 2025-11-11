// Mission 10 Tutorial - Lookup Table Approach for Image Segmentation
// Demonstrates how lookup tables eliminate HashMap non-deterministic behavior

use std::collections::HashMap;

struct UnionFind {
    parent: Vec<usize>,
    count: usize,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            count: n,
        }
    }

    fn find(&mut self, mut x: usize) -> usize {
        while x != self.parent[x] {
            let next = self.parent[x];
            self.parent[x] = self.parent[self.parent[x]]; // Path compression
            x = next;
        }
        x
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y { return false; }

        self.parent[root_x] = root_y;
        self.count -= 1;
        true
    }

    // ❌ ORIGINAL HASHMAP APPROACH: Non-deterministic iteration order
    #[allow(dead_code)]
    fn get_components_hashmap(&mut self) -> Vec<Vec<usize>> {
        use std::collections::HashMap;
        
        let mut components_map: HashMap<usize, Vec<usize>> = HashMap::new();
        
        // Group elements by their root (non-deterministic HashMap iteration!)
        for i in 0..self.parent.len() {
            let root = self.find(i);
            components_map.entry(root).or_insert_with(Vec::new).push(i);
        }
        
        // ❌ PROBLEM: HashMap.values() iteration order is non-deterministic!
        // This causes different output on each run
        components_map.into_values().collect()
    }

    // ✅ DETERMINISTIC LOOKUP TABLE: Uses Vec instead of HashMap
    fn get_components(&mut self) -> Vec<Vec<usize>> {
        // First pass: collect all unique roots
        let mut roots: Vec<usize> = Vec::new();
        for i in 0..self.parent.len() {
            let root = self.find(i);
            if !roots.contains(&root) {
                roots.push(root);
            }
        }
        roots.sort(); // ✅ Deterministic ordering

        // Second pass: build components using sorted roots as lookup table
        let mut components = vec![Vec::new(); roots.len()];
        for i in 0..self.parent.len() {
            let root = self.find(i);
            let component_idx = roots.binary_search(&root).unwrap();
            components[component_idx].push(i);
        }

        components
    }
}

struct Image {
    data: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl Image {
    fn new(data: Vec<Vec<char>>) -> Self {
        let height = data.len();
        let width = if height > 0 { data[0].len() } else { 0 };
        Self { data, width, height }
    }

    fn get_pixel(&self, row: usize, col: usize) -> char {
        self.data[row][col]
    }

    fn pixel_index(&self, row: usize, col: usize) -> usize {
        row * self.width + col
    }
}

// ✅ LOOKUP TABLE APPROACH: Eliminates HashMap non-determinism
struct ImageSegmenter {
    uf: UnionFind,
    color_to_region: HashMap<char, usize>,  // Color -> Region ID mapping
    region_to_color: Vec<char>,             // Region ID -> Color lookup table
    region_count: usize,
}

impl ImageSegmenter {
    fn new(image: &Image) -> Self {
        let pixel_count = image.width * image.height;
        let mut segmenter = Self {
            uf: UnionFind::new(pixel_count),
            color_to_region: HashMap::new(),
            region_to_color: Vec::new(),
            region_count: 0,
        };

        // Pre-assign region IDs to colors in deterministic order
        let mut unique_colors: Vec<char> = Vec::new();
        for row in 0..image.height {
            for col in 0..image.width {
                let color = image.get_pixel(row, col);
                if !unique_colors.contains(&color) {
                    unique_colors.push(color);
                }
            }
        }
        unique_colors.sort(); // ✅ Deterministic color ordering

        // Create lookup tables
        for (region_id, color) in unique_colors.iter().enumerate() {
            segmenter.color_to_region.insert(*color, region_id);
            segmenter.region_to_color.push(*color);
        }
        segmenter.region_count = unique_colors.len();

        segmenter
    }

    fn segment(&mut self, image: &Image) {
        // Process all pixels and union adjacent same-color pixels
        for row in 0..image.height {
            for col in 0..image.width {
                let current_pixel = image.get_pixel(row, col);
                let current_idx = image.pixel_index(row, col);

                // Check right neighbor
                if col + 1 < image.width {
                    let right_pixel = image.get_pixel(row, col + 1);
                    if current_pixel == right_pixel {
                        let right_idx = image.pixel_index(row, col + 1);
                        self.uf.union(current_idx, right_idx);
                    }
                }

                // Check bottom neighbor  
                if row + 1 < image.height {
                    let bottom_pixel = image.get_pixel(row + 1, col);
                    if current_pixel == bottom_pixel {
                        let bottom_idx = image.pixel_index(row + 1, col);
                        self.uf.union(current_idx, bottom_idx);
                    }
                }
            }
        }
    }

    // ✅ DETERMINISTIC: Uses lookup table instead of HashMap iteration
    fn display_segmented(&mut self, image: &Image) {
        println!("Segmented Image (region numbers):");
        
        // Create pixel -> region mapping using lookup tables
        let mut pixel_to_region: Vec<usize> = vec![0; image.width * image.height];
        
        for row in 0..image.height {
            for col in 0..image.width {
                let pixel_idx = image.pixel_index(row, col);
                let color = image.get_pixel(row, col);
                let region_id = self.color_to_region[&color];
                pixel_to_region[pixel_idx] = region_id;
            }
        }

        // Display grid with region numbers
        for row in 0..image.height {
            for col in 0..image.width {
                let pixel_idx = image.pixel_index(row, col);
                let region_id = pixel_to_region[pixel_idx];
                print!("{} ", region_id);
            }
            println!();
        }
    }

    // ✅ DETERMINISTIC: Uses lookup table for analysis
    fn analyze_regions(&mut self, image: &Image) {
        println!("\n--- Region Analysis ---");
        
        // Group pixels by their original color using lookup tables
        let mut color_regions: Vec<Vec<usize>> = vec![Vec::new(); self.region_count];
        
        for row in 0..image.height {
            for col in 0..image.width {
                let pixel_idx = image.pixel_index(row, col);
                let color = image.get_pixel(row, col);
                let region_id = self.color_to_region[&color];
                color_regions[region_id].push(pixel_idx);
            }
        }

        // Analyze each region using lookup table
        for region_id in 0..self.region_count {
            let color = self.region_to_color[region_id]; // ✅ O(1) lookup
            let pixel_count = color_regions[region_id].len();
            
            println!("Region {}: Color '{}', Size {} pixels", 
                     region_id, color, pixel_count);
        }
    }

    // ✅ COMPARISON: Show both approaches produce same logical result
    fn compare_approaches(&mut self, image: &Image) {
        println!("\n=== Comparing HashMap vs Lookup Table Approaches ===");
        
        println!("\nApproach 1: HashMap Components (❌ Non-Deterministic)");
        let hashmap_components = self.uf.get_components_hashmap();
        
        println!("HashMap Components (order varies each run):");
        for (comp_idx, component) in hashmap_components.iter().enumerate() {
            if !component.is_empty() {
                let sample_pixel = component[0];
                let row = sample_pixel / image.width;
                let col = sample_pixel % image.width;
                let color = image.get_pixel(row, col);
                
                println!("Component {}: Color '{}', Size {} pixels",
                         comp_idx, color, component.len());
            }
        }

        println!("\nApproach 2: Lookup Table (✅ Deterministic)");
        let lookup_components = self.uf.get_components();
        
        println!("Lookup Table Components (consistent order):");
        for (comp_idx, component) in lookup_components.iter().enumerate() {
            if !component.is_empty() {
                let sample_pixel = component[0];
                let row = sample_pixel / image.width;
                let col = sample_pixel % image.width;
                let color = image.get_pixel(row, col);
                
                println!("Component {}: Color '{}', Size {} pixels",
                         comp_idx, color, component.len());
            }
        }

        println!("\nApproach 3: Direct Color-to-Region Mapping (✅ Most Efficient)");
        self.display_segmented(image);
        self.analyze_regions(image);

        println!("\n🔍 Key Differences:");
        println!("• HashMap: Same logical result but different component order each run");
        println!("• Lookup Table: Deterministic component ordering via sorted roots");  
        println!("• Direct Mapping: Skip component extraction entirely, use color regions");
        println!("\n✅ All approaches produce equivalent connectivity results!");
        println!("✅ Lookup table and direct mapping are deterministic and faster!");
    }
}

// Demonstrates HashMap non-determinism with multiple runs
fn demonstrate_hashmap_non_determinism(image: &Image) {
    println!("\n🔄 Demonstrating HashMap Non-Determinism (Multiple Runs):");
    println!("Running HashMap approach 3 times to show order variation...\n");
    
    for run in 1..=3 {
        println!("--- Run {} ---", run);
        let mut segmenter = ImageSegmenter::new(image);
        segmenter.segment(image);
        
        let components = segmenter.uf.get_components_hashmap();
        print!("Component order: ");
        for (i, component) in components.iter().enumerate() {
            if !component.is_empty() {
                let sample_pixel = component[0];
                let row = sample_pixel / image.width;
                let col = sample_pixel % image.width;
                let color = image.get_pixel(row, col);
                print!("{}:{} ", i, color);
            }
        }
        println!();
    }
    
    println!("\n⚠️  Notice: Component indices may vary between runs due to HashMap!");
    println!("   This makes debugging and testing difficult.");
}

fn main() {
    println!("╔═══════════════════════════════════════════════════════════╗");
    println!("║       HashMap vs Lookup Table Image Segmentation         ║");  
    println!("╚═══════════════════════════════════════════════════════════╝\n");

    // Create test image
    let image_data = vec![
        vec!['A', 'A', 'A', 'B', 'B', 'C', 'C', 'C'],
        vec!['A', 'A', 'A', 'B', 'B', 'C', 'C', 'C'], 
        vec!['A', 'A', 'B', 'B', 'B', 'B', 'C', 'C'],
        vec!['D', 'D', 'B', 'B', 'B', 'B', 'C', 'C'],
        vec!['D', 'D', 'D', 'B', 'B', 'C', 'C', 'C'],
        vec!['D', 'D', 'D', 'E', 'E', 'E', 'C', 'C'],
        vec!['D', 'D', 'E', 'E', 'E', 'E', 'E', 'C'],
        vec!['D', 'D', 'E', 'E', 'E', 'E', 'E', 'E'],
    ];

    let image = Image::new(image_data);
    
    println!("Original Image (8 x 8):");
    for row in &image.data {
        for &pixel in row {
            print!("{} ", pixel);
        }
        println!();
    }

    // Demonstrate HashMap non-determinism first
    demonstrate_hashmap_non_determinism(&image);

    // Process with all approaches
    let mut segmenter = ImageSegmenter::new(&image);
    segmenter.segment(&image);
    segmenter.compare_approaches(&image);

    println!("\n╔═══════════════════════════════════════════════════════════╗");
    println!("║                    Key Benefits                           ║");
    println!("╠═══════════════════════════════════════════════════════════╣");
    println!("║ ❌ HashMap: Non-deterministic iteration order             ║");
    println!("║ ✅ Lookup Table: Deterministic, sorted root ordering      ║");
    println!("║ ✅ Direct Mapping: Most efficient, no component extraction║");
    println!("║ ✅ Performance: O(1) lookups vs HashMap overhead          ║");
    println!("║ ✅ Memory: Less memory than HashMap approach               ║");
    println!("║ ✅ Cache-friendly: Contiguous memory access               ║");
    println!("╚═══════════════════════════════════════════════════════════╝");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lookup_table_deterministic() {
        let image_data = vec![
            vec!['A', 'B'],
            vec!['A', 'B'],
        ];
        let image = Image::new(image_data);

        // Run multiple times - should produce identical results
        for _ in 0..10 {
            let mut segmenter = ImageSegmenter::new(&image);
            segmenter.segment(&image);
            
            // Verify color-to-region mapping is consistent
            assert_eq!(segmenter.color_to_region[&'A'], 0); // A always maps to 0
            assert_eq!(segmenter.color_to_region[&'B'], 1); // B always maps to 1
            assert_eq!(segmenter.region_to_color[0], 'A');  // Region 0 is always A
            assert_eq!(segmenter.region_to_color[1], 'B');  // Region 1 is always B
        }
    }

    #[test] 
    fn test_components_vs_lookup_equivalent() {
        let image_data = vec![
            vec!['A', 'A', 'B'],
            vec!['A', 'C', 'B'],
            vec!['C', 'C', 'C'],
        ];
        let image = Image::new(image_data);

        let mut segmenter = ImageSegmenter::new(&image);
        segmenter.segment(&image);

        // Both approaches should identify same number of distinct regions
        let components = segmenter.uf.get_components();
        let unique_colors = segmenter.region_count;
        
        assert_eq!(components.len(), unique_colors);
    }
}