/// This module is responsible for rendering the "Bookmarked Patchsets" screen.
///
/// This module displays a list of locally saved patches that the user has bookmarked
/// This screen allows users to navigate through saved patches and select them for details.
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, HighlightSpacing, List, ListItem, ListState},
    Frame,
};

use crate::app::screens::bookmarked::BookmarkedPatchsets;

/// Renders the main list of bookmarked patchsets with formatting and selection highlighting.
pub fn render_main(f: &mut Frame, bookmarked_patchsets: &BookmarkedPatchsets, chunk: Rect) {
    let patchset_index = bookmarked_patchsets.patchset_index;
    let mut list_items = Vec::<ListItem>::new();

    for (index, patch) in bookmarked_patchsets.bookmarked_patchsets.iter().enumerate() {
        let patch_title = format!("{:width$}", patch.title(), width = 70);
        let patch_title = format!("{:.width$}", patch_title, width = 70);
        let patch_author = format!("{:width$}", patch.author().name, width = 30);
        let patch_author = format!("{:.width$}", patch_author, width = 30);
        list_items.push(ListItem::new(
            Line::from(Span::styled(
                format!(
                    "{:03}. V{:02} | #{:02} | {} | {}",
                    index,
                    patch.version(),
                    patch.total_in_series(),
                    patch_title,
                    patch_author
                ),
                Style::default().fg(Color::Yellow),
            ))
            .centered(),
        ));
    }

    let list_block = Block::default()
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Double)
        .style(Style::default());

    let list = List::new(list_items)
        .block(list_block)
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .add_modifier(Modifier::REVERSED)
                .fg(Color::Cyan),
        )
        .highlight_symbol(">")
        .highlight_spacing(HighlightSpacing::Always);

    let mut list_state = ListState::default();
    list_state.select(Some(patchset_index));

    f.render_stateful_widget(list, chunk, &mut list_state);
}

/// Returns the styled text to be displayed in the footer for this screen.
pub fn mode_footer_text() -> Vec<Span<'static>> {
    vec![Span::styled(
        "Bookmarked Patchsets",
        Style::default().fg(Color::Green),
    )]
}

/// Returns the styled help text showing available keybindings.
pub fn keys_hint() -> Span<'static> {
    Span::styled(
        "(ESC / q) to return | (ENTER) to select | (?) help",
        Style::default().fg(Color::Red),
    )
}
