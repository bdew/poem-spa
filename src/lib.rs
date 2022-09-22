use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

use poem::{
    error::StaticFileError, http::Method, web::StaticFileRequest, Endpoint, FromRequest,
    IntoResponse, Request, Response, Result,
};

#[cfg(test)]
mod test;

#[derive(Debug)]
pub struct SPAEndpoint {
    base: PathBuf,
    index: PathBuf,
    assets: Vec<PathBuf>,
}

impl SPAEndpoint {
    pub fn new(base: impl Into<PathBuf>, index: impl Into<PathBuf>) -> Self {
        let base_path = base.into();
        Self {
            index: base_path.join(index.into()),
            base: base_path,
            assets: Vec::new(),
        }
    }

    #[must_use]
    pub fn with_assets(self, asset: impl Into<PathBuf>) -> Self {
        Self {
            assets: [self.assets, vec![self.base.join(asset.into())]].concat(),
            ..self
        }
    }

    fn path_allowed(&self, path: &Path) -> bool {
        path.starts_with(&self.base)
    }

    fn is_asset(&self, path: &Path) -> bool {
        self.assets.iter().any(|asset| path.starts_with(asset))
    }
}

async fn serve_static(req: &Request, file_path: &PathBuf) -> Result<Response> {
    Ok(StaticFileRequest::from_request_without_body(req)
        .await?
        .create_response(file_path, true)?
        .into_response())
}

#[async_trait::async_trait]
impl Endpoint for SPAEndpoint {
    type Output = Response;

    async fn call(&self, req: Request) -> Result<Self::Output> {
        if req.method() != Method::GET {
            return Err(StaticFileError::MethodNotAllowed(req.method().clone()).into());
        }

        let path = req
            .uri()
            .path()
            .trim_start_matches('/')
            .trim_end_matches('/');

        let path = percent_encoding::percent_decode_str(path)
            .decode_utf8()
            .map_err(|_| StaticFileError::InvalidPath)?;

        let mut file_path = self.base.clone();
        for p in Path::new(&*path) {
            if p == OsStr::new(".") {
                continue;
            } else if p == OsStr::new("..") {
                file_path.pop();
            } else {
                file_path.push(p);
            }
        }

        if !self.path_allowed(&file_path) {
            return Err(StaticFileError::Forbidden(file_path.display().to_string()).into());
        }

        if file_path.exists() && file_path.is_file() {
            serve_static(&req, &file_path).await
        } else if self.is_asset(&file_path) {
            if file_path.exists() {
                return Err(StaticFileError::Forbidden(file_path.display().to_string()).into());
            } else {
                Err(StaticFileError::NotFound.into())
            }
        } else {
            serve_static(&req, &self.index).await
        }
    }
}
