use regex::Regex;
use std::collections::HashMap;
use std::fs;

fn extract_code(s: &str) -> Option<String> {
    let re = Regex::new(r"([A-Z]{2,4})-(\d{3})").ok()?;
    if let Some(cap) = re.captures(s) {
        return Some(format!("{}-{}", &cap[1], &cap[2]));
    }
    None
}

pub fn load_map() -> HashMap<String, String> {
    let mut m = HashMap::new();
    if let Ok(content) = fs::read_to_string("/home/eynor/Documents/2_Proyect_Private/5_Note_Wrok/Traduccion.md") {
        let lines_vec: Vec<String> = content.lines().map(|x| x.to_string()).collect();
        let mut i = 0;
        while i < lines_vec.len() {
            let line = lines_vec[i].clone();
            if let Some(code) = extract_code(&line) {
                // look ahead for name
                let mut j = i + 1;
                while j < lines_vec.len() && j < i + 5 {
                    let nl = lines_vec[j].trim().to_string();
                    if nl.is_empty() { j += 1; continue; }
                    if nl.starts_with("Paralelo") || nl.starts_with("Docente") || nl.starts_with("Aula") || nl.starts_with("Ingresar") { j += 1; continue; }
                    if extract_code(&nl).is_some() { break; }
                    // likely name
                    if nl.chars().any(|c| c.is_alphabetic()) && nl.len() > 3 {
                        m.insert(code.clone(), nl.clone());
                        break;
                    }
                    j += 1;
                }
            }
            i += 1;
        }
    }
    if m.is_empty() {
        m.insert("PGE-711".to_string(), "Política y Gerencia Empresarial".to_string());
        m.insert("TQA-711".to_string(), "Testing, Quality Assurance".to_string());
        m.insert("PGM-712".to_string(), "Programación Gráfica y Multimedia II".to_string());
        m.insert("IAR-711".to_string(), "Inteligencia Artificial".to_string());
        m.insert("RED-712".to_string(), "Redes II".to_string());
        m.insert("PII-713".to_string(), "Proyecto Integrador Intermedio III".to_string());
        m.insert("PDM-711".to_string(), "Programación de Dispositivos Móviles".to_string());
        m.insert("SCO-711".to_string(), "Sistemas de Control".to_string());
    }
    m
}

pub fn translate(code: &str) -> Option<String> {
    let map = load_map();
    if let Some(v) = map.get(code) { return Some(v.clone()); }
    if let Some(c) = extract_code(code) {
        if let Some(v) = map.get(&c) { return Some(v.clone()); }
    }
    None
}
