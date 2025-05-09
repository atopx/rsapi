use axum::body::Bytes;
use axum::extract::FromRequest;
use axum::extract::Request;
use axum::response::IntoResponse as _;
use axum::response::Response;

pub struct BufferBody(pub Bytes);

// we must implement `FromRequest` (and not `FromRequestParts`) to consume the body
impl<S> FromRequest<S> for BufferBody
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let body = Bytes::from_request(req, state).await.map_err(|err| err.into_response())?;

        Ok(Self(body))
    }
}
