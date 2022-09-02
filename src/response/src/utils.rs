pub fn parse_file_mime_type(file: &str) -> String {
    let file_name = file.to_string();
    let ext_name = file_name.split('.').last().unwrap_or("txt");

    match ext_name {
        "html" => "text/html".to_string(),
        "css" => "text/css".to_string(),
        "js" => "text/javascript".to_string(),
        "map" => "application/json".to_string(),
        "jpg" | "jpeg" => "image/jpeg".to_string(),
        "png" => "image/png".to_string(),
        "ico" => "image/x-icon".to_string(),
        "svg" => "image/svg+xml".to_string(),
        _ => "text/plain".to_string(),
    }
}
