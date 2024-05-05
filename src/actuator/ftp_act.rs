use anyhow::Result;
use axum::{
    extract::State,
    http::{StatusCode, Uri},
    response::Html,
    Router,
};
use std::net::{Ipv4Addr, SocketAddr};

use crate::{Actuator, FtpOpts};
use tracing::info;

#[derive(Debug, Clone)]
pub struct AppState {
    dir: String,
}

impl Actuator for FtpOpts {
    fn execute(self) -> Result<()> {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()?;
        rt.block_on(app_init(self))?;
        Ok(())
    }
}

async fn app_init(opt: FtpOpts) -> Result<()> {
    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, opt.port));
    info!("Serving {:?} on {}", &opt.dir, addr);
    // axum router
    let router = Router::new()
        // .nest_service("/", ServeDir::new(opt.dir.clone())).fallback_service(ServeFile::new(opt.dir.clone()))
        // .route("/a", get(index_headler))
        .fallback(fallback)
        .with_state(AppState { dir: opt.dir });

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router.into_make_service()).await?;
    Ok(())
}

async fn fallback(uri: Uri, State(state): State<AppState>) -> (StatusCode, String) {
    info!("fallback {uri}");
    (
        StatusCode::NOT_FOUND,
        format!("No route for {uri} {:?}", state.dir),
    )
}

async fn _index_headler(State(_state): State<AppState>) -> Result<Html<String>, StatusCode> {
    Ok(Html("Hello word!".into()))
}
