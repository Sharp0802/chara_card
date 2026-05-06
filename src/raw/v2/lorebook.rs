use crate::raw::ext::Extensions;
use crate::raw::{shm, Content};
use serdev::{Deserialize, Serialize};

/// Represents lorebook (a.k.a. character book) data.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Lorebook {
    /// Represents name of lorebook.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Represents description of lorebook.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Represents scan depth of lorebook.
    ///
    /// ### Expected Behaviour
    ///
    /// If this field is presented,
    /// `keys` field in each [entry](LorebookEntry)
    /// **SHOULD** be checked with only recent this number of messages.
    ///
    /// If the context is not chat based,
    /// or if the application doesn't support chat logs,
    /// or if retrieving recent logs is impossible,
    /// the application **SHOULD** ignore this field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_depth: Option<u64>,

    /// Represents token budget.
    ///
    /// ### Expected Behaviour
    ///
    /// If this field is presented,
    /// and if total number of token exceeds this field,
    /// the application **SHOULD** omit the lorebook whose priority is lowest.
    ///
    /// If priority **and** insertion_order are not presented,
    /// or if it's impossible to determine order of priorities,
    /// or if token count cannot be determined,
    /// implementation of omitting is application-specific.
    ///
    /// ### Implementation Notes
    ///
    /// The specification doesn't specify whether omitted entries can affect other entries,
    /// Thus, to be consistent with `recursive_scanning`,
    /// Omitted entries **SHOULD NOT** be scanned by other entries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_budget: Option<u64>,

    /// Represents whether application should scan entries recursively.
    ///
    /// ### Expected Behaviour
    ///
    /// If this field is not presented,
    /// default value is application-specific.
    ///
    /// If this field is true,
    /// contents of entries already matched **MAY** be re-scanned by unmatched entries.
    /// Re-scanning process loops until there is no newly matched entry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recursive_scanning: Option<bool>,

    /// Represents application-specific extensions.
    pub extensions: Extensions,

    /// Represents entries of lorebook.
    pub entries: Vec<shm::LorebookEntry>,
}

/// Represents whether the entry is placed before or after character definitions.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum EntryPosition {
    BeforeChar,
    AfterChar,
}

/// Represents the identifier of lorebook entry.
#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
#[serde(untagged)]
pub enum Id {
    /// Numeric identifier.
    ///
    /// Until version 2, numeric identifier is only valid form of identifier.
    Number(u64),

    /// String identifier.
    ///
    /// String identifier is only valid version 3.
    /// To be compatible with older versions,
    /// Use [numeric identifier](Id::Number).
    String(String),
}

