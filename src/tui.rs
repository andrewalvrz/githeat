use crate::heatmap::Heatmap;
use ratatui::{prelude::*, widgets::{Block, Borders, List, ListItem,}};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};

pub fn render_heatmap(
    data: &Heatmap,
    by_author: bool,
    top: Option<usize>,
    sort: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    let backend = CrosstermBackend::new(&mut stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|f| {
        let size = f.size();
        let title = if by_author {
            "Git Heatmap — Author Mode"
        } else {
            "Git Heatmap — File Mode"
        };

        let block = Block::default().title(title).borders(Borders::ALL);

        let mut entries: Vec<_> = if !data.file_counts.is_empty() {
            data.file_counts.iter().collect()
        } else {
            data.author_counts.iter().collect()
        };

        if let Some(order) = sort {
            if order == "asc" {
                entries.sort_by(|a, b| a.1.cmp(b.1));
            } else {
                entries.sort_by(|a, b| b.1.cmp(a.1));
            }
        } else {
            entries.sort_by(|a, b| b.1.cmp(a.1));
        }

        if let Some(limit) = top {
            entries.truncate(limit);
        }

        let items: Vec<ListItem> = entries
            .iter()
            .map(|(key, count)| {
                let bar = "█".repeat((*count / 3).max(1));
                ListItem::new(format!("{:<40} {} ({})", key, bar, count))
            })
            .collect();

        let list = List::new(items).block(block);
        f.render_widget(list, size);
    })?;

    disable_raw_mode()?;
    Ok(())
}
