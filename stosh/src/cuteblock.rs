use derive_new::new;
// TODO: Export this into a standalone crate
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Style, Styled as _};
use ratatui::symbols::border::Set;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Padding, Widget};
use ratatui_rseq::Renderable;

#[derive(Debug, Default, new)]
pub struct CuteBlock<'a> {
    top_titles: Vec<Line<'a>>,
    bottom_titles: Vec<Line<'a>>,
    bstyle: Style,
    btype: BorderType,
    inner: Block<'a>,
}

impl<'a> CuteBlock<'a> {
    pub fn bordered() -> Self {
        CuteBlock {
            inner: Block::bordered(),

            ..CuteBlock::default()
        }
    }

    pub fn title_top<T: Into<Line<'a>>>(mut self, title: T) -> Self {
        self.top_titles.push(title.into());
        self
    }

    pub fn title_bottom<T: Into<Line<'a>>>(mut self, title: T) -> Self {
        self.bottom_titles.push(title.into());
        self
    }

    pub fn border_style<S: Into<Style>>(self, style: S) -> Self {
        self.map_inner(|b| b.border_style(style))
    }

    pub fn style<S: Into<Style>>(self, style: S) -> Self {
        self.map_inner(|b| b.style(style))
    }

    pub fn borders(self, flag: Borders) -> Self {
        self.map_inner(|b| b.borders(flag))
    }

    pub fn border_type(self, border_type: BorderType) -> Self {
        self.map_inner(|b| b.border_type(border_type))
    }

    pub fn border_set(self, border_set: Set) -> Self {
        self.map_inner(|b| b.border_set(border_set))
    }

    pub fn padding(self, padding: Padding) -> Self {
        self.map_inner(|b| b.padding(padding))
    }

    pub fn into_block(self) -> Block<'a> {
        self.into()
    }

    fn map_inner<F>(mut self, f: F) -> Self
    where
        F: FnOnce(Block<'a>) -> Block<'a>,
    {
        self.inner = f(self.inner);
        self
    }
}

impl<'a> From<CuteBlock<'a>> for Block<'a> {
    fn from(cb: CuteBlock<'a>) -> Self {
        use BorderType::*;

        let CuteBlock {
            top_titles,
            bottom_titles,
            bstyle,
            btype,
            inner,
        } = cb;

        let (decl, decr) = match btype {
            Plain | Rounded => ("┨", "┠"),
            Double => ("╡", "╞"),
            Thick => ("┥", "┝"),
            QuadrantInside => ("▟", "▙"),
            QuadrantOutside => ("▜", "▛"),
        };
        let decl = &decl.set_style(bstyle);
        let decr = &decr.set_style(bstyle);

        fn decorate<'a>(mut line: Line<'a>, left: &Span<'a>, right: &Span<'a>) -> Line<'a> {
            line.spans.insert(0, left.clone());
            line.spans.push(right.clone());
            line
        }

        let inner = top_titles
            .into_iter()
            .fold(inner, |b, t| b.title_top(decorate(t, decl, decr)));

        bottom_titles
            .into_iter()
            .fold(inner, |b, t| b.title_bottom(decorate(t, decl, decr)))
    }
}

impl<'a> Renderable for CuteBlock<'a> {
    fn into_widget(self) -> impl Widget {
        Block::from(self)
    }
}

impl<'a> Widget for CuteBlock<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.into_widget().render(area, buf)
    }
}
