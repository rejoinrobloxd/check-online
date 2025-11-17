use ratatui::{
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};
use std::sync::Arc;
use super::data::RealtimeData;

pub fn render(f: &mut Frame, data: &Arc<RealtimeData>) {
    let main_chunks = Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(f.size());

    let left_chunks = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([Constraint::Percentage(33), Constraint::Percentage(33), Constraint::Percentage(34)].as_ref())
        .split(main_chunks[0]);

    let stats = data.stats.lock().unwrap();
    let countdown = data.countdown.lock().unwrap();
    let stats_text = vec![
        Line::from(vec![Span::styled("LIVE: ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)), Span::from(format!("{}", stats.live))]),
        Line::from(vec![Span::styled("DEAD: ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)), Span::from(format!("{}", stats.dead))]),
        Line::from(vec![Span::styled("BANNED: ", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)), Span::from(format!("{}", stats.banned))]),
        Line::from(vec![Span::styled("ERROR: ", Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD)), Span::from(format!("{}", stats.error))]),
        Line::from(vec![Span::styled("DUPLICATE: ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)), Span::from(format!("{}", stats.duplicates))]),
        Line::from(""),
        Line::from(vec![Span::styled("Next check in: ", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)), Span::styled(format!("{}s", countdown), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))]),
        Line::from(""),
        Line::from(Span::styled("Press 'q' or ESC", Style::default().fg(Color::Gray))),
    ];
    let stats_block = Block::default()
        .borders(Borders::ALL)
        .title("Statistics");
    let stats_paragraph = Paragraph::new(stats_text)
        .block(stats_block);
    f.render_widget(stats_paragraph, left_chunks[0]);

    let presence = data.presence.lock().unwrap();
    let presence_text = vec![
        Line::from(vec![Span::styled("ONLINE: ", Style::default().fg(Color::Rgb(144, 238, 144)).add_modifier(Modifier::BOLD)), Span::from(format!("{}", presence.online))]),
        Line::from(vec![Span::styled("OFFLINE: ", Style::default().fg(Color::Rgb(64, 64, 64)).add_modifier(Modifier::BOLD)), Span::from(format!("{}", presence.offline))]),
        Line::from(vec![Span::styled("IN-GAME: ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)), Span::from(format!("{}", presence.in_game))]),
        Line::from(vec![Span::styled("IN-STUDIO: ", Style::default().fg(Color::Rgb(255, 165, 0)).add_modifier(Modifier::BOLD)), Span::from(format!("{}", presence.in_studio))]),
    ];
    let presence_block = Block::default()
        .borders(Borders::ALL)
        .title("Presence");
    let presence_paragraph = Paragraph::new(presence_text)
        .block(presence_block);
    f.render_widget(presence_paragraph, left_chunks[1]);

    let hardware = data.hardware.lock().unwrap();
    let ram_used_mb = hardware.ram_used / (1024 * 1024);
    let ram_total_mb = hardware.ram_total / (1024 * 1024);
    let ram_percent = if hardware.ram_total > 0 {
        (hardware.ram_used as f64 / hardware.ram_total as f64) * 100.0
    } else {
        0.0
    };
    
    let cpu_usage_color = if hardware.cpu_usage > 80.0 {
        Color::Red
    } else if hardware.cpu_usage > 50.0 {
        Color::Yellow
    } else {
        Color::Green
    };
    
    let mut hardware_text = vec![
        Line::from(vec![Span::styled("CPU: ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)), Span::from(if hardware.cpu_name.is_empty() { "N/A" } else { &hardware.cpu_name })]),
        Line::from(vec![Span::styled("CPU Usage: ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)), Span::styled(format!("{:.1}%", hardware.cpu_usage), Style::default().fg(cpu_usage_color))]),
        Line::from(vec![Span::styled("RAM: ", Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD)), Span::from(format!("{} MB / {} MB", ram_used_mb, ram_total_mb))]),
        Line::from(vec![Span::styled("RAM Usage: ", Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD)), Span::from(format!("{:.1}%", ram_percent))]),
    ];
    
    if let Some(ping) = hardware.ping {
        hardware_text.push(Line::from(""));
        hardware_text.push(Line::from(vec![Span::styled("Ping: ", Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD)), Span::from(format!("{} ms", ping))]));
    } else {
        hardware_text.push(Line::from(""));
        hardware_text.push(Line::from(vec![Span::styled("Ping: ", Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD)), Span::from("N/A")]));
    }
    
    let hardware_block = Block::default()
        .borders(Borders::ALL)
        .title("Hardware Info");
    let hardware_paragraph = Paragraph::new(hardware_text)
        .block(hardware_block);
    f.render_widget(hardware_paragraph, left_chunks[2]);
    let logs = data.logs.lock().unwrap();
    let start_idx = if logs.len() > 50 {
        logs.len() - 50
    } else {
        0
    };
    let log_items: Vec<ListItem> = logs.iter()
        .skip(start_idx)
        .rev()
        .map(|log| {
            let mut spans = vec![];
            if let Some(timestamp_end) = log.find("] ") {
                let timestamp = &log[..timestamp_end + 1];
                spans.push(Span::styled(timestamp.to_string(), Style::default().fg(Color::Gray)));
                spans.push(Span::raw(" "));
                let remaining = &log[timestamp_end + 2..];
                if let Some(tag_end) = remaining.find("]") {
                    let tag = &remaining[..tag_end + 1];
                    let message = remaining[tag_end + 1..].trim_start();
                    let tag_color = if tag == "[LIVE]" {Color::Green} 
                    else if tag == "[DEAD]" { Color::Yellow} 
                    else if tag == "[BANNED]" {Color::Red} 
                    else if tag == "[ERROR]" {Color::Magenta} 
                    else if tag == "[DUPLICATE]" {Color::Cyan} 
                    else {Color::White};
                    
                    spans.push(Span::styled(tag.to_string(), Style::default().fg(tag_color).add_modifier(Modifier::BOLD)));
                    if !message.is_empty() {
                        spans.push(Span::raw(" "));
                        spans.push(Span::raw(message));
                    }
                } 
                else {spans.push(Span::raw(remaining));}
            } 
            else {spans.push(Span::raw(log.clone()));}
            
            ListItem::new(vec![Line::from(spans)])
        })
        .collect();
    
    let log_block = Block::default()
        .borders(Borders::ALL)
        .title("Logs (Real-time)");
    let log_list = List::new(log_items)
        .block(log_block);
    f.render_widget(log_list, main_chunks[1]);
}
