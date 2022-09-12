use axum::{
    async_trait,
    body::HttpBody,
    extract::{FromRequest, RequestParts},
    response::{IntoResponse, Response},
    BoxError,
};
use serde::{de::DeserializeOwned, Serialize};
use std::ops::{Deref, DerefMut};

use crate::errors::ApiError;

#[derive(Debug, Clone, Copy, Default)]
#[cfg_attr(docsrs, doc(cfg(feature = "json")))]
pub struct Json<T>(pub T);

#[async_trait]
impl<T, B> FromRequest<B> for Json<T>
where
    T: DeserializeOwned,
    B: HttpBody + Send,
    B::Data: Send,
    B::Error: Into<BoxError>,
{
    type Rejection = ApiError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        match axum::Json::<T>::from_request(req).await {
            Ok(value) => Ok(Self(value.0)),
            Err(rejection) => Err(rejection.into()),
        }
    }
}

impl<T> Deref for Json<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Json<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> From<T> for Json<T> {
    fn from(inner: T) -> Self {
        Self(inner)
    }
}

impl<T> IntoResponse for Json<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        axum::Json(self.0).into_response()
    }
}
