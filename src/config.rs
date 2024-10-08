#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub client_origin: String,
    pub domain: String,

    pub mongodb_url: String,
    pub mongodb_test_url: String,
    pub mongo_initdb_db: String,
    pub mongo_test_db: String,

    pub access_token_private_key: String,
    pub access_token_public_key: String,
    pub access_token_expires_in: String,
    pub access_token_max_age: i64,

    pub refresh_token_private_key: String,
    pub refresh_token_public_key: String,
    pub refresh_token_expires_in: String,
    pub refresh_token_max_age: i64,

    pub google_oauth_client_id: String,
    pub google_oauth_client_secret: String,
    pub google_oauth_redirect_url: String,
}

impl Config {
    pub fn init() -> Config {
        let database_url = get_env_var("DATABASE_URL");
        let client_origin = get_env_var("CLIENT_ORIGIN");
        let domain = get_env_var("DOMAIN");

        let mongodb_url = get_env_var("MONGODB_URL");
        let mongodb_test_url = get_env_var("MONGODB_TEST_URL");
        let mongo_test_db = get_env_var("MONGO_TEST_DATABASE");
        let mongo_initdb_db = get_env_var("MONGO_INITDB_DATABASE");

        let access_token_private_key = get_env_var("ACCESS_TOKEN_PRIVATE_KEY");
        let access_token_public_key = get_env_var("ACCESS_TOKEN_PUBLIC_KEY");
        let access_token_expires_in = get_env_var("ACCESS_TOKEN_EXPIRED_IN");
        let access_token_max_age = get_env_var("ACCESS_TOKEN_MAXAGE");

        let refresh_token_private_key = get_env_var("REFRESH_TOKEN_PRIVATE_KEY");
        let refresh_token_public_key = get_env_var("REFRESH_TOKEN_PUBLIC_KEY");
        let refresh_token_expires_in = get_env_var("REFRESH_TOKEN_EXPIRED_IN");
        let refresh_token_max_age = get_env_var("REFRESH_TOKEN_MAXAGE");

        let google_oauth_client_id =
            std::env::var("GOOGLE_OAUTH_CLIENT_ID").expect("GOOGLE_OAUTH_CLIENT_ID must be set");
        let google_oauth_client_secret = std::env::var("GOOGLE_OAUTH_CLIENT_SECRET")
            .expect("GOOGLE_OAUTH_CLIENT_SECRET must be set");
        let google_oauth_redirect_url = std::env::var("GOOGLE_OAUTH_REDIRECT_URL")
            .expect("GOOGLE_OAUTH_REDIRECT_URL must be set");

        Config {
            database_url,
            client_origin,
            domain,
            mongodb_url,
            mongodb_test_url,
            mongo_initdb_db,
            mongo_test_db,
            access_token_private_key,
            access_token_public_key,
            refresh_token_private_key,
            refresh_token_public_key,
            access_token_expires_in,
            refresh_token_expires_in,
            access_token_max_age: access_token_max_age.parse::<i64>().unwrap(),
            refresh_token_max_age: refresh_token_max_age.parse::<i64>().unwrap(),
            google_oauth_client_id,
            google_oauth_client_secret,
            google_oauth_redirect_url,
        }
    }
}

fn get_env_var(var_name: &str) -> String {
    std::env::var(var_name).unwrap_or_else(|_| panic!("{} must be set", var_name))
}
