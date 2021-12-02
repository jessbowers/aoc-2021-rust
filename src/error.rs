use derive_more::Display;

#[derive(Debug, Display)]
enum MyError {
    #[display(fmt = "IO {}", _0)]
    IO(std::io::Error),

    #[display(fmt = "Internal {}", _0)]
    Internal(String),
}

impl From<std::io::Error> for MyError {
    fn from(err: std::io::Error) -> Self {
        MyError::IO(err)
    }
}
