use serde::Deserialize;
use serde_json::json;
use async_trait::async_trait;

use super::source::{Source, SourceError, Post, PostPreview};


const HOST: &str = "https://substack.com/api/v1";

pub struct Substack {
    client: reqwest::Client,
}

impl Substack {

    pub fn new() -> Self {
        let client = reqwest::ClientBuilder::new()
            .cookie_store(true)
            .https_only(true)
            .build()
            .unwrap();

        Self {
            client,
        }
    }

    pub async fn login(&self, email: &str, password: &str) {
        let response = self.client
            .post(format!("{}/login", HOST))
            .json(&json!({
                "email": email,
                "password": password,
            }))
            .send()
            .await;

    }

}

#[derive(Deserialize)]
struct TotalPosts {
    posts: Vec<PostPreview<u32>>,
}

#[derive(Deserialize)]
struct PostWrapper {
    post: Post<u32>,
}

#[async_trait]
impl Source for Substack {
    type PostId = u32;


    async fn get_posts(&self) -> Result<Vec<PostPreview<u32>>, SourceError> {
        let response = self.client
            .get(format!("{}/reader/posts/", HOST))
            .query(&[("limit", "12")])
            .send()
            .await?;

        let posts: TotalPosts = response.json().await?;
            
        Ok(posts.posts)

    }

    async fn get_post(&self, id: u32) -> Result<Post<u32>, SourceError> {
        let response = self.client
            .get(format!("{}/posts/by-id/{}", HOST, id))
            .send()
            .await?;

        let post: PostWrapper = response.json().await?;

        Ok(post.post)
    }

}