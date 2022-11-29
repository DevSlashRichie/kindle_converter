use std::fmt::Debug;

use serde::Deserialize;
use thiserror::Error;
use async_trait::async_trait;

#[derive(Error, Debug)]
pub enum SourceError {
    #[error("Invalid credentials")]
    LoginFailed,
    #[error("Could not request data: {0}")]
    RequestError(#[from] reqwest::Error),
}

#[derive(Debug, Deserialize)]
pub struct PostPreview<ID> {
    pub id: ID,
    pub title: String,
}

#[derive(Debug, Deserialize)]
pub struct Post<ID> {
    pub id: ID,
    pub title: String,
    pub cover_image: String,
    pub body_html: String,
}


#[async_trait]
pub trait Source {

    type PostId: Debug + Clone + Send + Sync;

    async fn get_posts(&self) -> Result<Vec<PostPreview<Self::PostId>>, SourceError>;

    async fn get_post(&self, id: Self::PostId) -> Result<Post<Self::PostId>, SourceError>;

}
