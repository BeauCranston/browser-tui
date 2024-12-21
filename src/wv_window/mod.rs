use web_view::*;

pub fn init_web_view() {
    web_view::builder()
        .title("My Browser")
        .content(Content::Url("https://doc.rust-lang.org/book/"))
        .size(800, 600)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .run()
        .unwrap();
}
