use crate::Router;
use anyhow::Result;
use hyper::{
    service::{make_service_fn, service_fn},
    Server,
};
use std::{convert::Infallible, net::SocketAddr};

pub async fn start() -> Result<()> {
    Router::init();

    let service = make_service_fn(move |_| {
        let service = service_fn(move |req| Router::route(req));
        async move { Ok::<_, Infallible>(service) }
    });

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    Server::bind(&addr).serve(service).await?;

    Ok(())
}
