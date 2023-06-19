use actix_web::http::header::ContentType;
use actix_web::HttpResponse;
use actix_web_flash_messages::IncomingFlashMessages;
use std::fmt::Write;

pub async fn submit_newsletter_form(
    flash_message: IncomingFlashMessages,
) -> Result<HttpResponse, actix_web::Error> {
    let mut msg_html = String::new();
    for m in flash_message.iter() {
        writeln!(msg_html, "<p><i>{}</i></p>", m.content()).unwrap();
    }
    let idempotency_key = uuid::Uuid::new_v4().to_string();
    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"
            <!DOCTYPE html>
            <html lang="en">
                <head>
                    <meta http-equiv="content-type" content="text/html; charset=utf-8">
                    <title>Newsletters</title>
                </head>
                <body>
                    {msg_html}
                    <form action="/admin/newsletters" method="post">
                        <label>Newsletter title
                            <input 
                                type="text" 
                                placeholder="title" 
                                name="title"
                            >
                        </label>

                        <label>Newsletter text content
                            <input 
                                type="text" 
                                placeholder="text content" 
                                name="text_content"
                            >
                        </label>

                        <label>Newsletter html content
                            <input 
                                type="text" 
                                placeholder="html content" 
                                name="html_content"
                            >
                        </label>

                        </br>
                        <input hidden type="text" name="idempotency_key" value="{idempotency_key}"
                        <button type="submit">Submit newsletter</button>
                    </form>
                    <p><a href="/admin/dashboard">&lt; - Back</a></p>
                </body>
            </html>
        "#,
        )))
}
