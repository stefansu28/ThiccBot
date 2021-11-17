use crate::{error::ThiccError, ErrorMap, ThiccClient, ThiccResult};
use std::collections::HashMap;

use core::fmt;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Alias {
    pub name: String,
    pub command: String,
}

impl fmt::Display for Alias {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

pub struct AliasManager<'a> {
    client: &'a ThiccClient,
    guild_route: String,
}

impl AliasManager<'_> {
    pub async fn get(&self, search: &str) -> ThiccResult<Option<Alias>> {
        let res = self
            .client
            .get_json::<Alias>(&format!("{}/{}", self.guild_route, search))
            .await;
        ThiccClient::swallow_404(res)
    }

    pub async fn list(&self) -> ThiccResult<Vec<Alias>> {
        self.client.get_json::<Vec<Alias>>(&self.guild_route).await
    }

    pub async fn create(&self, alias: &Alias) -> ThiccResult<Alias> {
        let errors: ErrorMap = HashMap::from([(
            reqwest::StatusCode::BAD_REQUEST,
            ThiccError::ResourceAlreadyExist {
                name: alias.name.clone(),
                resource_type: "Alias".to_string(),
            },
        )]);
        let res = self.client.post_json(&self.guild_route, alias).await;
        ThiccClient::handle_status(res, errors)
    }
}

const ALIAS_ROUTE: &str = "alias/discord";

impl ThiccClient {
    pub fn alias(&self, guild_id: u64) -> AliasManager {
        let guild_route = format!("{}/{}", ALIAS_ROUTE, guild_id);
        AliasManager {
            client: &self,
            guild_route,
        }
    }
}