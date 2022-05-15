use warp::{http, Filter};
use parking_lot::RwLock;
use std::sync::Arc;
use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Item {
    id: i64,
    name: String,
    quantity: i32,
    value: f64,
}

static mut IDENTIFIER: i64 = 0;

type Items = Vec<Item>;

#[derive(Clone)]
struct Store {
  grocery_list: Arc<RwLock<Items>>,
}

impl Store {
    fn new() -> Self {
        Store {
            grocery_list: Arc::new(RwLock::new(Vec::new())),
        }
    }
}

async fn update_grocery_list(
    item: Item,
    store: Store
    ) -> Result<impl warp::Reply, warp::Rejection> {

        unsafe {
            IDENTIFIER = IDENTIFIER + 1;
            store.grocery_list.write().push(
                Item 
                { 
                    id: IDENTIFIER,
                    name: item.name,
                    quantity: item.quantity,
                    value: item.value 
                });
        }

        Ok(warp::reply::with_status(
            "Added items to the grocery list",
            http::StatusCode::CREATED,
        ))
}

async fn delete_grocery_list_item(
    id: i64,
    store: Store
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let mut r = store.grocery_list.write();

        r.retain(|a| a.id != id);

        Ok(warp::reply::with_status(
            "Removed item from grocery list",
            http::StatusCode::OK,
        ))
}

async fn get_grocery_list(
    _context: Store
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let mut result = Vec::new();
        let r = _context.grocery_list.read();


        for value in r.iter() {
            result.push(value);
        }

        Ok(warp::reply::json(
            &result
        ))
}

fn delete_json() -> impl Filter<Extract = (i64,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

fn post_json() -> impl Filter<Extract = (Item,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

#[tokio::main]
async fn main() {
    let store = Store::new();
    let store_filter = warp::any().map(move || store.clone());

    let add_items = warp::post()
        .and(warp::path("v1"))
        .and(warp::path("groceries"))
        .and(warp::path::end())
        .and(post_json())
        .and(store_filter.clone())
        .and_then(update_grocery_list);

    let get_items = warp::get()
        .and(warp::path("v1"))
        .and(warp::path("groceries"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(get_grocery_list);

    let delete_item = warp::delete()
        .and(warp::path("v1"))
        .and(warp::path("groceries"))
        .and(warp::path::end())
        .and(delete_json())
        .and(store_filter.clone())
        .and_then(delete_grocery_list_item);


    let update_item = warp::put()
        .and(warp::path("v1"))
        .and(warp::path("groceries"))
        .and(warp::path::end())
        .and(post_json())
        .and(store_filter.clone())
        .and_then(update_grocery_list);



    let routes = add_items.or(get_items).or(delete_item).or(update_item);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}