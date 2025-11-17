use chrono::Local;
use sysinfo::System;
use std::net::IpAddr;

use super::data::{RealtimeData, HardwareInfo};

pub fn format_log_message(message: &str) -> String {
    let now = Local::now();
    let timestamp = now.format("[%d/%m/%Y | %H:%M:%S]");
    format!("{} {}", timestamp, message)
}

fn shorten_cpu_name(full_name: &str) -> String {
    let name_upper = full_name.to_uppercase();
    if let Some(threadripper_pos) = name_upper.find("THREADRIPPER") {
        let after_threadripper = &full_name[threadripper_pos..];
        if let Some(space_pos) = after_threadripper.find(' ') {
            let model_part = &after_threadripper[space_pos + 1..];
            let end = model_part.find(|c: char| !c.is_alphanumeric() && c != '-' && c != ' ').unwrap_or(model_part.len());
            let model = model_part[..end.min(model_part.len())].trim();
            if !model.is_empty() {
                return format!("Threadripper {}", model);
            }
        }
    }
    if let Some(epyc_pos) = name_upper.find("EPYC") {
        let after_epyc = &full_name[epyc_pos..];
        if let Some(space_pos) = after_epyc.find(' ') {
            if let Some(num_start) = after_epyc[space_pos..].find(|c: char| c.is_ascii_digit()) {
                let num_part = &after_epyc[space_pos + num_start..];
                let end = num_part.find(|c: char| !c.is_ascii_digit() && c != '-' && c != ' ').unwrap_or(num_part.len());
                return format!("EPYC {}", num_part[..end].trim());
            }
        }
    }
    if let Some(ultra_pos) = name_upper.find("ULTRA") {
        let before_ultra = &full_name[..ultra_pos];
        if before_ultra.to_uppercase().contains("CORE") {
            let after_ultra = &full_name[ultra_pos..];
            if let Some(space_pos) = after_ultra.find(' ') {
                let model_part = &after_ultra[space_pos + 1..];
                let end = model_part.find(|c: char| !c.is_alphanumeric() && c != '-' && c != ' ' && c != '@').unwrap_or(model_part.len());
                let model = model_part[..end.min(model_part.len())].trim();
                if !model.is_empty() {
                    return format!("Ultra {}", model);
                }
            }
        }
    }
    if let Some(core_pos) = name_upper.find("CORE") {
        let after_core = &full_name[core_pos..];
        let after_core_upper = after_core.to_uppercase();
        for (i, ch) in after_core_upper.char_indices() {
            if ch == 'I' {
                if let Some(next_ch) = after_core_upper.chars().nth(i + 1) {
                    if next_ch.is_ascii_digit() {
                        let i_part = &after_core[i..];
                        if let Some(dash_pos) = i_part.find('-') {
                            let model = &i_part[..dash_pos + 1];
                            let num_part = &i_part[dash_pos + 1..];
                            let end = num_part.find(|c: char| !c.is_ascii_digit() && c != '-' && c != ' ').unwrap_or(num_part.len());
                            return format!("{}{}", model.to_uppercase(), num_part[..end].trim());
                        }
                        break;
                    }
                }
            }
        }
    }
    
    if let Some(xeon_pos) = name_upper.find("XEON") {
        let after_xeon = &full_name[xeon_pos..];
        if let Some(space_pos) = after_xeon.find(' ') {
            let model_part = &after_xeon[space_pos + 1..];
            let end = model_part.find(|c: char| !c.is_alphanumeric() && c != '-' && c != ' ' && c != '@').unwrap_or(model_part.len());
            let model = model_part[..end.min(model_part.len())].trim();
            if !model.is_empty() {
                return format!("Xeon {}", model);
            }
        }
    }
    
    if let Some(atom_pos) = name_upper.find("ATOM") {
        let after_atom = &full_name[atom_pos..];
        if let Some(space_pos) = after_atom.find(' ') {
            let model_part = &after_atom[space_pos + 1..];
            let end = model_part.find(|c: char| !c.is_alphanumeric() && c != '-' && c != ' ' && c != '@').unwrap_or(model_part.len());
            let model = model_part[..end.min(model_part.len())].trim();
            if !model.is_empty() {
                return format!("Atom {}", model);
            }
        }
    }
    
    if let Some(ryzen_pos) = name_upper.find("RYZEN") {
        let after_ryzen = &full_name[ryzen_pos..];
        if let Some(space_pos) = after_ryzen.find(' ') {
            let num_part = &after_ryzen[space_pos + 1..];
            let end = num_part.find(|c: char| !c.is_ascii_digit() && c != ' ').unwrap_or(num_part.len().min(2));
            let ryzen_num = num_part[..end.min(num_part.len())].trim();
            if !ryzen_num.is_empty() {
                return format!("AMD Ryzen {}", ryzen_num);
            }
        }
    }
    

    if let Some(pentium_pos) = name_upper.find("PENTIUM") {
        let after_pentium = &full_name[pentium_pos..];
        if let Some(space_pos) = after_pentium.find(' ') {
            let model_part = &after_pentium[space_pos + 1..];
            let end = model_part.find(|c: char| !c.is_alphanumeric() && c != '-' && c != ' ' && c != '@').unwrap_or(model_part.len());
            let model = model_part[..end.min(model_part.len())].trim();
            if !model.is_empty() {
                return format!("Pentium {}", model);
            }
        }
    }
    if let Some(celeron_pos) = name_upper.find("CELERON") {
        let after_celeron = &full_name[celeron_pos..];
        if let Some(space_pos) = after_celeron.find(' ') {
            let model_part = &after_celeron[space_pos + 1..];
            let end = model_part.find(|c: char| !c.is_alphanumeric() && c != '-' && c != ' ' && c != '@').unwrap_or(model_part.len());
            let model = model_part[..end.min(model_part.len())].trim();
            if !model.is_empty() {
                return format!("Celeron {}", model);
            }
        }
    }
    
    if let Some(athlon_pos) = name_upper.find("ATHLON") {
        let after_athlon = &full_name[athlon_pos..];
        if let Some(space_pos) = after_athlon.find(' ') {
            let model_part = &after_athlon[space_pos + 1..];
            let end = model_part.find(|c: char| !c.is_alphanumeric() && c != '-' && c != ' ' && c != '@').unwrap_or(model_part.len());
            let model = model_part[..end.min(model_part.len())].trim();
            if !model.is_empty() {
                return format!("Athlon {}", model);
            }
        }
    }
    
    let parts: Vec<&str> = full_name.split_whitespace().collect();
    for (i, part) in parts.iter().enumerate() {
        if part.contains('-') && part.chars().any(|c| c.is_ascii_digit()) {
            if i > 0 {
                return format!("{} {}", parts[i-1], part);
            } else {
                return part.to_string();
            }
        }
    }
    
    let parts: Vec<&str> = full_name.split_whitespace().take(3).collect();
    parts.join(" ")
}



pub async fn update_hardware_info(data: &RealtimeData) {
    let mut system = System::new_all();
    system.refresh_all();


    let cpu_name = if let Some(cpu) = system.cpus().first() {
        shorten_cpu_name(cpu.brand())
    } else {
        "Unknown".to_string()
    };

    let cpu_usage = if !system.cpus().is_empty() {
        let total: f32 = system.cpus().iter().map(|c| c.cpu_usage()).sum();
        total / system.cpus().len() as f32
    } else {
        0.0
    };


    let ram_total = system.total_memory();
    let ram_used = system.used_memory();

    let ping = match "8.8.8.8".parse::<IpAddr>() {
        Ok(ip) => {
            match ping_rs::send_ping(&ip, std::time::Duration::from_secs(1), &[0; 8], None) {
                Ok(result) => Some(result.rtt as u32),
                Err(_) => None,
            }
        }
        Err(_) => None,
    };
    let mut hardware = data.hardware.lock().unwrap();
    *hardware = HardwareInfo {
        cpu_name,
        cpu_usage,
        ram_used,
        ram_total,
        ping,
    };
}
