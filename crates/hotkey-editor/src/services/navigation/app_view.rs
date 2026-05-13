/// Which top-level page the app should render.
///
/// Backed by the `?view=` URL parameter so deep-linking and browser
/// back/forward both work.  Each variant maps 1:1 to a query value:
///
/// - `Editor`          → `?view=editor`   (default if missing/invalid)
/// - `Collisions { kind }` → `?view=collisions&kind=positions|hotkeys`
/// - `Resolve`         → `?view=resolve`
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum AppView {
    Editor,
    Collisions { kind: CollisionKind },
    Resolve,
}

/// Which sub-page the collisions view should render.
///
/// Read from the `?kind=` URL parameter.  Defaults to `Positions` when
/// `kind` is missing or unrecognized.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum CollisionKind {
    Positions,
    Hotkeys,
}

impl AppView {
    /// Canonical default when no view is selected.
    pub(crate) fn default_view() -> Self {
        Self::Editor
    }

    /// Builds an `AppView` from the raw `view` and `kind` query strings.
    /// Unknown values fall back to the editor.
    #[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
    pub(crate) fn from_query_params(view_param: Option<&str>, kind_param: Option<&str>) -> Self {
        let view_value = view_param.unwrap_or("editor");
        match view_value {
            "collisions" => {
                let kind = CollisionKind::from_query_param(kind_param);
                Self::Collisions { kind }
            }
            "resolve" => Self::Resolve,
            _ => Self::Editor,
        }
    }

    /// The string written to the `view` URL parameter.
    #[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
    pub(crate) fn view_param(&self) -> &'static str {
        match self {
            Self::Editor => "editor",
            Self::Collisions { .. } => "collisions",
            Self::Resolve => "resolve",
        }
    }

    /// The string written to the `kind` URL parameter, if applicable.
    /// `None` for views without a sub-kind.
    #[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
    pub(crate) fn kind_param(&self) -> Option<&'static str> {
        match self {
            Self::Collisions { kind } => Some(kind.kind_param()),
            Self::Editor | Self::Resolve => None,
        }
    }
}

impl CollisionKind {
    #[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
    fn from_query_param(kind_param: Option<&str>) -> Self {
        match kind_param.unwrap_or("positions") {
            "hotkeys" => Self::Hotkeys,
            _ => Self::Positions,
        }
    }

    #[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
    fn kind_param(self) -> &'static str {
        match self {
            Self::Positions => "positions",
            Self::Hotkeys => "hotkeys",
        }
    }
}

#[cfg(test)]
mod app_view_tests {
    use super::*;

    #[test]
    fn default_query_yields_editor() {
        let view = AppView::from_query_params(None, None);
        assert_eq!(view, AppView::Editor);
    }

    #[test]
    fn collisions_with_positions_kind_parses_correctly() {
        let view = AppView::from_query_params(Some("collisions"), Some("positions"));
        let expected = AppView::Collisions {
            kind: CollisionKind::Positions,
        };
        assert_eq!(view, expected);
    }

    #[test]
    fn collisions_with_hotkeys_kind_parses_correctly() {
        let view = AppView::from_query_params(Some("collisions"), Some("hotkeys"));
        let expected = AppView::Collisions {
            kind: CollisionKind::Hotkeys,
        };
        assert_eq!(view, expected);
    }

    #[test]
    fn collisions_without_kind_defaults_to_positions() {
        let view = AppView::from_query_params(Some("collisions"), None);
        let expected = AppView::Collisions {
            kind: CollisionKind::Positions,
        };
        assert_eq!(view, expected);
    }

    #[test]
    fn resolve_parses_correctly() {
        let view = AppView::from_query_params(Some("resolve"), None);
        assert_eq!(view, AppView::Resolve);
    }

    #[test]
    fn unknown_view_falls_back_to_editor() {
        let view = AppView::from_query_params(Some("nonsense"), None);
        assert_eq!(view, AppView::Editor);
    }

    #[test]
    fn unknown_kind_falls_back_to_positions() {
        let view = AppView::from_query_params(Some("collisions"), Some("nonsense"));
        let expected = AppView::Collisions {
            kind: CollisionKind::Positions,
        };
        assert_eq!(view, expected);
    }

    #[test]
    fn view_param_round_trips() {
        let editor = AppView::Editor;
        let collisions_positions = AppView::Collisions {
            kind: CollisionKind::Positions,
        };
        let collisions_hotkeys = AppView::Collisions {
            kind: CollisionKind::Hotkeys,
        };
        let resolve = AppView::Resolve;

        assert_eq!(editor.view_param(), "editor");
        assert_eq!(collisions_positions.view_param(), "collisions");
        assert_eq!(collisions_hotkeys.view_param(), "collisions");
        assert_eq!(resolve.view_param(), "resolve");

        assert_eq!(editor.kind_param(), None);
        assert_eq!(collisions_positions.kind_param(), Some("positions"));
        assert_eq!(collisions_hotkeys.kind_param(), Some("hotkeys"));
        assert_eq!(resolve.kind_param(), None);
    }
}
