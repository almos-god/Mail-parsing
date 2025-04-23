use mailparse::parse_mail; // 仅用于解码MIME内容
use std::fs;
use std::collections::HashMap;

// 自定义邮件头解析器（替代mailparse的头部解析）
fn parse_headers(raw: &str) -> HashMap<String, String> {
    let mut headers = HashMap::new();
    let mut current_key = String::new();
    let mut current_value = String::new();

    for line in raw.lines() {
        if line.is_empty() {
            break; // 头部结束
        }

        if line.starts_with(' ') || line.starts_with('\t') {
            // 多行头部的延续行
            current_value.push_str(line.trim_start());
        } else if let Some((key, value)) = line.split_once(':') {
            // 保存上一个头
            if !current_key.is_empty() {
                headers.insert(current_key.trim().to_string(), current_value.trim().to_string());
            }
            // 开始新头
            current_key = key.to_string();
            current_value = value.to_string();
        }
    }

    // 添加最后一个头
    if !current_key.is_empty() {
        headers.insert(current_key.trim().to_string(), current_value.trim().to_string());
    }

    headers
}

// 使用标准库解析邮件结构
fn parse_eml_structure(raw: &[u8]) -> (HashMap<String, String>, Vec<u8>) {
    let content = String::from_utf8_lossy(raw);
    if let Some(header_end) = content.find("\r\n\r\n") {
        let headers = parse_headers(&content[..header_end]);
        let body = raw[header_end + 4..].to_vec(); // +4 跳过\r\n\r\n
        (headers, body)
    } else {
        (HashMap::new(), raw.to_vec())
    }
}

fn parse_eml(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let raw = fs::read(file_path)?;

    // 使用标准库解析基本结构
    let (headers, body) = parse_eml_structure(&raw);
    println!("Subject: {:?}", headers.get("Subject"));
    println!("From: {:?}", headers.get("From"));
    println!("To: {:?}", headers.get("To"));
    println!("Date: {:?}", headers.get("Date"));

    // 仅用mailparse解码MIME内容
    let parsed = parse_mail(&raw)?;
    println!("\nBody:");
    println!("{}", parsed.get_body()?);

    // 附件处理仍用mailparse（因为MIME解析复杂）
    for (i, part) in parsed.subparts.iter().enumerate() {
        if let Some(filename) = part.get_content_disposition().params.get("filename") {
            println!("\nAttachment {}: {}", i + 1, filename);
        }
    }

    Ok(())
}

fn parse_txt_mail(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)?;

    // 纯标准库实现文本邮件解析
    if let Some((headers, body)) = content.split_once("\n\n") {
        let parsed_headers = parse_headers(headers);
        println!("Headers:");
        for (key, value) in &parsed_headers {
            println!("{}: {}", key, value);
        }
        println!("\nBody:\n{}", body);
    } else {
        println!("Content:\n{}", content);
    }

    Ok(())
}

fn main() {
    let eml_file = "邮件原始文件解析.eml";
    let txt_file = "eml.txt";

    println!("Parsing EML file:");
    if let Err(e) = parse_eml(eml_file) {
        eprintln!("Error parsing EML: {}", e);
    }

    println!("\nParsing TXT file:");
    if let Err(e) = parse_txt_mail(txt_file) {
        eprintln!("Error parsing TXT: {}", e);
    }
}