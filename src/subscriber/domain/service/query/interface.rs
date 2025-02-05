use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use enum_as_inner::EnumAsInner;

use crate::subscriber::domain::error::Error;
use crate::subscriber::domain::infrastructure::SubscriberRepository;
use crate::subscriber::domain::model::Subscriber;
use crate::subscriber::domain::service::query::readers;

#[derive(Clone, EnumAsInner)]
pub enum Query {
    GetConfirmedSubscribers,
}

#[async_trait::async_trait]
pub trait SingularQueryReader: Send + Sync + 'static {
    async fn read(&self, query: Query) -> Result<Subscriber, Error>;
}
#[async_trait::async_trait]
pub trait PluralQueryReader: Send + Sync + 'static {
    async fn read(&self, query: Query) -> Result<Vec<Subscriber>, Error>;
}

pub type SingularQueryReaderFunction = Arc<
    dyn Fn(Query) -> Pin<Box<dyn Future<Output = Result<Subscriber, Error>> + Send>> + Send + Sync,
>;

#[async_trait::async_trait]
impl SingularQueryReader for SingularQueryReaderFunction {
    async fn read(&self, query: Query) -> Result<Subscriber, Error> {
        self(query).await
    }
}

pub type PluralQueryReaderFunction = Arc<
    dyn Fn(Query) -> Pin<Box<dyn Future<Output = Result<Vec<Subscriber>, Error>> + Send>>
        + Send
        + Sync,
>;

#[async_trait::async_trait]
impl PluralQueryReader for PluralQueryReaderFunction {
    async fn read(&self, query: Query) -> Result<Vec<Subscriber>, Error> {
        self(query).await
    }
}

pub fn new_singular_query_reader(
    subscriber_repository: impl SubscriberRepository,
) -> SingularQueryReaderFunction {
    Arc::new(move |query| {
        let _subscriber_repository = subscriber_repository.clone();

        Box::pin(async move {
            match query {
                Query::GetConfirmedSubscribers => Err(Error::QueryResultCollectivityMismatched),
            }
        })
    })
}

pub fn new_plural_query_reader(
    subscriber_repository: impl SubscriberRepository,
) -> PluralQueryReaderFunction {
    Arc::new(move |query| {
        let subscriber_repository = subscriber_repository.clone();

        Box::pin(async move {
            match query {
                Query::GetConfirmedSubscribers => {
                    readers::get_confirmed_subscribers::read(subscriber_repository).await
                }
            }
        })
    })
}
