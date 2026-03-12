pub mod query;
pub mod mutation;

use async_graphql::{EmptySubscription, Schema};

pub use query::QueryRoot;
pub use mutation::MutationRoot;

pub type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;
