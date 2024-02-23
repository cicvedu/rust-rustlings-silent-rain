use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    println!("cargo:rustc-cfg=feature=\"pass\"");
}
