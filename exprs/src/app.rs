use crate::{Config, Request, Response, thread_pool::ThreadPool};
use std::net::TcpListener;

#[derive(Debug)]
pub struct App {
    pool: ThreadPool,
    listener: TcpListener,
}

impl App {
    pub fn new(config: Config) -> Self {
        let addr = format!("127.0.0.1:{}", config.port);
        let listener = TcpListener::bind(&addr).expect(format!("Unable to bind to {}", addr).as_str());
        let pool = ThreadPool::new(config.threads);

        Self { pool, listener }
    }

    pub fn start(&self) {
        for stream in self.listener.incoming() {
            self.pool.execute(|| {
                let mut request = match Request::new(stream.unwrap()) {
                    Ok(it) => it,
                    Err(err) => {
                        println!("Error: {}", err.message);
                        return;
                    }
                };
                Self::handle_connection(&mut request);
            });
        }
    }

    fn handle_connection(request: &mut Request) {
        let mut response = Response::new(request);
        response.send("Hello, world!")
    }
}
