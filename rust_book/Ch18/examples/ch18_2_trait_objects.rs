//! Chapter 18.2: Using Trait Objects for Values of Different Types
//!
//! This example demonstrates:
//! - Defining traits for common behavior
//! - Using Box<dyn Trait> for trait objects
//! - Dynamic dispatch with heterogeneous collections
//! - Trade-offs between trait objects and generics

/// The Draw trait defines common behavior for UI components
pub trait Draw {
    fn draw(&self);
}

/// Button component with label
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!(
            "┌{}┐",
            "─".repeat(self.width as usize)
        );
        println!(
            "│{:^width$}│",
            self.label,
            width = self.width as usize
        );
        println!(
            "└{}┘",
            "─".repeat(self.width as usize)
        );
    }
}

/// SelectBox component with multiple options
pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!(
            "╔{}╗",
            "═".repeat(self.width as usize)
        );
        
        for (i, option) in self.options.iter().enumerate() {
            if i < self.height as usize {
                println!(
                    "║{:<width$}║",
                    option,
                    width = self.width as usize
                );
            }
        }
        
        // Fill remaining height
        for _ in self.options.len()..(self.height as usize) {
            println!(
                "║{:<width$}║",
                "",
                width = self.width as usize
            );
        }
        
        println!(
            "╚{}╝",
            "═".repeat(self.width as usize)
        );
    }
}

/// TextField component for text input
pub struct TextField {
    pub width: u32,
    pub placeholder: String,
    pub value: String,
}

impl Draw for TextField {
    fn draw(&self) {
        println!(
            "[{}]",
            "─".repeat(self.width as usize)
        );
        
        let display_text = if self.value.is_empty() {
            &self.placeholder
        } else {
            &self.value
        };
        
        println!(
            " {:<width$} ",
            display_text,
            width = self.width as usize
        );
        
        println!(
            "[{}]",
            "─".repeat(self.width as usize)
        );
    }
}

/// Screen holds heterogeneous components using trait objects
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn new() -> Self {
        Screen {
            components: vec![],
        }
    }

    pub fn add_component(&mut self, component: Box<dyn Draw>) {
        self.components.push(component);
    }

    pub fn run(&self) {
        println!("\n╔════════════════════════════════════╗");
        println!("║         Screen Rendering           ║");
        println!("╚════════════════════════════════════╝\n");
        
        for (i, component) in self.components.iter().enumerate() {
            println!("Component {}:", i + 1);
            component.draw();
            println!();
        }
    }
}

impl Default for Screen {
    fn default() -> Self {
        Self::new()
    }
}

// Demonstrating the alternative with generics (compile-time)
pub struct ScreenGeneric<T: Draw> {
    pub components: Vec<T>,
}

impl<T: Draw> Default for ScreenGeneric<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Draw> ScreenGeneric<T> {
    pub fn new() -> Self {
        ScreenGeneric {
            components: vec![],
        }
    }

