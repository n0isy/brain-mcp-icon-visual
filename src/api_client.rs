use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize)]
pub struct SearchReq {
    pub keyword: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}

#[derive(Serialize)]
pub struct SvgReq {
    pub icon: String,
}

#[derive(Serialize)]
pub struct RenderReq {
    pub svgs: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<String>,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct SearchResp {
    pub count: usize,
    pub grid_png: String,
    pub urls: HashMap<String, String>,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct SvgResp {
    pub icon: String,
    pub resolved_url: String,
    pub svg: String,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct RenderResp {
    pub count: usize,
    pub grid_png: String,
}

#[derive(Clone)]
pub struct ApiClient {
    http: reqwest::Client,
    base: String,
}

impl ApiClient {
    pub fn new(http: reqwest::Client, base: String) -> Self {
        Self { http, base }
    }

    pub async fn search(&self, req: &SearchReq) -> Result<SearchResp, reqwest::Error> {
        self.http
            .post(format!("{}/search", self.base))
            .json(req)
            .send()
            .await?
            .json()
            .await
    }

    pub async fn svg(&self, req: &SvgReq) -> Result<SvgResp, reqwest::Error> {
        self.http
            .post(format!("{}/svg", self.base))
            .json(req)
            .send()
            .await?
            .json()
            .await
    }

    pub async fn render(&self, req: &RenderReq) -> Result<RenderResp, reqwest::Error> {
        self.http
            .post(format!("{}/render", self.base))
            .json(req)
            .send()
            .await?
            .json()
            .await
    }
}
