use crate::heatmap::Heatmap;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem},
};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};

pub fn render_heatmap(data: &Heatmap) -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    let backend = CrosstermBackend::new(&mut stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|f| {
        let size = f.size();

        let items: Vec<ListItem> = if !data.file_counts.is_empty() {
            data.file_counts
                .iter()
                .map(|(file, count)| {
                    let bar = "█".repeat((*count / 5).max(1));
                    ListItem::new(format!("{:<40} {}", file, bar))
                })
                .collect()
        } else {
            data.author_counts
                .iter()
                .map(|(author, count)| {
                    ListItem::new(format!("{:<20} {} commits", author, count))
                })
                .collect()
        };

        let block = Block::default().title("Git Heatmap").borders(Borders::ALL);
        let list = List::new(items).block(block);
        f.render_widget(list, size);
    })?;

    disable_raw_mode()?;
    Ok(())
}
