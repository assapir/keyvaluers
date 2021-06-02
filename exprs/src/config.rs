#[derive(Debug, Copy, Clone)]
pub struct Config {
    pub(crate) port: u32,
    pub(crate) threads: usize,
}

impl Config {
    /// Create a new config instance
    ///
    /// # Arguments
    /// * `port` - Optional [`u32`] to use as the port. Defaults to 7878.
    /// * `threads` - Optional [`usize`] to set as number of threads. Defaults to [num_cpus::get()](https://docs.rs/num_cpus/1.13.0/num_cpus/fn.get.html).
    ///
    /// # Examples
    /// ```rust
    /// use exprs::Config;
    /// let config = Config::new(Some(8080), Some(10));
    /// ```
    /// With defaults:
    /// ```rust
    /// use exprs::Config;
    /// let config = Config::new(None, None);
    /// ```
    pub fn new(port: Option<u32>, threads: Option<usize>) -> Self {
        let port = match port {
            None => 7878,
            Some(p) => p,
        };

        let threads = match threads {
            None => num_cpus::get(),
            Some(t) => t,
        };

        Self { port, threads }
    }
}

#[cfg(test)]
mod test {
    use super::Config;

    #[test]
    fn test_config_defaults() {
        let config = Config::new(None, None);
        assert_eq!(config.port, 7878);
        assert_eq!(config.threads, num_cpus::get());
    }
}
