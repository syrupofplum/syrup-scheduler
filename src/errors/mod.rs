#[derive(Debug)]
pub enum ErrorBundleType {
    GeneralError,
    TaskBasketEmpty
}

#[derive(Debug)]
pub struct ErrorBundle {
    err_type: ErrorBundleType,
    err_msg: String
}

impl ErrorBundle {
    pub fn new() -> Self {
        Self {
            err_type: ErrorBundleType::GeneralError,
            err_msg: "".to_string()
        }
    }

    pub fn err_type(mut self, err_type: ErrorBundleType) -> Self {
        self.err_type = err_type;
        self
    }

    pub fn err_msg<T: Into<String>>(mut self, err_msg: T) -> Self {
        self.err_msg = err_msg.into();
        self
    }
}
