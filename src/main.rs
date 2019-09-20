extern crate web_view;

use web_view::*;

fn main() {
    let size = (700, 400);
    let resizable = true;
    let debug = false;
    let titlebar_transparent = true;
    let frontend_cb = |_webview: &mut _, _arg: &_, _userdata: &mut _| {};
    let userdata = ();

    let html = format!(r#"
        <!DOCTYPE HTML>
        <html>
        <head>
          <meta charset="UTF-8">
          <title>Main</title>

          <!-- Styles -->
          <link rel="stylesheet" href="https://stackpath.bootstrapcdn.com/bootstrap/4.3.1/css/bootstrap.min.css" integrity="sha384-ggOyR0iXCbMQv3Xipma34MD+dH/1fQ784/j6cY/iJTQUOhcWr7x9JvoRxT2MZw1T" crossorigin="anonymous">

          <style>{css}</style>

          <!-- Elm -->
          <script>{elm}</script>
        </head>
        
        <body>
          <div id="elm"></div>
          <script> var app = Elm.Main.init({{ node: document.getElementById('elm') }}); </script>
        </body>
        </html>
    "#,
    css = include_str!("../www/style.css"),
    elm = include_str!("../www/main.js"));

    run(
        "",
        Content::Html(html),
        Some(size),
        resizable,
        debug,
        titlebar_transparent,
        move |mut webview| {
            webview.set_background_color(0.11, 0.12, 0.13, 1.0);
        },
        frontend_cb,
        userdata
    );
}
