use std::fs;

use axum::Extension;
use axum::body::Bytes;
use axum::extract::Multipart;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde_json::json;
use tracing::info;

use crate::jwt;
use crate::response::Empty;
use crate::response::Response;

struct ExtractedFile {
    filename: String,
    data: Bytes,
}

const FILEDIR: &str = "./data";

/// example upload file
pub async fn upload(
    Extension(claims): Extension<jwt::Claims>, mut multipart: Multipart,
) -> impl IntoResponse {
    let mut extracted_file = None;

    // 处理每一个字段
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap_or_default().to_string();
        match name.as_str() {
            "file" => {
                if let Some(filename) = field.file_name().map(|s| s.to_string()) {
                    let data = match field.bytes().await {
                        Ok(bytes) => bytes,
                        Err(e) => {
                            tracing::warn!(error=%e, "Error reading file bytes");
                            return Response::error("error reading file bytes", StatusCode::BAD_REQUEST);
                        }
                    };
                    extracted_file = Some(ExtractedFile { filename, data });
                } else {
                    tracing::warn!("File field missing filename");
                    return Response::error("file field missing filename", StatusCode::BAD_REQUEST);
                }
            }
            // process other fields here
            _ => {}
        };
    }

    // 处理文件
    let extract_file = match extracted_file {
        Some(file) => file,
        None => {
            tracing::warn!("No file field found in multipart form data");
            return Response::error(
                "no file field found in multipart form data",
                StatusCode::BAD_REQUEST,
            );
        }
    };

    if extract_file.filename.is_empty() {
        tracing::warn!("Filename is empty");
        return Response::error("filename is empty", StatusCode::BAD_REQUEST);
    }

    if let Err(e) = fs::write(format!("{}/{}", FILEDIR, &extract_file.filename), &extract_file.data) {
        tracing::warn!(error=%e, "Error writing file");
        return Response::error("error writing file", StatusCode::BAD_REQUEST);
    }

    Response::success(json!({
        "filename": &extract_file.filename,
        "size": extract_file.data.len(),
        "owner": claims.username,
    }))
}

/// example download file
pub async fn download(
    Path(filename): Path<String>, Extension(claims): Extension<jwt::Claims>,
) -> axum::http::Response<axum::body::Body> {
    info!("{} download {}", claims.username, filename);

    let file = match tokio::fs::File::open(&format!("{}/{}", FILEDIR, filename)).await {
        Ok(file) => file,
        Err(_) => {
            return Response::<Empty>::error("file not found", StatusCode::NOT_FOUND).into_response();
        }
    };

    let stream = tokio_util::io::ReaderStream::new(file);
    let body = axum::body::Body::from_stream(stream);

    Response::<Empty>::file(filename, body, None)
}
