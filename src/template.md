# Rust Recipes:
![Custom Shield](https://img.shields.io/badge/Language-Rust-orange.svg?style=for-the-badge&logo=rust) 

1. Printing in same-line:
```rust
print!("Something")
print!(" Else")
----------------------------------------------------------------------
OUTPUT:
Something Else
```

2. Getting the current time in UNIX:

   ```rust
   use std::time::{Systemtime, UNIX_EPOCHS};
   
   fn main(){
       let start = SystemTime::now();
       let since_the_epoch = start.duration_since(UNIX_EPOCH)
                               .expect("Time went backwards");
   
       let current_time: String = (since_the_epoch.as_secs() * 1000 +
                                   since_the_epoch.subsec_nanos() as u64 / 1_000_000)
                                   .to_string(); // Wierdly u64: has to_string() and u32: has to_String()
       								
       println!("{:?}", current_time);
   }
   ```

   