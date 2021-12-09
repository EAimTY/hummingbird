use crate::{Config, RouteTable};
use anyhow::Result;
use hyper::{
    service::{make_service_fn, service_fn},
    Server,
};
use std::convert::Infallible;

pub async fn start() -> Result<()> {
    let service = make_service_fn(move |_| {
        let service = service_fn(RouteTable::route);
        async move { Ok::<_, Infallible>(service) }
    });

    Server::bind(&Config::read().application.listen)
        .serve(service)
        .await?;

    Ok(())
}
