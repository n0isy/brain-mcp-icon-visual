use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetSvgParams {
    /// SVG source. Accepts:
    /// - URL from search results: "https://mdn.alipayobjects.com/…/original"
    /// - Iconify ID: "mdi/cloud", "lucide/home", "tabler/database"
    /// - Absolute file path: "/home/user/project/icons/logo.svg"
    /// - Inline SVG: "<svg xmlns='…'>…</svg>"
    /// Always use absolute paths for local files.
    pub source: String,
}
