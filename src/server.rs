use rmcp::{
    ErrorData, ServerHandler,
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::*,
    tool, tool_handler, tool_router,
};
use std::collections::HashMap;

use crate::api_client::{ApiClient, RenderReq, SearchReq};
use crate::resolve::resolve_svg;
use crate::tools::{GetSvgParams, RenderGridParams, SearchIconsParams};

#[derive(Clone)]
pub struct IconServer {
    tool_router: ToolRouter<Self>,
    api: ApiClient,
}

#[tool_router]
impl IconServer {
    pub fn new(api_base: String) -> Self {
        let http = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("http client");
        let api = ApiClient::new(http, api_base);
        Self {
            tool_router: Self::tool_router(),
            api,
        }
    }

    #[tool(
        name = "search_icons",
        description = "Semantic icon search. Returns a 4x4 PNG grid (512x512, cells 0-15) and a mapping of cell numbers to SVG URLs. Use cell numbers to reference icons in subsequent get_svg and render_grid calls."
    )]
    async fn search_icons(
        &self,
        Parameters(p): Parameters<SearchIconsParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let resp = self
            .api
            .search(&SearchReq {
                keyword: p.keyword,
                background: Some(p.background),
                color: p.color,
            })
            .await
            .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;

        Ok(grid_with_urls(&resp.grid_png, &resp.urls))
    }

    #[tool(
        name = "get_svg",
        description = "Retrieve raw SVG markup from any source. Returns the complete SVG string. Use this to inspect or modify SVG content before passing to render_grid."
    )]
    async fn get_svg(
        &self,
        Parameters(p): Parameters<GetSvgParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let svg = resolve_svg(&self.api, &p.source)
            .await
            .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;

        Ok(svg_text(&svg, &p.source))
    }

    #[tool(
        name = "render_grid",
        description = "Render 1-16 SVGs onto a 4x4 comparison grid (512x512, cells 0-15). Uses the same grid layout as search_icons for pixel-perfect visual comparison. Typical uses: compare icon variants side-by-side, preview local project icons alongside search results, show multiple iterations of a design."
    )]
    async fn render_grid(
        &self,
        Parameters(p): Parameters<RenderGridParams>,
    ) -> Result<CallToolResult, ErrorData> {
        if p.sources.is_empty() || p.sources.len() > 16 {
            return Err(ErrorData::invalid_params(
                "sources must contain 1-16 items",
                None,
            ));
        }

        let total = p.sources.len();
        let mut svgs = Vec::with_capacity(total);
        let mut errors = Vec::new();

        for (i, source) in p.sources.iter().enumerate() {
            match resolve_svg(&self.api, source).await {
                Ok(svg) => svgs.push(svg),
                Err(e) => {
                    errors.push(format!("cell {}: {}", i, e));
                    svgs.push(String::new());
                }
            }
        }

        let resolved = total - errors.len();

        let resp = self
            .api
            .render(&RenderReq {
                svgs,
                background: Some(p.background),
            })
            .await
            .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;

        Ok(grid_with_summary(&resp.grid_png, total, resolved, &errors))
    }
}

#[tool_handler]
impl ServerHandler for IconServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::builder().enable_tools().build())
            .with_server_info(Implementation::new(
                "mcp-icon-visual",
                env!("CARGO_PKG_VERSION"),
            ))
    }
}

fn grid_with_urls(grid_png_b64: &str, urls: &HashMap<String, String>) -> CallToolResult {
    let mut md = String::from("| Cell | URL |\n|------|-----|\n");
    let mut keys: Vec<usize> = urls.keys().filter_map(|k| k.parse().ok()).collect();
    keys.sort();
    for k in keys {
        if let Some(url) = urls.get(&k.to_string()) {
            md.push_str(&format!("| {} | {} |\n", k, url));
        }
    }
    CallToolResult::success(vec![
        Content::image(grid_png_b64.to_string(), "image/png"),
        Content::text(md),
    ])
}

fn grid_with_summary(
    grid_png_b64: &str,
    total: usize,
    resolved: usize,
    errors: &[String],
) -> CallToolResult {
    let summary = if errors.is_empty() {
        format!("Rendered {}/{} sources", resolved, total)
    } else {
        format!(
            "Rendered {}/{} sources ({})",
            resolved,
            total,
            errors.join(", ")
        )
    };
    CallToolResult::success(vec![
        Content::image(grid_png_b64.to_string(), "image/png"),
        Content::text(summary),
    ])
}

fn svg_text(svg: &str, source: &str) -> CallToolResult {
    CallToolResult::success(vec![Content::text(format!(
        "Source: `{source}`\n\n```xml\n{svg}\n```"
    ))])
}
