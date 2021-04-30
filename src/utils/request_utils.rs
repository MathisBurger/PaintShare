use actix_web::HttpRequest;

/// This function trys to get the requested cookie
/// from the request and returns false as
/// value in the tuple, if cookie not found
pub fn get_cookie(req: HttpRequest, name: &str) -> (bool, String) {

    let cookie_header = req
        .headers()
        .get("cookie");

    match cookie_header {
        Some(cookie) => {},
        None => { return (false, "".to_string()) }
    }


    let cookie: Vec<&str> = cookie_header.unwrap()
        .to_str()
        .unwrap()
        .split("&")
        .collect();

    let auth_token: Vec<&str> = cookie
        .into_iter()
        .filter(|each| {
            let body: Vec<&str> = each.split("=").collect();

            body[0] == name
        })
        .collect();

    if &auth_token.len() == &(0 as usize)  {
        return (false, "".to_string())
    }

    let cookie_part: Vec<&str> = auth_token[0].split("=").collect();

    (true, cookie_part[1].to_owned())
}