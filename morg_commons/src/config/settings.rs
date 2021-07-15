#[derive(Debug, Deserialize)]
pub struct MainConfig {
	pub cache: CacheConfig,
	pub server: ServerConfig,
	pub database_url: String,
	pub cloud_store: Option<CloudStoreDetails>,
}

#[derive(Debug, Deserialize)]
pub struct CacheConfig {
	pub host: String,
	pub port: Option<String>,
	pub password: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
	pub port: isize,
	pub host: String,
	pub store: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CloudStoreDetails {
	pub token: String,
}