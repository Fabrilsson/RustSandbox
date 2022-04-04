use warp::Filter;

#[tokio::main]
async fn main() {
    
    static CONST_STRING: &'static str = "this is a static string";

    let bigstring_rust = warp::path!("bigstringRust")
        .map(|| std::iter::repeat("x").take(10 * 1024).collect::<String>());

    let bigstring_static_rust = warp::path!("bigstaticstringRust")
    .map(|| CONST_STRING);

    let routes = bigstring_rust.or(bigstring_static_rust);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}