/// Represents version-2-specific data of lorebook entry.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LorebookEntry {
    /// Represents an array of activation keys.
    ///
    /// ### Expected Behaviour
    ///
    /// Entry is matched (activated) iff any item in the field is *matched* in *context*.
    ///
    /// By default, string matching is ordinal inclusion test.
    /// Currently, fields that can change string matching behaviour are:
    ///
    /// - `use_regex` in [`v3::LorebookEntry`](crate::raw::v3::LorebookEntry).
    /// - `case_sensitive` in this struct.
    ///
    /// By default, context means message logs.
    /// Regarding configuration of [`Lorebook`](Lorebook),
    /// context can be extended to another lorebooks,
    /// or be shrunken into recent messages.
    pub keys: Vec<String>,

    /// Represents content of entry.
    pub content: Content,

    /// Represents application-specific extensions.
    pub extensions: Extensions,

    /// Represents whether the entry is enabled.
    ///
    /// ### Expected Behaviour
    ///
    /// If the value is false,
    /// the entry **MUST NOT** be considered as match,
    /// regardless to whatever the other fields.
    pub enabled: bool,

    /// Represents insertion order of the entry.
    ///
    /// ### Expected Behaviour
    ///
    /// Lower value means the entry is added to prompt earlier.
    ///
    /// If this field is presented and `priority` field is not presented,
    /// The entry whose insertion order is lowest **MAY** be omitted first.
    /// See `token_budget` field in [`Lorebook`](Lorebook).
    pub insertion_order: u64,

    /// Represents whether the match should be case-sensitive.
    ///
    /// ### Expected Behaviour
    ///
    /// If this field is not presented,
    /// default value is application-specific.
    ///
    /// If the value is true,
    /// string matching of keys **SHOULD** be case-sensitive.
    /// Otherwise,
    /// string matching of keys **SHOULD** be case-insensitive.
    ///
    /// ### Implementation Notes
    ///
    /// Although the specification did not specify the default value of this field,
    /// Regarding the purpose of key matching,
    /// case-insensitive seems more proper option for key matching.
    ///
    /// Therefore, Applications **SHOULD** use false as default value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub case_sensitive: Option<bool>,

    /// Represents whether the entry is considered as matched regardless to keys.
    ///
    /// ### Expected Behaviour
    ///
    /// If `use_regex` in [`v3::LorebookEntry`](crate::raw::v3::LorebookEntry) is set to true,
    /// this field **SHOULD** be ignored.
    ///
    /// If the value is true,
    /// the entry **MUST** be considered as matched regardless to `keys` and `secondary_keys`.
    ///
    /// ### Implementation Notes
    ///
    /// Although the specification did not specify omit of constant entry,
    /// Regarding the purpose of `constant`,
    /// Constant entries seem to be non-omittable.
    ///
    /// Therefore, Until there is no more non-constant entry to omit,
    /// Applications **SHOULD NOT** omit constant entry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constant: Option<bool>,

    /// Represents name of the entry.
    ///
    /// ### Implementation Notes
    ///
    /// Although the specification notes this field **MAY** be used to identify the entry,
    /// also because the spec. notes this field is added for compatibility with *AgnAI* and *RisuAI*,
    /// it seems inappropriate that use this field as unique identifier of the entry.
    ///
    /// Therefore, Applications **SHOULD NOT** use this field as unique identifier of entry,
    /// and **MAY** use index of entry as identifier instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Represents priority of the entry.
    ///
    /// ### Implementation Notes
    ///
    /// This field basically does same thing with `insertion_order` field,
    /// and also the specification notes this field is add for compatibility with *AgnAI*.
    ///
    /// Therefore, if distribution of `insertion_order` is not too dense to order entries,
    /// Applications **SHOULD NOT** use this field to order entries.
    ///
    /// Also, Applications **SHOULD** use only `insertion_order` for new character cards.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<u64>,

    /// Represents identifier of the entry.
    ///
    /// ### Implementation Notes
    ///
    /// Although the specification notes this field **MAY** be used to identify the entry,
    /// also because the spec. notes this field is added for compatibility with *ST* and *RisuAI*,
    /// it seems inappropriate that use this field as unique identifier of the entry.
    ///
    /// Therefore, Applications **SHOULD NOT** use this field as unique identifier of entry.
    /// and **MAY** use index of entry as identifier instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Id>,

    /// Represents extra comment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Represents whether the `secondary_keys` field is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selective: Option<bool>,

    /// Represents how the entry is placed regarding the position of character definitions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<EntryPosition>,

    /// Represents an array of secondary keys.
    ///
    /// ### Expected Behaviour
    ///
    /// If `selective` is not set to true,
    /// or if `use_regex` is true,
    /// This field **SHOULD** be ignored.
    ///
    /// Otherwise,
    /// all of secondary keys **SHOULD** be matched in context to activate entry
    /// *(Note that conditions by other fields is also required)*.
    ///
    /// ### Implementation Notes
    ///
    /// Although the specification notes matching policy of secondary keys is application-specific,
    /// regarding the purpose of the secondary key,
    /// it seems appropriate that the matching policy is same between the key and the secondary key.
    ///
    /// Therefore, Applications **SHOULD** use same matching policy for keys and secondary keys.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub secondary_keys: Vec<String>,

    /// ***Non-Standard Item** (observed from artefacts of RisuAI)*
    ///
    /// Represents whether the entry is folder or regular entry.
    ///
    /// ### Observations
    ///
    /// Observed values from artefacts are `normal` and `folder`.
    ///
    /// If the value is `normal`, the entry is just standard lorebook entry.
    ///
    /// If the value is `folder`,
    /// It's observed that `keys` has only one element,
    /// and that element is used as internal identifier of folder in artefacts.
    ///
    /// ### Implementation Notes
    ///
    /// Application **SHOULD** exclude folder entries from matching target.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,

    /// ***Non-Standard Item** (observed from artefacts of RisuAI)*
    ///
    /// Represents what folder the entry belongs to.
    ///
    /// ### Observations
    ///
    /// Observed values are an element of `keys` in folder entries.
    ///
    /// It seems entries struct filesystem-like hierarchy,
    /// So this field can be not presented for
    /// top entries (who doesn't belong to some folder).
    ///
    /// ### Implementation Notes
    ///
    /// Applications **MAY** implement hierarchy structure of entries
    /// to support this system.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder: Option<String>,
}
