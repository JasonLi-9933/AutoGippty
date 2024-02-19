mod ai_function;
mod apis;
mod helpers;
mod models;

use helpers::command_line::get_user_response;

fn main() {
    let user_req: String = get_user_response("how are you doing today?");
    dbg!(user_req);
}
