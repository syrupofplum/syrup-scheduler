#[derive(Debug)]
pub enum ErrorType {

}

#[derive(Debug)]
pub struct ErrorBundle {
    err_type: ErrorType,
    err_msg: String
}
