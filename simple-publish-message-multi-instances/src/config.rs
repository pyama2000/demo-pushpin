pub struct Config {
    pub port: u32,
    pub grip_proxies: Vec<GripProxy>,
}

impl Config {
    pub fn from_env() -> Self {
        let port = std::env::var("APPLICATION_PORT")
            .unwrap_or_else(|_| panic!("APPLICATION_PORT must be set"))
            .parse()
            .unwrap();

        let grip_proxy_hosts = std::env::var("GRIP_PROXY_HOSTS")
            .unwrap_or_else(|_| panic!("GRIP_PROXY_HOSTS must be set"));
        let grip_proxy_hosts: Vec<_> = grip_proxy_hosts.split(',').collect();
        let grip_proxy_port = std::env::var("GRIP_PROXY_PUBLISH_PORT")
            .unwrap_or_else(|_| panic!("GRIP_PROXY_PUBLISH_PORT must be set"))
            .parse()
            .unwrap();
        let mut grip_proxies = Vec::new();
        for host in grip_proxy_hosts {
            grip_proxies.push(GripProxy::new(host.to_string(), grip_proxy_port))
        }

        Self { port, grip_proxies }
    }
}

pub struct GripProxy {
    pub host: String,
    pub publish_port: u32,
}

impl GripProxy {
    pub fn new(host: String, publish_port: u32) -> Self {
        Self { host, publish_port }
    }
}
