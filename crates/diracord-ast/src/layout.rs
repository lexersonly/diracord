//! Components v2 layout: `layout { container { text "..." row { button(...) } } }`.
//!
//! A layout is a small tree of *elements*. It also accepts `local`, `if` and
//! `for`, so parts of the layout can depend on data. The interpreter turns it
//! into the same object as the "B" form (`{ layout: [...] }`), see
//! `docs/rfcs/components-v2.md`.

use crate::{Arg, Expr, Ident};
use diracord_diagnostics::Span;

/// `layout { ... }` or the body of a container/section/row.
#[derive(Clone, Debug, PartialEq)]
pub struct Layout {
    /// Items in order.
    pub items: Vec<LayoutItem>,
    /// From `{` to `}`.
    pub span: Span,
}

/// One item of a layout.
#[derive(Clone, Debug, PartialEq)]
pub struct LayoutItem {
    /// What it is.
    pub kind: LayoutItemKind,
    /// Its full extent.
    pub span: Span,
}

/// The kinds of layout items.
#[derive(Clone, Debug, PartialEq)]
pub enum LayoutItemKind {
    /// A component: `text "oi"`, `separator(spacing: "large")`, `row { ... }`.
    Element {
        /// Which component.
        kind: ElementKind,
        /// Span of the component's name.
        name_span: Span,
        /// Arguments, positional and named.
        args: Vec<Arg>,
        /// Children, for `container`, `section` and `row`.
        children: Option<Layout>,
    },
    /// `local x = value` (visible until the end of this layout block).
    Local {
        /// Name.
        name: Ident,
        /// Value.
        value: Expr,
    },
    /// `if cond { ... } else if ... else { ... }` with layout bodies.
    If {
        /// Conditions and bodies.
        branches: Vec<LayoutBranch>,
        /// Final `else`.
        otherwise: Option<Layout>,
    },
    /// `for x in list { ... }` with a layout body.
    For {
        /// Loop variable.
        var: Ident,
        /// What is iterated.
        iter: Expr,
        /// Body.
        body: Layout,
    },
}

/// One `if` / `else if` inside a layout.
#[derive(Clone, Debug, PartialEq)]
pub struct LayoutBranch {
    /// Condition.
    pub cond: Expr,
    /// Items when true.
    pub body: Layout,
    /// From `if` to the end of the body.
    pub span: Span,
}

/// The components a layout can use.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ElementKind {
    /// Box with an accent color; holds other components.
    Container,
    /// 1–3 texts with one accessory (a `thumbnail` or a `button`).
    Section,
    /// A line of up to 5 buttons, or one menu.
    Row,
    /// Markdown text.
    Text,
    /// Small image, as a section accessory.
    Thumbnail,
    /// Line or space between components.
    Separator,
    /// 1–10 images.
    Gallery,
    /// An attached file (`attachment://name`).
    File,
    /// A button (inside `row` or as a section accessory).
    Button,
    /// A select menu (inside `row`).
    Menu,
}

/// A parameter of a layout element: name and whether it is required.
pub type ElementParam = (&'static str, bool);

impl ElementKind {
    /// Every element.
    pub const ALL: [ElementKind; 10] = [
        ElementKind::Container,
        ElementKind::Section,
        ElementKind::Row,
        ElementKind::Text,
        ElementKind::Thumbnail,
        ElementKind::Separator,
        ElementKind::Gallery,
        ElementKind::File,
        ElementKind::Button,
        ElementKind::Menu,
    ];

    /// Name in code.
    pub fn as_str(self) -> &'static str {
        match self {
            ElementKind::Container => "container",
            ElementKind::Section => "section",
            ElementKind::Row => "row",
            ElementKind::Text => "text",
            ElementKind::Thumbnail => "thumbnail",
            ElementKind::Separator => "separator",
            ElementKind::Gallery => "gallery",
            ElementKind::File => "file",
            ElementKind::Button => "button",
            ElementKind::Menu => "menu",
        }
    }

    /// Parses a name.
    pub fn from_name(name: &str) -> Option<ElementKind> {
        ElementKind::ALL.into_iter().find(|k| k.as_str() == name)
    }

    /// `true` for elements written with a `{ }` body.
    pub fn has_children(self) -> bool {
        matches!(
            self,
            ElementKind::Container | ElementKind::Section | ElementKind::Row
        )
    }

    /// Parameters in positional order. Every one can also be passed by name.
    pub fn params(self) -> &'static [ElementParam] {
        match self {
            ElementKind::Container => &[("color", false), ("spoiler", false)],
            ElementKind::Section | ElementKind::Row => &[],
            ElementKind::Text => &[("content", true)],
            ElementKind::Thumbnail => &[("url", true), ("description", false), ("spoiler", false)],
            ElementKind::Separator => &[("spacing", false), ("line", false)],
            ElementKind::Gallery => &[("items", true)],
            ElementKind::File => &[("url", true), ("spoiler", false)],
            ElementKind::Button => &[
                ("id", false),
                ("label", false),
                ("style", false),
                ("emoji", false),
                ("disabled", false),
                ("url", false),
            ],
            ElementKind::Menu => &[
                ("id", true),
                ("options", true),
                ("placeholder", false),
                ("min", false),
                ("max", false),
            ],
        }
    }

    /// Can `self` be placed directly inside `parent` (`None` = top of the layout)?
    pub fn allowed_in(self, parent: Option<ElementKind>) -> bool {
        use ElementKind::*;
        match parent {
            None => matches!(
                self,
                Container | Section | Row | Text | Separator | Gallery | File
            ),
            Some(Container) => matches!(self, Section | Row | Text | Separator | Gallery | File),
            Some(Section) => matches!(self, Text | Thumbnail | Button),
            Some(Row) => matches!(self, Button | Menu),
            Some(_) => false,
        }
    }

    /// Where this element may go, in words (for error messages).
    pub fn placement_hint(self) -> &'static str {
        use ElementKind::*;
        match self {
            Container => "no topo do layout",
            Thumbnail => "dentro de `section`",
            Button => "dentro de `row` ou de `section`",
            Menu => "dentro de `row`",
            Section | Row | Text | Separator | Gallery | File => {
                "no topo do layout ou dentro de `container`"
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_roundtrip() {
        for k in ElementKind::ALL {
            assert_eq!(ElementKind::from_name(k.as_str()), Some(k));
        }
    }

    #[test]
    fn placement() {
        assert!(ElementKind::Container.allowed_in(None));
        assert!(!ElementKind::Container.allowed_in(Some(ElementKind::Container)));
        assert!(ElementKind::Thumbnail.allowed_in(Some(ElementKind::Section)));
        assert!(!ElementKind::Thumbnail.allowed_in(None));
        assert!(!ElementKind::Button.allowed_in(Some(ElementKind::Container)));
        assert!(ElementKind::Menu.allowed_in(Some(ElementKind::Row)));
    }

    #[test]
    fn required_params_come_first() {
        for k in ElementKind::ALL {
            let first_opt = k
                .params()
                .iter()
                .position(|p| !p.1)
                .unwrap_or(k.params().len());
            assert!(k.params()[first_opt..].iter().all(|p| !p.1), "{k:?}");
        }
    }
}
