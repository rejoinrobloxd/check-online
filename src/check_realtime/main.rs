use rfd::FileDialog;
use std::fs;
use std::io::Write;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use super::data::RealtimeData;
use super::ui::render;
use super::utils::update_hardware_info;

pub async fn check_realtime() -> Result<(), Box<dyn std::error::Error>> {
    let file_dialog = FileDialog::new()
        .add_filter("Text files", &["txt"])
        .set_directory("/");

    let cookie_file_path = match file_dialog.pick_file() {
        Some(path) => path,
        None => {
            println!("No file selected.");
            return Ok(());
        }
    };

    let content = fs::read_to_string(cookie_file_path)?;
    let cookies_content: Vec<String> = if content.contains("\n\n") {
        content.split("\n\n").filter(|s| !s.trim().is_empty()).map(|s| s.replace("\n", "")).collect()
    } else {
        content.lines().filter(|s| !s.trim().is_empty()).map(|s| s.to_string()).collect()
    };


    use std::io::{self, stdin};
    loop {
        print!("Enter re-check interval in seconds (60-360): ");
        io::stdout().flush()?;
        let mut input = String::new();
        stdin().read_line(&mut input)?;
        let interval = input.trim().parse::<u32>();
        
        match interval {
            Ok(val) if val >= 60 && val <= 360 => {
                let data = Arc::new(RealtimeData::new(cookies_content, val));
                enable_raw_mode()?;
                let mut stdout = std::io::stdout();
                execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
                let backend = CrosstermBackend::new(stdout);
                let mut terminal = Terminal::new(backend)?;
                let data_clone = Arc::clone(&data);
                data_clone.check_all().await?;
                update_hardware_info(&data).await;
                let data_countdown = Arc::clone(&data);
                let countdown_handle = tokio::spawn(async move {
                    loop {
                        sleep(Duration::from_secs(1)).await;
                        let mut countdown = data_countdown.countdown.lock().unwrap();
                        let interval = *data_countdown.check_interval.lock().unwrap();
                        if *countdown > 0 {
                            *countdown -= 1;
                        } else {
                            *countdown = interval;
                        }
                    }
                });

                let data_timer = Arc::clone(&data);
                let check_handle = tokio::spawn(async move {
                    loop {
                        let interval = *data_timer.check_interval.lock().unwrap();
                        sleep(Duration::from_secs(interval as u64)).await;
                        if let Err(e) = data_timer.check_all().await {
                            eprintln!("Error during check: {}", e);
                        }
                    }
                });

                let data_hardware = Arc::clone(&data);
                let hardware_handle = tokio::spawn(async move {
                    loop {
                        sleep(Duration::from_secs(2)).await;
                        update_hardware_info(&data_hardware).await;
                    }
                });
                loop {
                    terminal.draw(|f| render(f, &data))?;

                    if crossterm::event::poll(Duration::from_millis(100))? {
                        if let Event::Key(key) = event::read()? {
                            if key.kind == KeyEventKind::Press {
                                match key.code {
                                    KeyCode::Char('q') | KeyCode::Esc => {
                                        break;
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                }

                countdown_handle.abort();
                check_handle.abort();
                hardware_handle.abort();

                let _ = disable_raw_mode();
                let mut stdout = std::io::stdout();
                let _ = execute!(
                    stdout,
                    LeaveAlternateScreen,
                    DisableMouseCapture,
                    crossterm::cursor::Show,
                    crossterm::cursor::MoveTo(0, 0)
                );
                let _ = stdout.flush();
                
                drop(terminal);
                

                use crossterm::terminal::is_raw_mode_enabled;
                if is_raw_mode_enabled().unwrap_or(false) {
                    let _ = disable_raw_mode();
                }
                

                execute!(
                    std::io::stdout(),
                    crossterm::terminal::Clear(crossterm::terminal::ClearType::All),
                    crossterm::cursor::MoveTo(0, 0),
                    crossterm::cursor::Show
                ).ok();
                std::io::stdout().flush().ok();
                
                println!();
                println!();
                return Ok(());
            }
            Ok(val) => { println!("Invalid input: {} is not in range 60-360. Please try again.", val); }
            Err(_) => { println!("Invalid input: Please enter a number between 60 and 360."); }
        }
    }
}
