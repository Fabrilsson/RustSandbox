use warp::Filter;

#[tokio::main]
async fn main() {
    
    let hello = warp::path!("bigstringRust")
        .map(|| std::iter::repeat("x").take(10 * 1024).collect::<String>());

    warp::serve(hello)
        .run(([127, 0, 0, 1], 3030))
        .await;
}