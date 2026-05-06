use std::path::PathBuf;
use url::Url;

#[derive(Debug, Clone)]
pub enum Source {
    CCDefault,
    Embedded(PathBuf),
    File(PathBuf),
    Misc(Url),
}

impl From<Url> for Source {
    fn from(mut url: Url) -> Self {
        match url.scheme() {
            "ccdefault" => return Self::CCDefault,

            "embeded" | "embedded" => {
                let host = url.host_str().unwrap_or("");
                let path = url.path();

                let path = format!(
                    "{}{}",
                    urlencoding::decode(host).unwrap(),
                    urlencoding::decode(path).unwrap()
                );

                return Self::Embedded(path.to_string().into());
            }

            "file" => {
                if let Ok(path) = url.to_file_path() {
                    return Self::File(path);
                }
            }

            _ => {}
        }

        Self::Misc(url)
    }
}

impl Into<Url> for Source {
    fn into(self) -> Url {
        match self {
            Source::CCDefault => "ccdefault:".parse().unwrap(),
            Source::Embedded(path) => format!("embeded://{}", path.to_str().unwrap()).parse().unwrap(),
            Source::File(path) => Url::from_file_path(path).unwrap(),
            Source::Misc(url) => url,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("ccdefault:")]
    #[case("embeded://")]
    #[case("embeded://path")]
    #[case("embeded:///path")]
    #[case("embeded://path/where")]
    #[case("embeded:///path/where")]
    #[case("file:")]
    #[case("file://")]
    #[case("file://path")]
    #[case("file:///path")]
    #[case("file://path/where")]
    #[case("file:///path/where")]
    #[case("http://domain")]
    #[case("http:///path")]
    #[case("http://domain/where")]
    #[case("http:///path/where")]
    fn isomorphic_test(#[case] url: &str) {
        let url: Url = url.parse().unwrap();

        let source: Source = url.clone().into();
        let source_url: Url = source.into();

        assert_eq!(url, source_url);
    }
}
