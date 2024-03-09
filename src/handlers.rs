use wrap::Filter;
use super::models::Post;

pub async fn get_post(id: u64) -> Result<impl warp::Reply, wrap::Rejection> {
    let post = Post {
        id,
        title: String::from("Hello, Warp!"),
        body: String::from("This is a post about Warp."),
    };
    Ok(warp::reply::json(&post))
}