fn main() {
    // Add target as compile-time environment variable
    println!(
        "cargo:rustc-env=TARGET={}",
        std::env::var("TARGET").unwrap()
    );
}