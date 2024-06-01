use std::fmt::{Debug, Formatter};
use std::future::Future;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use bb8::RunError;
use diesel_async::AsyncConnection;
use parking_lot::{Mutex, MutexGuard};
use crate::error::data_error::DataError;
use crate::util::db::{connection, DbConnection};

#[derive(derive_more::Deref, derive_more::DerefMut)]
pub struct Transaction<'a> {
    conn: Arc<Mutex<DbConnection<'a>>>
}

impl <'a> Debug for Transaction<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Transaction")
    }
}

impl <'a> Transaction<'a> {
    pub fn new(conn: Arc<Mutex<DbConnection<'a>>>) -> Self{
        Self {
            conn
        }
    }

    pub fn conn(self: &Self) -> Arc<Mutex<DbConnection<'a>>> {
        self.conn.clone()
    }
}

trait XFn<'a, I: 'a, O> {
    type Output: Future<Output = O> + 'a;
    fn call(&self, session: I) -> Self::Output;
}
impl<'a, I: 'a, O, F, Fut> XFn<'a, I, O> for F
    where
        F: Fn(I) -> Fut,
        Fut: Future<Output = O> + 'a,
{
    type Output = Fut;
    fn call(&self, x: I) -> Fut {
        self(x)
    }
}
pub async fn transactional<'a, F, Fut, R, E>(f: F) -> Result<R, E>
    where
        Fut: Future<Output=Result<R, DataError>> +'a,
        F: FnOnce(Arc<Transaction<'a>>) -> Fut,
        R: Send + 'a,
        E: From<DataError>
{
    /*
    let conn = connection().await.map_err(RunError::into)?;
    conn.transaction(|_conn| async move {

    })
     */
    // TODO: this doesn't actually guard a txn
    let mut conn = Arc::new(Mutex::new(connection().await.map_err(RunError::into)?));
    let txn = Arc::new(Transaction::new(conn.clone()));
    f(txn.clone()).await.map_err(DataError::into)
}
