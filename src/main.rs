fn main() {
    let query = r#"{
        "Properties": {
            "AuthMethod": "RPS",
            "SiteName": "user.auth.xboxlive.com",
            "RpsTicket": "d=<access token>"
        },
        "RelyingParty": "http://auth.xboxlive.com",
        "TokenType": "JWT"
    }"#;


    let resp = ureq::post("https://user.auth.xboxlive.com/user/authenticate")
        .send_string(query);

    println!("ureq:");
    if let Ok(response) = resp {
        println!("{:?}", response);
    } else if let Err(e) = resp {
        println!("{}", e);
    }

    let resp = isahc::post("https://user.auth.xboxlive.com/user/authenticate", query).unwrap();
    
    println!("isahc:");
    println!("{:?}", resp);
}
