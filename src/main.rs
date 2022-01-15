use fastly::http::StatusCode;
use fastly::{mime, Error, Request, Response};

#[fastly::main]
fn main(mut req: Request) -> Result<Response, Error> {
    // Pattern match on the path...
    match req.get_path() {
        "/echo" => {
            let mut body = Vec::<u8>::default();
            if let Some(Ok(v)) = req.take_body().read_chunks(65535).next() {
                body = v;
            }
            Ok(Response::from_status(StatusCode::OK).with_body_bytes(&body))
        }

        "/loop.html" => Ok(Response::from_status(StatusCode::OK)
            .with_body_octet_stream(include_bytes!("../static/loop.html"))
            .with_content_type(mime::TEXT_HTML_UTF_8)),

        _ => Ok(Response::from_status(StatusCode::NOT_FOUND).with_body_text_plain("")),
    }
}
