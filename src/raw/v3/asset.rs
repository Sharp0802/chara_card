use serdev::{Deserialize, Serialize};
use url::Url;

/// Represents asset definition in character card
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Asset {
    /// Represents type of asset.
    ///
    /// ### Expected Behaviour
    ///
    /// Applications **SHOULD** support the following standard types:
    ///
    /// - `icon` : An icon or portrait of the character.
    /// - `background` : A background image associated with the character card.
    /// - `user_icon` : An icon or portrait of the user/player.
    /// - `emotion` : An expression-specific image of character (e.g., "happy", "sad").
    ///
    /// The implementation and rendering logic for `emotion` assets is application-specific.
    pub r#type: String,

    /// Represents URI of asset.
    ///
    /// ### Expected Behaviour
    ///
    /// Applications **SHOULD** implement the following internal schemas:
    ///
    /// - `ccdefault://` : Points to an application-specific default fallback image.
    /// - `embeded://` : Points to a relative file path within the `.charx` archive.
    ///   - *(Note: The single `d` in `embeded` is intentional by the specification.)*
    ///
    /// Applications **MAY** support additional schemas such as `file://`, `http(s)://`, or `ftp://`.
    ///
    /// > [!CAUTION]
    /// > **Security Risk**: Implementing external schemas
    /// > (e.g., `file://`, `http(s)://`, etc.)
    /// > significantly increases the application's attack surface.
    /// >
    /// > See [Implementation Safety](#Implementation-Safety) section.
    ///
    /// ### Implementation Safety
    ///
    /// To prevent unauthorized data exfiltration or local file access:
    ///
    /// 1. Applications **SHOULD** prompt the user for permission before accessing external sources.
    /// 2. At minimum, one security warning **SHOULD** be displayed per character card before
    ///    any external network or filesystem requests are initiated.
    pub uri: Url,

    /// Represents name of asset.
    ///
    /// ### Implementation Notes
    ///
    /// It cannot be directly used as unique identifier of asset.
    /// For details, see `assets` field
    /// in [`v3::CharacterCardData`](crate::raw::v3::CharacterCardData).
    pub name: String,

    /// Represents extension of asset (e.g., `png`, `webp`).
    ///
    /// This value **MUST NOT** include a leading period (`.`).
    pub ext: String,
}
