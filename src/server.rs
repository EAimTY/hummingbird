use crate::{Database, Router};
use anyhow::Result;
use hyper::{
    service::{make_service_fn, service_fn},
    Server,
};
use std::{convert::Infallible, net::SocketAddr};

pub async fn start(db: Database) -> Result<()> {
    Router::init();

    let service = make_service_fn(move |_| {
        let db = db.clone();

        let service = service_fn(move |req| Router::route(db.clone(), req));
        async move { Ok::<_, Infallible>(service) }
    });

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    Server::bind(&addr).serve(service).await?;

    Ok(())
}
