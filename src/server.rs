use crate::RouteTable;
use anyhow::Result;
use hyper::{
    service::{make_service_fn, service_fn},
    Server,
};
use std::{convert::Infallible, net::SocketAddr};

pub async fn start() -> Result<()> {
    let service = make_service_fn(move |_| {
        let service = service_fn(RouteTable::route);
        async move { Ok::<_, Infallible>(service) }
    });

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    Server::bind(&addr).serve(service).await?;

    Ok(())
}
