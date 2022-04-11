fn env(s: &str) -> String { std::env::var(s).unwrap() }

fn main() {
    for s in env("DEP_GBA_LINK_SEARCH").split(':') {
        println!("cargo:rustc-link-search={}", s);
    }

    for s in env("DEP_GBA_LINK_ARGS").split(':') {
        println!("cargo:rustc-link-arg-bins={}", s);
    }
}