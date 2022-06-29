use async_trait::async_trait;
use profile_domain::{
    error::{Error, Result},
    model::profile_creation_data::ProfileCreationData,
    port::profile_repository::ProfileRepository,
};
use sqlx::{Postgres, Transaction};

pub enum TransactionResult<T> {
    Commit(T),
    Rollback(T),
}

pub struct PgProfileRepository<'a, 'b: 'a> {
    pub transaction: &'a mut Transaction<'b, Postgres>,
}

pub trait RepositoryProvider<T> {
    fn get_repository(&mut self) -> T;
}

#[async_trait(?Send)]
impl<'a, 'b: 'a> ProfileRepository for PgProfileRepository<'a, 'b> {
    async fn create_profile(&mut self, new_profile: &ProfileCreationData) -> Result<()> {
        sqlx::query!(
            "
        INSERT INTO profile ( name, email, password ) 
        VALUES ($1, $2, $3)
        ",
            new_profile.name,
            new_profile.email,
            new_profile.password,
        )
        .execute(&mut *self.transaction)
        .await
        .map_err(|_| Error::DbConnectionError)
        .map(|_| (()))
    }
}