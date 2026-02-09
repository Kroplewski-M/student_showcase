#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_max_age_mins: i64,
    pub port: u16,
    pub post_mark_config: PostMarkConfig,
    pub auth_cookie_name: String,
}

#[derive(Clone)]
pub struct PostMarkConfig {
    pub mail_from_email: String,
    pub server_token: String,
}

impl Config {
    pub fn init() -> Config {
        let database_url =
            std::env::var("DATABASE_URL").expect("DATABASE_URL IS NOT SET IN THE ENV");
        let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET IS NOT SET IN THE ENV");
        let jwt_max_age_mins =
            std::env::var("JWT_MAX_AGE_MINS").expect("JWT_MAX_AGE_MINS IS NOT SET IN THE ENV");
        let mail_from_email = std::env::var("POSTMARK_FROM_EMAIL")
            .expect("POSTMARK_FROM_EMAIL IS NOT SET IN THE ENV");
        let mail_server_token = std::env::var("POSTMARK_SERVER_TOKEN")
            .expect("POSTMARK_SERVER_TOKEN IS NOT SET IN THE ENV");

        let auth_cookie_name =
            std::env::var("COOKIE_NAME").expect("COOKIE_NAME IS NOT SET IN THE ENV");
        Config {
            database_url,
            jwt_secret,
            jwt_max_age_mins: jwt_max_age_mins
                .parse::<i64>()
                .expect("JWT_MAX_AGE_MINS IS NOT IN THE CORRECT FORMAT"),
            port: 8080,
            post_mark_config: PostMarkConfig {
                mail_from_email,
                server_token: mail_server_token,
            },
            auth_cookie_name,
        }
    }
}
