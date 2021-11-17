use crate::{database::Database, router::Router};
use hyper::{
    service::{make_service_fn, service_fn},
    Server,
};
use std::{convert::Infallible, net::SocketAddr};

pub async fn start(database: Database) {
    Router::init();

    let service = make_service_fn(move |_| {
        let database = database.clone();

        let service = service_fn(move |request| Router::route(database.clone(), request));
        async move { Ok::<_, Infallible>(service) }
    });

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    Server::bind(&addr).serve(service).await.unwrap();
}
