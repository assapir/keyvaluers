use crate::request::Request;

pub struct Response<'a> {
    request: &'a mut Request,
}

impl<'a> Response<'a> {
    pub fn new(request: &'a mut Request) -> Self {
        Self { request }
    }

    pub fn send(&mut self, message: &str) {
        self.request.write(message).unwrap();
        self.request.end().unwrap();
    }
}
