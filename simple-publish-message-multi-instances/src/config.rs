pub struct Config {
    pub port: u32,
}

impl Config {
    pub fn from_env() -> Self {
        let port = std::env::var("APPLICATION_PORT")
            .unwrap_or_else(|_| panic!("APPLICATION_PORT must be set"))
            .parse()
            .unwrap();
        Self { port }
    }
}
