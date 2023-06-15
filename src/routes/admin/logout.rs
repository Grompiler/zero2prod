use crate::session_state::TypedSession;
use crate::utils::see_other;
use actix_web::HttpResponse;
use actix_web_flash_messages::FlashMessage;

pub async fn log_out(session: TypedSession) -> HttpResponse {
    session.log_out();
    FlashMessage::info("You have successfully logged out.").send();
    see_other("/login")
}
