use airtable_flows::create_record;
use http_req::{
    request::{Method, Request},
    uri::Uri,
};
use lambda_flows::{request_received, send_response};
use serde_json::Value;
use slack_flows::send_message_to_channel;

#[no_mangle]
pub fn run() {
    request_received(|qry, _body| {
        if let Some(code) = qry.get("code") {
            send_message_to_channel("ik8", "general", code.to_string());

            // if let Some(code) = code.as_str() {
            //     let at = get_access_token(
            //         [
            //             ("grant_type", "authorization_code"),
            //             ("code", &code),
            //             ("redirect_uri", REDIRECT_URL.as_str()),
            //         ]
            //         .into_iter(),
            //     )
            //     .await
            //     .map_err(|e| (StatusCode::UNAUTHORIZED, e))?;

            //         if let Some(user) =     get_authed_user(&at.access_token)
            //         .await
            //         .map(|user| {
            //             let location = format!(
            //                 "{}/api/connected?authorId={}&authorName={}&authorState={}&refreshState={}",
            //                 HAIKU_API_PREFIX.as_str(),
            //                 user.id,
            //                 user.username,
            //                 encrypt(&at.access_token),
            //                 encrypt(&at.refresh_token)
            //             );

            //             (StatusCode::FOUND, [("Location", location)])
            //         })
            //         .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e)) {
            //             let record = serde_json::json!({
            //                 "Login": user["login"],
            //                 "Name": user["name"],
            //                 "Email": user["email"],
            //                 "Location": user["location"],
            //                 "Created At": user["created_at"]
            //             });
            //             create_record("jaykchen", "appButiJsqQBEjzVV", "ghgh", record.clone());
            //             send_message_to_channel("ik8", "general", user["blog"].to_string());
            //         }
            // }
        }

        send_response(
            200,
            vec![(String::from("content-type"), String::from("text/html"))],
            "ok".as_bytes().to_vec(),
        );
    });
}
