use chrono::Local;
use std::io::Write;

pub fn make_root_path(target: &str) -> String {
    let mut str = target.to_string();

    if target.starts_with('/') && target.ends_with('/') {
        return str;
    } else if target.starts_with('/') {
        str.push('/');
    } else if target.ends_with('/') {
        str.insert(0, '/');
    } else {
        str.insert(0, '/');
        str.push('/');
    };

    str
}

pub fn init_logger() {
    env_logger::Builder::from_default_env()
        .format_timestamp_secs()
        .format(|f, record| {
            writeln!(
                f,
                "[{}]{:>5} > {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .init();
}
