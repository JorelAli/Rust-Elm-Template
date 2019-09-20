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
          <style>
            {css}
          </style>
          <script>
            {elm}
          </script>
          <!--<link rel="stylesheet" href="whatever-you-want.css">
          <script src="main.js"></script>-->
        </head>
        
        <body>
          <div id="elm"></div>
          <script>
          var app = Elm.Main.init({{
            node: document.getElementById('elm')
          }});
          </script>
        </body>
        </html>
    "#,
    css = include_str!("../www/style.css"),
    elm = include_str!("../www/main.js"));

    /*let html = format!(r#"
    <html>
        <head>
        <title>My title</title>
        <link href="https://fonts.googleapis.com/css?family=PT+Sans" rel="stylesheet">
        <style>{css}</style>
        </head>
        <body>
        {body}
        <script>{js}</script>
        </body>
    </html>
    "#,
    css = include_str!("../www/style.css"),
    body = include_str!("../www/body.html"),
    js = include_str!("../www/dist.js"));*/

    //let html = include_str!("../www/index.html");

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
