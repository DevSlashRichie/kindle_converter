use sources::{substack::{self}, source::Source};

mod sources;
mod converter;

use substack::Substack;

use std::fs;


pub async fn execute() {
    let substack = Substack::new();

    substack
        .login(
            "****", 
            "*****"
        ).await;
    
    print!("Logged in!");

    let last_post = substack.get_posts()
        .await
        .unwrap();

    print!("Got posts!");

    let last_post = last_post.last()
        .unwrap();

    print!("Got last post!");

    let last_post = substack.get_post(last_post.id)
        .await
        .unwrap();

    print!("Got last post totally!");

    let file = converter::html::convert(
        last_post.title,
         last_post.body_html
    )
    .unwrap();

    print!("Converted!");

    fs::write("test.pdf", file)
        .unwrap();

    print!("Saved!");
}
