use std::net::TcpListener;

use exprs::{request::Request, thread_pool::ThreadPool, response::Response};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        pool.execute(|| {
            let mut request = match Request::new(stream.unwrap()) {
                Ok(it) => it,
                Err(err) => {
                    println!("Error: {}", err.message);
                    return;
                }
            };
            handle_connection(&mut request);
        });
    }
}

fn handle_connection(request: &mut Request) {
    let mut response = Response::new(request);
    response.send("Hello, world!")
}
