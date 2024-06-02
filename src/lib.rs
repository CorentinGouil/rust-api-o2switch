use napi_derive::napi;

use axum::{routing::get, Router};
use tokio::net::{TcpListener, UnixListener};

#[napi]
pub async fn listen(path: Option<String>) {
    let task = tokio::task::spawn(async move {
        let app = Router::new().route("/", get(|| async { "Hello, world!" }));

        match path {
            Some(p) => axum::serve(UnixListener::bind(p).unwrap(), app)
                .await
                .unwrap(),
            None => axum::serve(TcpListener::bind("127.0.0.1:3000").await.unwrap(), app)
                .await
                .unwrap(),
        };
    });

    task.await.unwrap();
}
