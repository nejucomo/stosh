use ratatui::style::Color::{Black, Cyan, DarkGray, Gray, Green, LightRed, Red, White, Yellow};
use ratatui::style::Style;
use ratatui::widgets::BorderType;

pub(crate) const STYLES: Styles = Styles {
    text: TextStyles {
        histix: Style::new().fg(White).bg(Black),
        input: Style::new().fg(Gray).bg(DarkGray).underline_color(Cyan),
        stdout: Style::new().fg(Gray).bg(Black),
        stderr: Style::new().fg(Yellow).bg(Black),
    },
    status: StatusStyles {
        spawning: Style::new().fg(White).bg(DarkGray),
        spawn_err: Style::new().fg(Red).bg(DarkGray),
        running: Style::new().fg(White).bg(Black),
        exit_success: Style::new().fg(Green).bg(Black),
        exit_error: Style::new().fg(LightRed).bg(Black),
    },
    border: BorderStyles {
        input: BorderStyle {
            btype: BorderType::Double,
            style: Style::new().fg(Gray).bg(Black),
        },
        view: BorderStyle {
            btype: BorderType::Rounded,
            style: Style::new().fg(DarkGray).bg(Black),
        },
    },
};

#[derive(Debug)]
pub(crate) struct Styles {
    pub(crate) text: TextStyles,
    pub(crate) status: StatusStyles,
    pub(crate) border: BorderStyles,
}

#[derive(Debug)]
pub(crate) struct TextStyles {
    pub(crate) histix: Style,
    pub(crate) input: Style,
    pub(crate) stdout: Style,
    pub(crate) stderr: Style,
}

#[derive(Debug)]
pub(crate) struct StatusStyles {
    pub(crate) spawning: Style,
    pub(crate) spawn_err: Style,
    pub(crate) running: Style,
    pub(crate) exit_success: Style,
    pub(crate) exit_error: Style,
}

#[derive(Debug)]
pub(crate) struct BorderStyles {
    pub(crate) input: BorderStyle,
    pub(crate) view: BorderStyle,
}

#[derive(Debug)]
pub(crate) struct BorderStyle {
    pub(crate) btype: BorderType,
    pub(crate) style: Style,
}
