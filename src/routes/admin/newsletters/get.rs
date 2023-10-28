use actix_web::http::header::ContentType;
use actix_web::HttpResponse;
use actix_web_flash_messages::IncomingFlashMessages;
use std::fmt::Write;

pub async fn newsletters_form(
    flash_messages: IncomingFlashMessages,
) -> Result<HttpResponse, actix_web::Error> {
    let mut error_html = String::new();
    for m in flash_messages.iter() {
        writeln!(error_html, "<p><i>{}</i></p>", m.content()).unwrap();
    }

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"<!DOCTYPE html>
            <html lang="en">
                <head>
                    <meta http-equiv="content-type" content="text/html; charset=utf-8">
                    <title>Send a newsletter issue</title>
                </head>
                <body>
                    {error_html}
                    <form action="/admin/newsletters" method="post">
                        <label>Title
                            <input
                                type="text"
                                placeholder="Newsletter title"
                                name="title"
                            >
                        </label>
                        <br/>
                        <label>Text Content
                            <input
                                type="text"
                                placeholder="Text content"
                                name="text_content"
                            >
                        </label>
                        <br/>
                        <label>Html Content
                            <input
                                type="text"
                                placeholder="Html content"
                                name="html_content"
                            >
                        </label>
                        <br/>
                        <button type="submit">Send</button>
                    </form>
                </body>
            </html>"#
        )))
}
