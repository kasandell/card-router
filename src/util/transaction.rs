use std::fmt::{Debug, Formatter};
use std::ops::{Deref};
use diesel_async::AsyncConnection;
use diesel_async::scoped_futures::{ScopedFutureExt};
use futures::future::BoxFuture;
use crate::error::data_error::DataError;
use crate::util::db::{connection, DbConnection};

#[derive(derive_more::Deref, derive_more::DerefMut)]
pub struct Transaction<'a, 'b: 'a> {
    conn: &'a mut DbConnection<'b>
}

impl <'a, 'b> Debug for Transaction<'a, 'b> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Transaction")
    }
}

impl <'a, 'b> Transaction<'a, 'b> {
    pub fn new(conn: &'a mut DbConnection<'b>) -> Self{
        Self {
            conn: conn
        }
    }
}

pub async fn transactional<T, E, F>(f: F) -> Result<T, E>
    where
        T: Send + 'static,
        E: From<DataError>,
        F: for<'conn> FnOnce(&'conn mut Transaction) -> BoxFuture<'conn, Result<T, DataError>> + Send + 'static,
{

    let mut conn = connection().await.map_err(|e| DataError::from(e))?;
    let result = conn.transaction(|mut _conn| async move {
        let mut txn = Transaction::new(_conn);
        let x = f(&mut txn).await;
        x
    }.scope_boxed()).await?;
    Ok(result)

}