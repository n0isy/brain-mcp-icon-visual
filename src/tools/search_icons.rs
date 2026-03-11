use schemars::JsonSchema;
use serde::Deserialize;

fn default_bg() -> String {
    "#FFFFFF".into()
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SearchIconsParams {
    /// Semantic search query. Examples: "data analysis", "cloud computing", "arrows".
    pub keyword: String,

    /// Grid background color, CSS hex. Default: "#FFFFFF".
    #[serde(default = "default_bg")]
    pub background: String,

    /// Recolor all icons to this CSS hex color. Null keeps original colors.
    #[serde(default)]
    pub color: Option<String>,
}
