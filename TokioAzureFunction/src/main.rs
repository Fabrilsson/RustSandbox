use std::env;
use std::net::Ipv4Addr;
use warp::Filter;

#[tokio::main]
async fn main() {

    let example1 = warp::get()
        .and(warp::path("api"))
        .and(warp::path("httpexample"))        
        .map(|| std::iter::repeat("X").take(10 * 1024).collect::<String>());

    let port_key = "FUNCTIONS_CUSTOMHANDLER_PORT";
    let port: u16 = match env::var(port_key) {
        Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
        Err(_) => 3000,
    };

    warp::serve(example1).run((Ipv4Addr::UNSPECIFIED, port)).await
}