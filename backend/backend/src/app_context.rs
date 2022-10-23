use crate::{Clients, config_from_cmd, Repositories};
use crate::config::config::Config;

#[derive(Clone)]
pub struct AppContext {
    pub clients: Clients,
    pub repositories: Repositories,
    pub config: Config
}

impl AppContext {
    pub fn new(clients: Clients,
               repositories: Repositories,
               config: Config) -> Self {
        AppContext {
            clients,
            repositories,
            config
        }
    }
}

pub fn app_context() -> AppContext {
    let config = config_from_cmd();

    let clients = Clients::new(&config);

    let repositories = Repositories::new(&config.database_config);

    AppContext::new(clients,
                    repositories,
                    config)
}
