use axum::async_trait;
use axum::extract::{FromRequest, RequestParts};
use serde::de::DeserializeOwned;
use std::ops::Deref;

use crate::errors::ApiError;

#[derive(Debug, Clone, Copy, Default)]
pub struct Query<T>(pub T);

#[async_trait]
impl<T, B> FromRequest<B> for Query<T>
where
    T: DeserializeOwned,
    B: Send,
{
    type Rejection = ApiError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        match axum::extract::Query::<T>::from_request(req).await {
            Ok(value) => Ok(Self(value.0)),
            Err(rejection) => Err(rejection.into()),
        }
    }
}

impl<T> Deref for Query<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
