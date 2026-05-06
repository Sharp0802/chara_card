use serdev::{Deserialize, Serialize};

/// Represents png-exif extension.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct PngExif {
    /// Represents title of image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    
    /// Represents description of image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    
    /// Represents software used for image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub software: Option<String>,
    
    /// Represents source of image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    
    /// Represents user comment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}
