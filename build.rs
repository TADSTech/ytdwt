fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        let mut res = winres::WindowsResource::new();
        res.set_icon("app_icon.ico"); // We will need to create this
        res.set("ProductName", "ytdwt");
        res.set("FileDescription", "YouTube Downloader with Tools");
        res.set("LegalCopyright", "Copyright (c) 2026 Michael Tunwashe");
        res.compile().unwrap();
    }
}
