use crate::{request::Request, ReSendError};

pub struct Response<'a> {
    request: &'a mut Request,
    sent: bool,
}

impl<'a> Response<'a> {
    pub fn new(request: &'a mut Request) -> Self {
        Self {
            request,
            sent: false,
        }
    }

    pub fn send(&mut self, message: &str) -> Result<(), ReSendError> {
        if self.sent {
            return Err(ReSendError::new());
        }
        self.sent = true;
        self.request.write(message).unwrap();
        self.request.end().unwrap();
        Ok(())
    }
}
