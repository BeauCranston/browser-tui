use std::io;

use crossterm::{
    event::{self, Event, KeyCode},
    terminal,
};
use web_view::{Content, WebView};
use wv_window::init_web_view;
mod term_ui;
mod wv_window;
fn main() -> io::Result<()> {
    // term_ui::init()

    // terminal::enable_raw_mode()?;

    // let web_view = init_web_view();
    //
    // web_view.run().unwrap();

    // Start capturing key presses
    // loop {
    //     if event::poll(std::time::Duration::from_millis(100))? {
    //         if let Event::Key(key_event) = event::read()? {
    //             match key_event.code {
    //                 KeyCode::Char('5') => {
    //                     // Send command to simulate 5 Tab presses
    //                     web_view
    //                         .eval("simulateTabPresses(5);")
    //                         .expect("Failed to execute JavaScript");
    //                 }
    //                 KeyCode::Esc => break, // Exit on ESC key
    //                 _ => {}
    //             }
    //         }
    //     }
    //
    //     // Update the web-view
    //     web_view.step();
    // }
    // terminal::disable_raw_mode()?;
    let mut webview = web_view::builder()
        .title("Web View Example")
        .content(Content::Url("https://doc.rust-lang.org/book/")) // Load the real URL
        .size(800, 600)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .build()
        .unwrap();

    // Inject JavaScript to add a keydown event listener
    webview
        .eval(
            r#"
        document.addEventListener('keydown', (event) => {
            console.log(`Key pressed: ${event.key}`);
            if(event.key === '5'){
                document.querySelectorAll()[3].focus() 
            }
        });
    "#,
        )
        .unwrap();

    // Run the webview
    webview.run().unwrap();
    Ok(())
}
