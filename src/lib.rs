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

static DISCORD_APP_CLIENT_ID: &str = "1062171154976100392";
static DISCORD_PUBLIC_KEY: &str =
    "10792ca4f4fe1fc1ce3848aae7e7e90236ccd829417eb35cda0a24bb3ab1999f";
static REDIRECT_URL: &str = "http://127.0.0.1:9000/";
// static REDIRECT_URL: &str = "https://code.flows.network/lambda/qspd8Z8TpU";
static SCOPES: &str = "applications.commands";

fn get_access(code: &str) -> Option<String> {
    let uri = Uri::try_from("https://discord.com/api/oauth2/token").unwrap();
    let params = serde_json::json!({
        "client_id": DISCORD_APP_CLIENT_ID,
        "client_secret": DISCORD_PUBLIC_KEY,
        "code": code,
    });
    let params = serde_json::to_string(&params).unwrap();

    let mut writer = Vec::new();
    if let Ok(res) = Request::new(&uri)
        .method(Method::POST)
        .header("content-type", "application/x-www-form-urlencoded")
        .header("accept", "application/x-www-form-urlencoded")
        .header("content-length", &params.as_bytes().len())
        .body(&params.as_bytes())
        .send(&mut writer)
    {
        if res.status_code().is_success() {
            if let Ok(res) = serde_json::from_slice::<Value>(&writer) {
                if let Some(at) = res["access_token"].as_str() {
                    return Some(at.to_string());
                }
            }
        }
    }

    None
}

fn get_user(token: &str) -> Option<Value> {
    let uri = Uri::try_from("https://api.github.com/user").unwrap();

    let mut writer = Vec::new();
    if let Ok(res) = Request::new(&uri)
        .method(Method::GET)
        .header("user-agent", "Flows.network function")
        .header("authorization", &format!("Bearer {}", token))
        .header("accept", "application/vnd.github+json")
        .send(&mut writer)
    {
        if res.status_code().is_success() {
            if let Ok(res) = serde_json::from_slice::<Value>(&writer) {
                return Some(res);
            }
        }
    }

    None
}
