pub mod entity;
pub mod mapper;
pub mod password_validator;
pub mod pg_auth_repository;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