    pub fn add_component(&mut self, component: T) {
        self.components.push(component);
    }

    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

fn main() {
    println!("=== Chapter 18.2: Trait Objects ===");

    // 1. Creating a heterogeneous screen with trait objects
    println!("1. Trait Objects - Heterogeneous Collections:");
    let mut screen = Screen::new();

    screen.add_component(Box::new(Button {
        width: 20,
        height: 3,
        label: String::from("OK"),
    }));

    screen.add_component(Box::new(SelectBox {
        width: 20,
        height: 4,
        options: vec![
            String::from("Option 1"),
            String::from("Option 2"),
            String::from("Option 3"),
        ],
    }));

    screen.add_component(Box::new(TextField {
        width: 20,
        placeholder: String::from("Enter name..."),
        value: String::from(""),
    }));

    screen.add_component(Box::new(Button {
        width: 20,
        height: 3,
        label: String::from("Cancel"),
    }));

    screen.run();

    // 2. Demonstrating why generics don't work for heterogeneous collections
    println!("\n2. Generic Limitation:");
    println!("   Generic Screen<T> can only hold ONE type:");
    
    let mut button_screen = ScreenGeneric::new();
    button_screen.add_component(Button {
        width: 15,
        height: 2,
        label: String::from("Button 1"),
    });
    button_screen.add_component(Button {
        width: 15,
        height: 2,
        label: String::from("Button 2"),
    });
    
    println!("   ✓ All buttons - compiles fine");
    println!("   ✗ Mix of Button and SelectBox - won't compile");
    
    // This would NOT compile:
    // button_screen.add_component(SelectBox { ... });

    // 3. Trait object syntax variations
    println!("\n3. Trait Object Syntax:");
    println!("   Box<dyn Draw>     - Owned trait object");
    println!("   &dyn Draw         - Borrowed trait object");
    println!("   &mut dyn Draw     - Mutable borrowed trait object");
    println!("   Rc<dyn Draw>      - Reference-counted trait object");
    println!("   Arc<dyn Draw>     - Thread-safe reference-counted");

    // 4. Dynamic vs Static dispatch
    println!("\n4. Dispatch Comparison:");
    println!("   Dynamic Dispatch (Trait Objects):");
    println!("   ✓ Heterogeneous collections");
    println!("   ✓ Runtime polymorphism");
    println!("   ✗ Small runtime cost (vtable lookup)");
    println!("   ✗ No compiler optimizations like inlining");
    
    println!("\n   Static Dispatch (Generics):");
    println!("   ✓ Zero-cost abstraction");
    println!("   ✓ Compiler can inline and optimize");
    println!("   ✗ Only one type per collection");
    println!("   ✗ Code duplication (monomorphization)");

    // 5. Object safety requirements
    println!("\n5. Object Safety:");
    println!("   A trait is object-safe if:");
    println!("   ✓ No generic methods");
    println!("   ✓ No Self in return types (except receiver)");
    println!("   ✓ No associated const or types (with exceptions)");
    
    println!("\n   Draw trait is object-safe because:");
    println!("   ✓ draw(&self) - no generics");
    println!("   ✓ Returns () - no Self");

    // 6. Real-world use cases
    println!("\n6. When to Use Trait Objects:");
    println!("   ✓ GUI systems (different widget types)");
    println!("   ✓ Plugin systems (unknown types at compile-time)");
    println!("   ✓ Game entities (heterogeneous behaviors)");
    println!("   ✓ Middleware chains (different processors)");
    println!("   ✓ Strategy pattern implementations");

    println!("\n7. Performance Considerations:");
    let components: Vec<Box<dyn Draw>> = vec![
        Box::new(Button {
            width: 10,
            height: 2,
            label: String::from("Test"),
        }),
    ];
    
    println!("   Trait object size: {} bytes", std::mem::size_of_val(&components[0]));
    println!("   (Pointer to data + pointer to vtable)");
    println!("   Cost: One extra indirection per method call");

    println!("\n=== End of Chapter 18.2 ===");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heterogeneous_screen() {
        let mut screen = Screen::new();
        
        screen.add_component(Box::new(Button {
            width: 10,
            height: 2,
            label: String::from("OK"),
        }));
        
        screen.add_component(Box::new(SelectBox {
            width: 10,
            height: 3,
            options: vec![String::from("Yes"), String::from("No")],
        }));
        
        assert_eq!(screen.components.len(), 2);
    }

    #[test]
    fn test_generic_screen_single_type() {
        let mut screen = ScreenGeneric::new();
        
        screen.add_component(Button {
            width: 10,
            height: 2,
            label: String::from("Button 1"),
        });
        
        screen.add_component(Button {
            width: 10,
            height: 2,
            label: String::from("Button 2"),
        });
        
        assert_eq!(screen.components.len(), 2);
    }

    #[test]
    fn test_button_implements_draw() {
        let button = Button {
            width: 10,
            height: 2,
            label: String::from("Test"),
        };
        
        // If this compiles, Button implements Draw
        let _drawable: &dyn Draw = &button;
    }

    #[test]
    fn test_trait_object_as_function_parameter() {
        fn render(component: &dyn Draw) {
            component.draw();
        }
        
        let button = Button {
            width: 5,
            height: 1,
            label: String::from("Hi"),
        };
        
        render(&button);
    }

    #[test]
    fn test_boxed_trait_objects() {
        let components: Vec<Box<dyn Draw>> = vec![
            Box::new(Button {
                width: 10,
                height: 2,
                label: String::from("OK"),
            }),
            Box::new(SelectBox {
                width: 10,
                height: 2,
                options: vec![String::from("A")],
            }),
            Box::new(TextField {
                width: 10,
                placeholder: String::from("Name"),
                value: String::from(""),
            }),
        ];
        
        assert_eq!(components.len(), 3);
    }
}
