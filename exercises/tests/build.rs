//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

fn main() {
    // Set the environment variable `TEST_FOO` for tests7.
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    // Enable the `pass` feature for tests8.
    println!("cargo:rustc-cfg=feature=\"pass\"");
}
