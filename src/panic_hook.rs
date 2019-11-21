use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::process::exit;

use crate::helpers::data_dir;

const PANIC_LOGFILE: &str = "panic.log";

pub fn on_panic(info: &std::panic::PanicInfo) {
    // Print panic to stderr, as usual.
    let panic_msg = format!("{:#}\n{:#?}", info, backtrace::Backtrace::new());
    eprintln!("{}", &panic_msg);

    let logfile_path = if let Some(mut path) = data_dir() {
        path.push(PANIC_LOGFILE);
        path
    } else {
        PathBuf::from(".")
    };

    // Open logfile for writing.
    let mut logfile = match OpenOptions::new()
        .create(true)
        .append(true)
        .open(&logfile_path)
    {
        Err(_) => {
            eprintln!(
                "Couldn't write error to file: {}",
                logfile_path.to_str().unwrap_or("<INVALID_LOGFILE_PATH>")
            );
            exit(1);
        }
        Ok(f) => f,
    };

    // Gather info.
    let now = chrono::Local::now();
    let date_string = now.format("%Y-%m-%d %H:%M:%S").to_string();

    let output = format!(
        "====================\n{}\n====================\n{}\n",
        date_string, panic_msg,
    );

    // Print panic info to file.
    if let Err(err) = logfile.write(output.as_bytes()) {
        eprintln!("Couldn't print panic info to file: {}", err);
        exit(1);
    }
}
