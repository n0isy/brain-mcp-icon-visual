use schemars::JsonSchema;
use serde::Deserialize;

fn default_bg() -> String {
    "#FFFFFF".into()
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct RenderGridParams {
    /// 1-16 SVG sources placed into grid cells 0-15. Each element accepts:
    /// - URL from search results: "https://mdn.alipayobjects.com/…/original"
    /// - Iconify ID: "mdi/cloud"
    /// - Absolute file path: "/home/user/project/icons/logo.svg"
    /// - Inline SVG: "<svg>…</svg>"
    /// Always use absolute paths for local files.
    pub sources: Vec<String>,

    /// Grid background color, CSS hex. Default: "#FFFFFF".
    #[serde(default = "default_bg")]
    pub background: String,
}
