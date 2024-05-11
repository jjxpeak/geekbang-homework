use anyhow::Result;
use axum::{
    extract::State,
    http::{StatusCode, Uri},
    response::Html,
    Router,
};
use std::{
    net::{Ipv4Addr, SocketAddr},
    path::{Path, PathBuf},
};
use tokio::{
    fs::{read_dir, File},
    io::AsyncReadExt,
};
use tower_http::services::ServeDir;

use crate::{Actuator, FtpOpts};
use tracing::info;

#[derive(Debug, Clone)]
pub struct AppState {
    dir: String,
}

#[derive(Debug)]
pub struct FileIndex {
    file_name: String,
    uri: Box<PathBuf>,
    file_type: FileType,
    content: Option<Vec<u8>>,
}

#[derive(Debug, PartialEq, PartialOrd)]
enum FileType {
    File,
    Dir,
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
    let serve_dir = ServeDir::new(opt.dir.clone());
    let _ = serve_dir.fallback(fallback);
    // axum router
    let router = Router::new()
        // .nest_service("/", serve_dir)
        // .route("/a", get(index_headler))
        .fallback(fallback)
        .with_state(AppState { dir: opt.dir });

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router.into_make_service()).await?;
    Ok(())
}

async fn fallback(uri: Uri, State(state): State<AppState>) -> (StatusCode, Html<String>) {
    let index = create_file_index(state.dir, uri.path().to_owned()).await;
    match index {
        Ok(file_index) => {
            match file_index.get(1) {
                Some(f) => if f.content.is_some() {},
                None => todo!(),
            }
            let html = build_html(file_index);
            (StatusCode::OK, html)
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Html::from(e.to_string())),
    }
}

fn build_html(list: Vec<FileIndex>) -> Html<String> {
    let mut html = String::new();
    html.push_str(
        "<!DOCTYPE html>
        <html lang=\"en\">
        <head>
            <meta charset=\"UTF-8\">
            <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
            <title>Index Of</title>
        </head>
        <body>
        <h1>Index Of</h1>
        ",
    );
    for fi in list {
        let content = match fi.file_type {
            FileType::File => {
                format!(
                    "<a href='/{}' download>{}</a><br/>",
                    fi.uri.to_string_lossy(),
                    fi.file_name
                )
            }
            FileType::Dir => {
                format!(
                    "<a href='/{}'>{}</a><br/>",
                    fi.uri.to_string_lossy(),
                    fi.file_name
                )
            }
        };
        html.push_str(content.as_str());
    }
    html.push_str("</body></html>");
    Html::from(html)
}

async fn create_file_index(base_dir: String, path: String) -> Result<Vec<FileIndex>> {
    let path = path.trim_matches('/').to_string();
    info!("path {:?}", path);
    let path_buf = Path::new(&base_dir).join(&path);
    let parent_path = path_buf
        .parent()
        .unwrap_or_else(|| base_dir.as_ref())
        .to_path_buf();
    info!("build_html {:?}", path_buf.parent());
    let mut file_index = Vec::with_capacity(20);
    match read_dir(&path_buf).await {
        Ok(mut read_dir) => {
            while let Some(entry) = read_dir.next_entry().await.unwrap() {
                let file_name = entry.file_name().to_string_lossy().as_ref().to_owned();
                info!("file_ame : {file_name},path : {path}");
                if let Ok(file_type) = entry.file_type().await {
                    let ft = if file_type.is_dir() {
                        FileType::Dir
                    } else {
                        FileType::File
                    };

                    file_index.push(FileIndex {
                        file_name,
                        uri: Box::new(entry.path().to_path_buf()),
                        file_type: ft,
                        content: None,
                    })
                }
            }
        }
        Err(_) => {
            let mut file = File::open(path_buf).await?;
            let mut buf: Vec<u8> = Vec::with_capacity(1024);
            match file.read(&mut buf).await {
                Ok(_) => file_index.push(FileIndex {
                    file_name: "".into(),
                    uri: Box::new("".to_owned().into()),
                    file_type: FileType::File,
                    content: Some(buf),
                }),
                Err(e) => return Err(e.into()),
            }
        }
    }
    file_index.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    file_index.insert(
        0,
        FileIndex {
            file_name: "../".to_owned(),
            uri: Box::new(parent_path),
            file_type: FileType::Dir,
            content: None,
        },
    );
    Ok(file_index)
}

impl PartialEq for FileIndex {
    fn eq(&self, other: &Self) -> bool {
        self.file_name == other.file_name
            && self.uri == other.uri
            && self.file_type == other.file_type
    }
}

impl PartialOrd for FileIndex {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.file_name.partial_cmp(&other.file_name)
    }
}
