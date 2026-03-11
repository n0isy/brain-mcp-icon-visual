use crate::api_client::{ApiClient, SvgReq};
use crate::error::ResolveError;

#[derive(Debug, PartialEq)]
pub enum SourceKind {
    Inline,
    Url,
    FilePath,
    IconifyId,
}

pub fn classify(source: &str) -> SourceKind {
    let s = source.trim();
    if s.starts_with('<') {
        return SourceKind::Inline;
    }
    if s.starts_with("http://") || s.starts_with("https://") {
        return SourceKind::Url;
    }
    if s.starts_with('/')
        || s.starts_with("./")
        || s.starts_with("~/")
        || s.ends_with(".svg")
    {
        return SourceKind::FilePath;
    }
    SourceKind::IconifyId
}

pub async fn resolve_svg(
    api: &ApiClient,
    source: &str,
) -> Result<String, ResolveError> {
    match classify(source) {
        SourceKind::Inline => Ok(source.trim().to_string()),
        SourceKind::FilePath => tokio::fs::read_to_string(source)
            .await
            .map_err(|e| ResolveError::FileNotFound(source.into(), e.to_string())),
        SourceKind::Url | SourceKind::IconifyId => {
            let resp = api.svg(&SvgReq { icon: source.into() }).await?;
            Ok(resp.svg)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classify_inline() {
        assert_eq!(classify("<svg>...</svg>"), SourceKind::Inline);
        assert_eq!(classify("  <svg xmlns='...'>"), SourceKind::Inline);
    }

    #[test]
    fn classify_url() {
        assert_eq!(
            classify("https://mdn.alipayobjects.com/foo"),
            SourceKind::Url
        );
        assert_eq!(classify("http://example.com/icon.svg"), SourceKind::Url);
    }

    #[test]
    fn classify_file() {
        assert_eq!(classify("/home/user/icon.svg"), SourceKind::FilePath);
        assert_eq!(classify("./icons/logo.svg"), SourceKind::FilePath);
        assert_eq!(classify("~/project/icon.svg"), SourceKind::FilePath);
        assert_eq!(classify("assets/cloud.svg"), SourceKind::FilePath);
    }

    #[test]
    fn classify_iconify() {
        assert_eq!(classify("mdi/cloud"), SourceKind::IconifyId);
        assert_eq!(classify("lucide/home"), SourceKind::IconifyId);
    }
}
