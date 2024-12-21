use web_view::{Content, WebView};

pub fn init_web_view<'a>() -> WebView<'a, ()> {
    let mut wv = web_view::builder()
        .title("My Browser")
        .content(Content::Url("https://doc.rust-lang.org/book/"))
        .size(800, 600)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .build()
        .unwrap();

    inject_javascript(
        &mut wv,
        r#"
         document.addEventListener("keydown", (event)=>{
            if(event.key === '5'){
                for (let i = 0; i < count; i++) {
                    document.dispatchEvent(new KeyboardEvent("keydown", { key: "Tab" }));
                }
            }
         })
        }"#,
    );

    wv
}

fn inject_javascript(webview: &mut WebView<()>, script: &str) {
    webview.eval(script).expect("Failed to inject JavaScript");
}
