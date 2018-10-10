#[macro_use]
extern crate serde_derive;
extern crate actix_web;
extern crate bytes;

use actix_web::middleware::{
    Middleware,
    Started,
};
use actix_web::{
    http,
    middleware,
    server,
    App,
    AsyncResponder,
    HttpMessage,
    HttpRequest,
    HttpResponse,
    Json,
    Result,
};
use bytes::Bytes;
use futures::future::Future;

fn main() {
    println!("Starting http server: 0.0.0.0:8080");

    server::new(|| {
        App::new()
            .middleware(middleware::Logger::default())
            .middleware(Saver)
            .resource("/v1/bitbucketWebhook", |r| {
                r.method(http::Method::POST).with(endpoint)
            })
    })
    .bind("0.0.0.0:8080")
    .unwrap()
    .run();
}

#[derive(Debug, Serialize, Deserialize)]
struct Payload {
    data: String,
}

fn endpoint(payload: Json<Payload>) -> Result<HttpResponse> {
    Ok(HttpResponse::build(http::StatusCode::OK)
        .content_type("text/plain")
        .body(format!("{:#?}", payload)))
}

struct Saver;

impl<S: 'static> Middleware<S> for Saver {
    fn start(&self, req: &HttpRequest<S>) -> Result<Started> {
        Ok(Started::Future(
            req.body()
                .limit(1024)
                .from_err()
                .and_then(|bytes: Bytes| {
                    println!("==== BODY ==== {:?}", bytes);

                    Ok(HttpResponse::Ok().into())
                })
                .responder(),
        ))
    }
}
