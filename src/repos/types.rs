use failure::Error as FailureError;
use futures::future::Future;

/// Repos layer Future
pub type RepoFuture<T> = Box<Future<Item = T, Error = FailureError> + Send>;
pub type RepoResult<T> = Result<T, FailureError>;
