use crate::network_scanner::{scan_target, ScanResult};  
use color_eyre::Result;  
use crossterm::event::{self, Event, KeyCode, KeyEventKind};  
use ratatui::{  
    backend::Backend,  
    layout::{Constraint, Direction, Layout},  
    style::{Color, Modifier, Style},  
    text::{Line, Span, Text},  
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},  
    Frame,  
};  

#[derive(Debug, Default)]  
pub struct ScannerState {  
    pub target: String,  
    pub scan_result: Vec<ScanResult>,  
    pub list_state: ListState,  
}  

impl ScannerState {  
    pub fn new() -> Self {  
        Self::default()  
    }  

    pub async fn scan(&mut self) {  
        if self.target.is_empty() {  
            return;  
        }  

        self.list_state = ListState::default();  
        self.scan_result = scan_target(&self.target).await;  
    }  

    pub fn draw(&mut self, f: &mut Frame) {  
        let chunks = Layout::default()  
            .direction(Direction::Vertical)  
            .constraints([  
                Constraint::Length(3),  
                Constraint::Length(3),  
                Constraint::Min(10),  
                Constraint::Length(3),  
            ])  
            .split(f.area());  

        let title = Paragraph::new(Text::from("Network Port Scanner"))  
            .block(Block::default().borders(Borders::ALL).title("Network Scanner"))  
            .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))  
            .alignment(ratatui::layout::Alignment::Center);  
        f.render_widget(title, chunks[0]);  

        let input = Paragraph::new(Text::from(self.target.as_str()))  
            .block(Block::default().borders(Borders::ALL).title("Target"))  
            .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));  
        f.render_widget(input, chunks[1]);  

        let result_items: Vec<ListItem> = self.scan_result.iter()  
            .map(|result| {  
                let status = if result.open { "Open" } else { "Closed" };  
                let content = format!("Port {}: {}", result.port, status);  

                let style = if result.open {  
                    Style::default().fg(Color::Green)  
                } else {  
                    Style::default().fg(Color::Red)  
                };  

                ListItem::new(Line::from(vec![Span::styled(content, style)]))  
            })  
            .collect();  

        let result_list = List::new(result_items)  
            .block(Block::default().borders(Borders::ALL).title("Scan Results"))  
            .highlight_style(Style::default()  
                .add_modifier(Modifier::BOLD)  
                .fg(Color::Cyan))  
            .highlight_symbol("> ");  

        f.render_stateful_widget(result_list, chunks[2], &mut self.list_state);  

        let footer = Paragraph::new(Text::from("Press ESC to exit | Enter to scan"))  
            .block(Block::default().borders(Borders::ALL))  
            .style(Style::default().fg(Color::Gray));  
        f.render_widget(footer, chunks[3]);  
    }  
}  

pub async fn scanner_interface(  
    terminal: &mut ratatui::Terminal<impl Backend>,   
    scanner_state: &mut ScannerState  
) -> Result<()> {  
    loop {  
        terminal.draw(|f| scanner_state.draw(f))?;  

        if let Event::Key(key) = event::read()? {  
            if key.kind != KeyEventKind::Press {  
                continue;  
            }  

            match key.code {  
                KeyCode::Esc => break Ok(()),  
                KeyCode::Char(c) => {  
                    scanner_state.target.push(c);  
                }  
                KeyCode::Backspace => {  
                    scanner_state.target.pop();  
                }  
                KeyCode::Enter => {  
                    scanner_state.scan().await;  
                }  
                KeyCode::Up => {  
                    if !scanner_state.scan_result.is_empty() {  
                        scanner_state.list_state.select_previous();  
                    }  
                }  
                KeyCode::Down => {  
                    if !scanner_state.scan_result.is_empty() {  
                        scanner_state.list_state.select_next();  
                    }  
                }  
                _ => {}  
            }  
        }  
    }  
}