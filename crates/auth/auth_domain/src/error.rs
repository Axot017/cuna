pub type Result<T> = std::result::Result<T, Error>;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Error {
    DbConnectionError,
    InvalidPassword,
    InvalidLogin,
    InvalicClientSecret,
}
