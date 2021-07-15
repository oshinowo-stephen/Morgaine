#[derive(Debug, Deserialize)]
pub struct MainConfig {
	cache: CacheConfig,
	server: ServerConfig,
	database_url: String,
	cloud_store: Option<CloudStoreDetails>,
}

#[derive(Debug, Deserialize)]
pub struct CacheConfig {
	host: String,
	port: Option<String>,
	password: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
	port: isize,
	host: String,
	store: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CloudStoreDetails {
	token: String,
}