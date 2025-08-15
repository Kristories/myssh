use cursive::align::HAlign;
use cursive::traits::*;
use cursive::views::{Dialog, SelectView, TextView};
use cursive::Cursive;
use std::fs;
use std::path::Path;

fn main() {
    let home = match std::env::var("HOME") {
        Ok(h) => h,
        Err(_) => {
            let mut siv = cursive::default();
            siv.add_layer(
                Dialog::text("HOME environment variable is not set").button("Quit", |s| s.quit()),
            );
            siv.run();
            return;
        }
    };

    let path = Path::new(&home).join(".ssh");

    let arr = match get_ssh_files(&path) {
        Ok(a) => a,
        Err(_) => {
            let mut siv = cursive::default();
            siv.add_layer(
                Dialog::text(format!("{} does not exist", path.display()))
                    .button("Quit", |s| s.quit()),
            );
            siv.run();
            return;
        }
    };

    let mut select = SelectView::new().h_align(HAlign::Center).autojump();

    select.add_all_str(arr);
    select.set_on_submit(show_next_window);

    let mut siv = cursive::default();

    siv.add_layer(Dialog::around(select.scrollable().fixed_size((40, 10))).title("My Keys"));
    siv.run();
}

fn get_ssh_files(dir: &Path) -> std::io::Result<Vec<String>> {
    let mut arr = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let path_str = path.display().to_string();

        if path.is_file() && !path_str.contains("known_hosts") {
            arr.push(path_str);
        }
    }
    Ok(arr)
}

fn show_next_window(siv: &mut Cursive, path: &str) {
    siv.pop_layer();

    let contents = fs::read_to_string(path).expect("Should have been able to read the file");
    let text = contents;

    siv.add_layer(Dialog::around(TextView::new(text)).button("Quit", |s| s.quit()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn missing_directory_returns_error() {
        let tmp = tempdir().unwrap();
        let missing = tmp.path().join("missing");
        let result = get_ssh_files(&missing);
        assert!(result.is_err());
    }
}
