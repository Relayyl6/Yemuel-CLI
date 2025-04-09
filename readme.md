### **Generic type` function decleration**
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
```