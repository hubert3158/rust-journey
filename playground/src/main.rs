fn main() {
    pub struct Config {
        retries: u32,
        timeout_ms: u64,
    }

    impl Config {
        pub const fn new(retries: u32) -> Self {
            Config {
                retries,
                timeout_ms: 5000,
            }
        }
    }

    static DEFAULT: Config = Config::new(3); // ❌ without const fn
    const FAST: Config = Config::new(1);
}
