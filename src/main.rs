mod network_scanner;  
mod network_scanner_interface;  

use color_eyre::Result;  
use crossterm::{  
    terminal::{disable_raw_mode, enable_raw_mode},  
    ExecutableCommand,  
};  
use ratatui::backend::CrosstermBackend;  
use ratatui::Terminal;  
use std::io::stdout;  

use network_scanner_interface::{scanner_interface, ScannerState};  

#[tokio::main]  
async fn main() -> Result<()> {  

    enable_raw_mode()?;  
    stdout().execute(crossterm::terminal::EnterAlternateScreen)?;  
    
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;  
    terminal.clear()?;  

    
    let mut scanner_state = ScannerState::new();  

     
    let result = scanner_interface(&mut terminal, &mut scanner_state).await;  
 
    disable_raw_mode()?;  
    stdout().execute(crossterm::terminal::LeaveAlternateScreen)?;  

    result  
}