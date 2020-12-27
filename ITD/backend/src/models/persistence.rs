// use futures::Future;
// use sqlx::PgPool;

// pub trait Persistent {
//     type Inner;
//     fn into_inner(self) -> Self::Inner;
//     fn persist(conn: &PgPool, inner: Self::Inner) -> Result<Self, Box<dyn std::error::Error>>;
    
//     // fn get(primary_key: P) -> impl Future<Option<Self>>;
// }