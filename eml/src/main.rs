use mailparse::{parse_mail, MailHeaderMap};
use std::fs;

fn parse_eml(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // 读取EML文件内容
    let eml_content = fs::read_to_string(file_path)?;

    // 解析邮件
    let parsed = parse_mail(eml_content.as_bytes())?;

    // 打印头部信息
    println!("Subject: {:?}", parsed.headers.get_first_value("Subject"));
    println!("From: {:?}", parsed.headers.get_first_value("From"));
    println!("To: {:?}", parsed.headers.get_first_value("To"));
    println!("Date: {:?}", parsed.headers.get_first_value("Date"));

    // 打印正文
    println!("\nBody:");
    println!("{}", parsed.get_body()?);

    for (i, part) in parsed.subparts.iter().enumerate() {
        let content_disposition = part.get_content_disposition();
        if content_disposition.disposition == mailparse::DispositionType::Attachment {
            let filename = content_disposition.params.get("filename").unwrap();
            println!("\nAttachment {}: {}", i + 1, filename);
            // 这里可以保存附件内容
        }
    }

    Ok(())
}

fn parse_txt_mail(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // 读取TXT文件内容
    let content = fs::read_to_string(file_path)?;

    // 简单分割头部和正文（假设空行分隔）
    if let Some((headers, body)) = content.split_once("\n\n") {
        println!("Headers:\n{}", headers);
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
