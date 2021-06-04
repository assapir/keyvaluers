use crate::{thread_pool::ThreadPool, Config, Request, Response};
use std::net::TcpListener;

type Handler = fn(req: &Request, res: &Response) -> ();

pub struct App {
    pool: ThreadPool,
    listener: TcpListener,
    handlers: Vec<Handler>,
}

impl App {
    pub fn new(config: Config) -> Self {
        let addr = format!("127.0.0.1:{}", config.port);
        let listener =
            TcpListener::bind(&addr).expect(format!("Unable to bind to {}", addr).as_str());
        let pool = ThreadPool::new(config.threads);

        Self { pool, listener, handlers: vec![] }
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
        match response.send("Hello, world!") {
            Ok(it) => it,
            Err(err) => panic!("Unable to send response: {}", err.message),
        }
    }
}
