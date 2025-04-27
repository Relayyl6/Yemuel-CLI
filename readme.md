<!-- ### **Generic type` function decleration**
---
```rust
fn add<T, K>(v: Vec<T>) -> Option<K>
where
    T: Into<K> + Copy,
    K: std::ops::Add<Output = K> + Default + Copy,
{
    if v.is_empty() {
        return None;
    }

    let mut result: K = K::default();

    for &i in &v {
        result = result + i.into();
    }

    Some(result)
}
```

### Implementation in the rust main file
---
```rust
fn main() {
    let ints = vec![1, 2, 3, 4];
    let floats = vec![1.5f32, 2.5, 3.0];

    let sum_as_f64: Option<f64> = add::<i32, f64>(ints);   // i32 -> f64
    let sum_f32: Option<f32> = add(floats);                // f32 -> f32

    println!("Sum as f64: {:?}", sum_as_f64); // Some(10.0)
    println!("Sum as f32: {:?}", sum_f32);    // Some(7.0)
}
``` -->



# 🧮 Rust CLI Calculator

A blazing-fast, type-safe command-line calculator with **CGPA support**, random number generation, and formatted output. Built with Rust’s safety guarantees.

[![Built with Rust](https://img.shields.io/badge/Rust-000000?logo=rust&style=for-the-badge)](https://www.rust-lang.org/)
[![CGPA Feature](https://img.shields.io/badge/Feature-CGPA_Calculator-blueviolet?style=for-the-badge)](https://en.wikipedia.org/wiki/CGPA)
[![Crates.io](https://img.shields.io/crates/v/calc-rs?style=for-the-badge)](https://crates.io/crates/calc-rs)

## Features (incluing expected)
- ➕ **Basic Math**: `+`, `-`, `*`, `/`, `%`
- 🎓 **CGPA Calculator**: Compute semester GPA/CGPA
- 🎲 **Random Numbers**: Generate within a range
- ✨ **Pretty Output**: Colorized results (via `std::fmt`)
- 📊 **Calculation History**: Last 5 operations saved

## 📦 Dependencies
```toml
[dependencies]
rand = "0.8"    # Random number generation
<!-- clap = "4.0"    # CLI argument parsing -->
<!-- tabled = "0.11" # Pretty GPA table formatting -->
```

## 🛠️ Installation
```bash
cargo install calc-rs  # Install from crates.io
# OR
git clone https://github.com/Relayyl6/Yemuel-CLI.git
cd Yemuel-CLI && cargo build --release
```

## 🖥️ Usage
### Basic Operations
```bash
calc add 5 3           # 5 + 3 = 8
calc divide 10 2       # 10 ÷ 2 = 5.0
```

### 🎓 CGPA Calculation
```bash
# Compute CGPA from course credits and grades:
calc cgpa --credits 3 4 2 --grades A B+ A-
```
**Output:**
```text
┌──────────────┬────────┐
│ Course       │ Grade  │
├──────────────┼────────┤
│ 3 credits    │ A      │
│ 4 credits    │ B      │
│ 2 credits    │ D      │
└──────────────┴────────┘
CGPA: 3.54/4.0
```

### 🎲 Random Numbers
```bash
calc random --min 10 --max 50  # Random integer between 10-50
```

## 🏗️ Code Structure
```text
src/
├── cgpa.rs            # CGPA logic (grade mapping, weighted average)
├── operation.rs       # Root arithmetic operations
├── shunting_yard.rs   # Implementation of shunting yard algorithm. Prioritizing PEMDAS
├── shunting_yard/     # Directory for shunting yard operationn
│   └── operation.rs   # Subroot arithmetic operations used in Shunting yard
└── main.rs            # CLI interface
```

## 🤝 Contributing
To add features (e.g., **SGPA support**):
1. Implement logic in `cgpa.rs`
2. Add tests:
   ```rust
   #[test]
   fn test_cgpa_calculation() {
       assert_eq!(calculate_cgpa(&[3, 4], &["A", "B+"]), 3.45);
   }
   ```
3. Submit a PR!

---

<!-- ### Key Additions:
1. **CGPA Feature Badge** - Violet badge highlights academic use-case
2. **Table Output** - Uses `tabled` crate for clean grade displays
3. **Test Example** - Encourages contributors to verify GPA logic
4. **Installation Options** - Both `cargo install` and manual build

Want to highlight **performance benchmarks** or add ASCII art? Let me know! -->