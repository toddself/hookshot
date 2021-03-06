use std::process::Output;

#[derive(Debug)]
pub struct Error {
    pub desc: &'static str,
    pub subject: Option<String>,
}
impl Error {
    pub fn subject(&self) -> Option<String> {
        self.subject.clone()
    }
}
pub struct CommandError {
    pub desc: &'static str,
    pub output: Option<Output>,
    pub detail: Option<String>,
}
