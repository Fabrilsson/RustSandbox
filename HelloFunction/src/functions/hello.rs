use azure_functions::{
    bindings::{HttpRequest, HttpResponse},
    func,
};

#[func]
pub fn hello(req: HttpRequest) -> HttpResponse {
    "Hello from Rust!".into()
}
