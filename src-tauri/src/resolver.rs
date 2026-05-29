use std::path::{Path, PathBuf};

use url::Url;

use crate::error::ConvoError;

/// A resolved conversation target: the file to read plus an optional turn anchor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedTarget {
    pub path: PathBuf,
    pub anchor: Option<String>,
}

/// Resolve a `convo://claude-code/<encoded-project>/<uuid>` URL (optionally with a
/// `#<anchor>` fragment) against the given projects root.
///
/// The projects root is passed in (rather than read from the environment) so this
/// function is pure and unit-testable.
pub fn resolve(raw: &str, projects_root: &Path) -> Result<ResolvedTarget, ConvoError> {
    let url = Url::parse(raw).map_err(|e| ConvoError::InvalidUrl(e.to_string()))?;

    if url.scheme() != "convo" {
        return Err(ConvoError::InvalidUrl(format!(
            "expected convo:// scheme, got {}://",
            url.scheme()
        )));
    }
    if url.host_str() != Some("claude-code") {
        return Err(ConvoError::InvalidUrl(format!(
            "unsupported host: {:?}",
            url.host_str()
        )));
    }

    let segments: Vec<&str> = url
        .path_segments()
        .map(|s| s.filter(|seg| !seg.is_empty()).collect())
        .unwrap_or_default();

    // Extra guard: reject if any segment contains path-traversal sequences.
    // The url crate may normalize %2f-encoded slashes or strip ".." dot-segments
    // during parsing, which can cause the segment count to change (e.g. a
    // percent-encoded "../etc/passwd" may be decoded and collapsed so only one
    // non-empty segment survives). We therefore check *before* the slice match
    // so that such inputs hit the traversal error path rather than the
    // "too few segments" path — and, critically, never yield a valid path.
    for seg in &segments {
        if seg.contains('/') || seg.contains("..") || seg.contains('\\') {
            return Err(ConvoError::PathTraversal(seg.to_string()));
        }
    }

    let [project, uuid] = segments.as_slice() else {
        return Err(ConvoError::InvalidUrl(
            "expected path of form /<project>/<uuid>".to_string(),
        ));
    };

    let candidate = projects_root.join(project).join(format!("{uuid}.jsonl"));

    // Defense in depth: the candidate's parent must be exactly the projects root.
    if candidate.parent().and_then(Path::parent) != Some(projects_root) {
        return Err(ConvoError::PathTraversal(candidate.display().to_string()));
    }

    let anchor = url.fragment().map(|f| f.to_string());

    Ok(ResolvedTarget {
        path: candidate,
        anchor,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn root() -> PathBuf {
        PathBuf::from("/home/u/.claude/projects")
    }

    #[test]
    fn resolves_basic_url() {
        let r = resolve("convo://claude-code/-home-u-proj/abc-123", &root()).unwrap();
        assert_eq!(
            r.path,
            PathBuf::from("/home/u/.claude/projects/-home-u-proj/abc-123.jsonl")
        );
        assert_eq!(r.anchor, None);
    }

    #[test]
    fn captures_anchor_fragment() {
        let r = resolve("convo://claude-code/-home-u-proj/abc-123#turn-9", &root()).unwrap();
        assert_eq!(r.anchor.as_deref(), Some("turn-9"));
    }

    #[test]
    fn rejects_wrong_scheme() {
        assert!(matches!(
            resolve("https://claude-code/x/y", &root()),
            Err(ConvoError::InvalidUrl(_))
        ));
    }

    #[test]
    fn rejects_unknown_host() {
        assert!(matches!(
            resolve("convo://web-export/x/y", &root()),
            Err(ConvoError::InvalidUrl(_))
        ));
    }

    #[test]
    fn rejects_traversal_in_uuid() {
        let res = resolve("convo://claude-code/proj/..%2f..%2fetc%2fpasswd", &root());
        assert!(res.is_err());
    }

    #[test]
    fn rejects_too_few_segments() {
        assert!(matches!(
            resolve("convo://claude-code/onlyproject", &root()),
            Err(ConvoError::InvalidUrl(_))
        ));
    }
}
