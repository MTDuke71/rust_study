# glam - Game Linear Algebra Mathematics

**Tags:** #rust-crate #linear-algebra #game-development #graphics #SIMD #performance #mathematics #vectors #matrices #quaternions  
**Created:** 2025-12-08  
**Related:** [[rust-performance-optimization]], [[computational-geometry]], [[SIMD-optimization]], [[bevy-game-engine]]

---

## 🎯 Core Concept

**glam** is a Rust crate providing fast, simple linear algebra for games and graphics. It focuses on **SIMD-optimized** vector and matrix operations with zero-cost abstractions.

**Key Philosophy**: Game math should be fast, simple, and compile-time optimized.

## 📦 Crate Information

- **Crate Name**: `glam`
- **Current Version**: 0.29+ (actively maintained)
- **Repository**: [bitshifter/glam-rs](https://github.com/bitshifter/glam-rs)
- **Documentation**: [docs.rs/glam](https://docs.rs/glam)
- **License**: MIT/Apache-2.0

```toml
[dependencies]
glam = "0.29"
```

## 🧮 Core Data Types

### **Vectors**

| Type | Description | Size | Use Case |
|------|-------------|------|----------|
| `Vec2` | 2D vector (f32) | 8 bytes | 2D positions, UI coordinates |
| `Vec3` | 3D vector (f32) | 12 bytes | 3D positions, directions, colors (RGB) |
| `Vec3A` | 3D SIMD-aligned | 16 bytes | Performance-critical 3D math |
| `Vec4` | 4D vector (f32) | 16 bytes | Homogeneous coords, colors (RGBA) |
| `DVec2/3/4` | Double precision | 2x size | Scientific computing |
| `IVec2/3/4` | Integer vectors (i32) | Varies | Grid coordinates, indices |
| `UVec2/3/4` | Unsigned integers (u32) | Varies | Array indices, counts |

### **Matrices**

| Type | Description | Size | Use Case |
|------|-------------|------|----------|
| `Mat2` | 2×2 matrix (f32) | 16 bytes | 2D rotations, scaling |
| `Mat3` | 3×3 matrix (f32) | 36 bytes | 2D transforms (with translation) |
| `Mat3A` | 3×3 SIMD-aligned | 48 bytes | Performance 2D/3D rotations |
| `Mat4` | 4×4 matrix (f32) | 64 bytes | 3D transforms, projections |
| `DMat2/3/4` | Double precision | 2x size | High-precision transforms |

### **Quaternions**

| Type | Description | Use Case |
|------|-------------|----------|
| `Quat` | Unit quaternion (f32) | 3D rotations (no gimbal lock) |
| `DQuat` | Double precision | High-precision rotations |

### **Transforms**

| Type | Description | Use Case |
|------|-------------|----------|
| `Affine2` | 2D affine transform | 2D games, UI |
| `Affine3A` | 3D affine transform | 3D games, physics |

## ⚡ Performance Features

### **1. SIMD Optimization**

Uses CPU vector instructions for parallel computation:

```rust
// This single operation computes 4 multiplications in parallel
let a = Vec4::new(1.0, 2.0, 3.0, 4.0);
let b = Vec4::new(5.0, 6.0, 7.0, 8.0);
let result = a * b; // Uses SSE/AVX SIMD instructions
```

**Supported SIMD Targets**:
- **SSE2**: x86/x64 (default)
- **SSE3/SSE4.1**: x86/x64 (with feature flags)
- **AVX/AVX2**: x86/x64 (with feature flags)
- **NEON**: ARM (Aarch64)
- **WASM SIMD**: WebAssembly

### **2. Zero-Cost Abstractions**

```rust
// This compiles to optimal SIMD assembly
let position = Vec3::new(1.0, 2.0, 3.0);
let velocity = Vec3::new(0.1, 0.2, 0.3);
let new_pos = position + velocity * 0.016; // Single SIMD instruction
```

### **3. Compile-Time Optimizations**

- Aggressive inlining
- Const evaluation where possible
- No heap allocations
- Optimal register usage

## 🎮 Common Operations

### **Vector Operations**

```rust
use glam::*;

// Construction
let v1 = Vec3::new(1.0, 2.0, 3.0);
let v2 = Vec3::ZERO; // (0, 0, 0)
let v3 = Vec3::ONE;  // (1, 1, 1)
let v4 = Vec3::X;    // (1, 0, 0)

// Arithmetic
let sum = v1 + v2;
let diff = v1 - v2;
let scaled = v1 * 2.0;
let component_mul = v1 * v2; // (x1*x2, y1*y2, z1*z2)

// Geometric operations
let length = v1.length();
let length_sq = v1.length_squared(); // Faster (no sqrt)
let normalized = v1.normalize();
let dot = v1.dot(v2);
let cross = v1.cross(v2);
let distance = v1.distance(v2);

// Interpolation
let lerp = v1.lerp(v2, 0.5); // Linear interpolation
let slerp = q1.slerp(q2, 0.5); // Spherical interpolation (quaternions)

// Clamping and comparison
let clamped = v1.clamp(Vec3::ZERO, Vec3::ONE);
let min = v1.min(v2);
let max = v1.max(v2);
```

### **Matrix Operations**

```rust
use glam::*;

// Construction
let identity = Mat4::IDENTITY;
let translation = Mat4::from_translation(Vec3::new(1.0, 2.0, 3.0));
let rotation = Mat4::from_rotation_y(std::f32::consts::PI / 2.0);
let scale = Mat4::from_scale(Vec3::new(2.0, 2.0, 2.0));

// Combining transforms (right to left: TRS order)
let model = translation * rotation * scale;

// Transform vectors
let point = Vec3::new(1.0, 0.0, 0.0);
let transformed = model.transform_point3(point);
let direction = Vec3::new(0.0, 1.0, 0.0);
let rotated_dir = model.transform_vector3(direction);

// Camera transforms
let view = Mat4::look_at_rh(
    Vec3::new(0.0, 5.0, 10.0), // eye position
    Vec3::ZERO,                 // target
    Vec3::Y,                    // up vector
);

let projection = Mat4::perspective_rh(
    std::f32::consts::FRAC_PI_2, // fov (90 degrees)
    16.0 / 9.0,                   // aspect ratio
    0.1,                          // near plane
    100.0,                        // far plane
);

// Inverse and transpose
let inverse = model.inverse();
let transpose = model.transpose();
```

### **Quaternion Operations**

```rust
use glam::*;

// Construction
let q1 = Quat::IDENTITY;
let q2 = Quat::from_rotation_y(std::f32::consts::PI / 2.0); // 90° around Y
let q3 = Quat::from_axis_angle(Vec3::Y, std::f32::consts::PI / 4.0);

// Euler angles (careful: gimbal lock possible)
let from_euler = Quat::from_euler(EulerRot::XYZ, pitch, yaw, roll);
let (x, y, z) = q1.to_euler(EulerRot::XYZ);

// Rotation operations
let rotated_vec = q2 * Vec3::X; // Rotate vector by quaternion
let combined = q1 * q2;         // Combine rotations

// Interpolation (smooth rotation)
let interpolated = q1.slerp(q2, 0.5);

// Convert to matrix
let rotation_matrix = Mat4::from_quat(q2);
```

## 🌍 Real-World Applications

### **1. Game Development**

```rust
// Player entity transform
struct Transform {
    position: Vec3,
    rotation: Quat,
    scale: Vec3,
}

impl Transform {
    fn to_matrix(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(
            self.scale,
            self.rotation,
            self.position,
        )
    }
    
    fn look_at(&mut self, target: Vec3, up: Vec3) {
        let direction = (target - self.position).normalize();
        self.rotation = Quat::from_rotation_arc(Vec3::Z, direction);
    }
}

// Physics update
fn update_physics(transform: &mut Transform, velocity: Vec3, delta_time: f32) {
    transform.position += velocity * delta_time;
}
```

### **2. Camera Systems**

```rust
struct Camera {
    position: Vec3,
    target: Vec3,
    up: Vec3,
    fov: f32,
    aspect_ratio: f32,
    near: f32,
    far: f32,
}

impl Camera {
    fn view_matrix(&self) -> Mat4 {
        Mat4::look_at_rh(self.position, self.target, self.up)
    }
    
    fn projection_matrix(&self) -> Mat4 {
        Mat4::perspective_rh(self.fov, self.aspect_ratio, self.near, self.far)
    }
    
    fn view_projection(&self) -> Mat4 {
        self.projection_matrix() * self.view_matrix()
    }
}
```

### **3. Physics Simulations**

```rust
struct RigidBody {
    position: Vec3,
    velocity: Vec3,
    acceleration: Vec3,
    mass: f32,
}

impl RigidBody {
    fn apply_force(&mut self, force: Vec3) {
        self.acceleration += force / self.mass;
    }
    
    fn update(&mut self, delta_time: f32) {
        self.velocity += self.acceleration * delta_time;
        self.position += self.velocity * delta_time;
        self.acceleration = Vec3::ZERO; // Reset forces
    }
    
    fn handle_collision(&mut self, normal: Vec3, restitution: f32) {
        // Reflect velocity along normal
        let vel_along_normal = self.velocity.dot(normal);
        if vel_along_normal < 0.0 {
            self.velocity -= (1.0 + restitution) * vel_along_normal * normal;
        }
    }
}
```

### **4. Ray Tracing / Raycasting**

```rust
struct Ray {
    origin: Vec3,
    direction: Vec3, // Should be normalized
}

impl Ray {
    fn at(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }
    
    fn intersect_sphere(&self, center: Vec3, radius: f32) -> Option<f32> {
        let oc = self.origin - center;
        let a = self.direction.length_squared();
        let half_b = oc.dot(self.direction);
        let c = oc.length_squared() - radius * radius;
        let discriminant = half_b * half_b - a * c;
        
        if discriminant < 0.0 {
            None
        } else {
            Some((-half_b - discriminant.sqrt()) / a)
        }
    }
    
    fn intersect_plane(&self, point: Vec3, normal: Vec3) -> Option<f32> {
        let denom = normal.dot(self.direction);
        if denom.abs() > 1e-6 {
            let t = (point - self.origin).dot(normal) / denom;
            if t >= 0.0 { Some(t) } else { None }
        } else {
            None
        }
    }
}
```

### **5. Advent of Code Applications**

```rust
// 3D coordinate problems (e.g., 2021 Day 19 - Scanner rotations)
fn rotate_point_around_axes(point: IVec3, rotation_index: usize) -> IVec3 {
    let v = Vec3::new(point.x as f32, point.y as f32, point.z as f32);
    let quat = ROTATION_QUATERNIONS[rotation_index];
    let rotated = quat * v;
    IVec3::new(rotated.x.round() as i32, rotated.y.round() as i32, rotated.z.round() as i32)
}

// 2D transformations (grid rotations, reflections)
fn rotate_90_degrees(point: Vec2) -> Vec2 {
    let rotation = Mat2::from_angle(std::f32::consts::FRAC_PI_2);
    rotation * point
}

// Distance calculations (Manhattan for grid, Euclidean for geometric)
fn manhattan_distance(a: IVec3, b: IVec3) -> i32 {
    (a - b).abs().element_sum()
}

fn euclidean_distance(a: Vec3, b: Vec3) -> f32 {
    a.distance(b)
}
```

## 🆚 Comparison with Alternatives

| Feature | glam | nalgebra | cgmath | ultraviolet |
|---------|------|----------|--------|-------------|
| **Focus** | Game math | General LA | Graphics | Wide SIMD |
| **SIMD** | ✅ SSE2/AVX | ⚠️ Optional | ❌ No | ✅ AVX2/512 |
| **Size** | Small | Large | Medium | Small |
| **Features** | Core math | Everything | Graphics | Core math |
| **Perf** | Excellent | Good | Moderate | Excellent |
| **Compile** | Fast | Slow | Fast | Fast |
| **Learning** | Easy | Complex | Moderate | Moderate |

### **When to Use Each**

**Use glam when:**
- Building games or real-time graphics
- Need maximum SIMD performance
- Want small binary size and fast compile times
- Working with Bevy or similar engines
- Doing Advent of Code with geometric problems

**Use nalgebra when:**
- Scientific computing or linear algebra research
- Need matrix decompositions (SVD, QR, Cholesky)
- Generic dimensions (compile-time variable size)
- Complex linear algebra operations

**Use cgmath when:**
- Legacy projects already using it
- Need stable API (less frequent updates)

**Use ultraviolet when:**
- Targeting AVX2/AVX-512 specifically
- Need cutting-edge SIMD performance
- Willing to accept less ecosystem support

## 🔧 Advanced Features

### **Feature Flags**

```toml
[dependencies]
glam = { version = "0.29", features = ["serde", "bytemuck", "mint"] }
```

| Feature | Description |
|---------|-------------|
| `serde` | Serialization support |
| `bytemuck` | Safe transmutation to bytes |
| `mint` | Math type interoperability |
| `rand` | Random number generation |
| `approx` | Approximate equality testing |
| `libm` | Pure Rust math (no_std) |
| `scalar-math` | Disable SIMD (debugging) |

### **SIMD Width Selection**

```rust
// Explicitly choose SIMD width at compile time
// Via cargo feature flags:
// --features "glam/sse2"    (default)
// --features "glam/sse3"
// --features "glam/avx"
// --features "glam/avx2"
```

### **Const Generics Support**

```rust
// Some operations are const-evaluable
const ZERO: Vec3 = Vec3::ZERO;
const IDENTITY: Mat4 = Mat4::IDENTITY;
```

## 🧪 Testing and Debugging

### **Approximate Equality**

```rust
use glam::*;

let a = Vec3::new(1.0, 2.0, 3.0);
let b = Vec3::new(1.0000001, 2.0, 3.0);

// Exact comparison (usually false due to float precision)
assert_eq!(a, b); // Fails!

// Use abs_diff_eq with epsilon
assert!(a.abs_diff_eq(b, 1e-6));

// Or with approx feature flag
#[cfg(feature = "approx")]
use approx::assert_relative_eq;
assert_relative_eq!(a, b, epsilon = 1e-6);
```

### **Debugging Transforms**

```rust
// Print matrices in readable format
println!("{:?}", matrix);

// Decompose transforms for inspection
let (scale, rotation, translation) = matrix.to_scale_rotation_translation();
println!("Scale: {:?}, Rotation: {:?}, Translation: {:?}", 
         scale, rotation, translation);

// Validate quaternions
assert!(quat.is_normalized());
```

## 📊 Performance Benchmarks

Typical performance vs scalar (non-SIMD) implementations:

| Operation | Speedup | Notes |
|-----------|---------|-------|
| Vec3 dot product | ~3x | SSE2 |
| Mat4 multiplication | ~4x | SSE2 |
| Quat * Vec3 | ~3x | SSE2 |
| Vec4 normalize | ~4x | SSE2 with rsqrt |

**Memory Layout Efficiency**:
- `Vec3A`: 16-byte aligned (faster than `Vec3` for heavy computation)
- `Mat4`: Cache-friendly column-major layout
- Quaternions: 16 bytes (same as Vec4)

## 🌟 Integration with Bevy

**Bevy** (popular Rust game engine) uses `glam` as its math library:

```rust
use bevy::prelude::*;

fn move_camera(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Camera>>,
) {
    for mut transform in &mut query {
        // glam types are used directly
        transform.translation.x += time.delta_seconds();
        transform.rotation *= Quat::from_rotation_y(0.01);
    }
}
```

## 🔗 Related Concepts

### **Mathematical Foundations**
- **[[linear-algebra]]**: Vector spaces, matrix theory
- **[[quaternions]]**: Rotation representation without gimbal lock
- **[[affine-transformations]]**: Translation, rotation, scaling, shearing
- **[[homogeneous-coordinates]]**: 4D representation for 3D transforms

### **Graphics Programming**
- **[[projection-matrices]]**: Perspective and orthographic projections
- **[[view-transformations]]**: Camera positioning and orientation
- **[[model-transformations]]**: Object placement in world space
- **[[normal-transformations]]**: Proper handling of surface normals

### **Game Development**
- **[[bevy-game-engine]]**: Uses glam as math library
- **[[entity-component-system]]**: Transform components
- **[[physics-engines]]**: Rigid body dynamics
- **[[collision-detection]]**: Geometric primitives

### **Performance Optimization**
- **[[SIMD-optimization]]**: Parallel computation techniques
- **[[cache-optimization]]**: Data layout for performance
- **[[compile-time-optimization]]**: Const evaluation and inlining

## 📚 Learning Resources

### **Official Documentation**
- [docs.rs/glam](https://docs.rs/glam) - API reference
- [GitHub Repository](https://github.com/bitshifter/glam-rs) - Source code and examples
- [CHANGELOG](https://github.com/bitshifter/glam-rs/blob/main/CHANGELOG.md) - Version history

### **Tutorials and Guides**
- **3D Math Primer**: Understanding vectors, matrices, quaternions
- **Game Programming Patterns**: Using transforms in game loops
- **Graphics Programming**: View and projection matrices
- **Physics Simulations**: Integrating forces and velocities

### **Books**
- *3D Math Primer for Graphics and Game Development* by Fletcher Dunn
- *Real-Time Rendering* by Tomas Akenine-Möller
- *Game Physics Engine Development* by Ian Millington

## 🎓 Common Pitfalls

### **1. Quaternion Multiplication Order**

```rust
// ❌ WRONG: Reversed order
let combined = rotation2 * rotation1; // Applies rotation1 THEN rotation2

// ✅ CORRECT: Left-to-right application
let combined = rotation1 * rotation2; // Applies rotation1 THEN rotation2
// Actually, this is context-dependent - verify with your use case!
```

### **2. Matrix Multiplication Order**

```rust
// ❌ WRONG: Incorrect TRS order
let model = scale * rotation * translation; // Wrong!

// ✅ CORRECT: Translation * Rotation * Scale
let model = translation * rotation * scale;
// Applies scale first, then rotation, then translation
```

### **3. Normalizing Zero Vectors**

```rust
// ❌ WRONG: Normalize without checking
let v = Vec3::ZERO;
let normalized = v.normalize(); // NaN!

// ✅ CORRECT: Check or use try_normalize
if v.length_squared() > 1e-6 {
    let normalized = v.normalize();
} else {
    // Handle zero vector case
}

// Or use try_normalize (returns Option)
if let Some(normalized) = v.try_normalize() {
    // Use normalized vector
}
```

### **4. Gimbal Lock with Euler Angles**

```rust
// ⚠️ CAUTION: Euler angles can cause gimbal lock
let quat = Quat::from_euler(EulerRot::XYZ, x, y, z);

// ✅ BETTER: Use quaternions directly
let quat = Quat::from_axis_angle(axis, angle);
```

### **5. Float Precision Issues**

```rust
// ❌ WRONG: Exact equality on floats
if position == target {
    // Rarely true due to float precision
}

// ✅ CORRECT: Use distance threshold
if position.distance(target) < 0.001 {
    // Close enough
}
```

## 🚀 Future Directions

- **Wider SIMD support**: AVX-512, ARM SVE
- **GPU compute integration**: SPIR-V, WGSL compatibility
- **Const generics**: More compile-time evaluation
- **Generic dimensions**: Compile-time variable sizes
- **Double precision improvements**: Better f64 SIMD

---

## *Links:*

**Core Concepts:** [[linear-algebra]] | [[quaternions]] | [[SIMD-optimization]] | [[affine-transformations]]

**Applications:** [[game-development]] | [[graphics-programming]] | [[physics-simulation]] | [[ray-tracing]]

**Related Crates:** [[bevy-game-engine]] | [[nalgebra]] | [[cgmath]] | [[ultraviolet]]

**Rust Concepts:** [[zero-cost-abstractions]] | [[const-generics]] | [[trait-implementations]] | [[performance-optimization]]

**Mathematics:** [[vector-operations]] | [[matrix-multiplication]] | [[rotation-representations]] | [[projection-matrices]]

**Advent of Code:** [[computational-geometry]] | [[3d-coordinate-systems]] | [[geometric-transformations]]

---

*glam represents the sweet spot between performance and simplicity for game and graphics mathematics in Rust. Its SIMD-optimized operations, zero-cost abstractions, and tight integration with the Bevy ecosystem make it the de facto standard for real-time mathematical computation in Rust game development.*
