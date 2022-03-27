use anyhow::Context;
use bytes::Bytes;
use error::{ClientErrors, ThiccError};
use log::trace;
use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION},
    Client, IntoUrl, RequestBuilder, Url,
};
use serde::{de::DeserializeOwned, Serialize};

pub mod alias;
pub mod error;
pub mod guilds;
pub mod key_words;
pub mod last_fm;
pub mod quotes;

pub type ThiccResult<T> = std::result::Result<T, ClientErrors>;

/// Wrapper around [`Client`] to support a `base_url`
#[derive(Debug, Clone)]
pub struct ThiccClient {
    client: Client,
    base_url: Url,
}

impl ThiccClient {
    /// Makes a new [`ThiccClient`] with the provided `base_url` and `api_key`
    pub fn new<U: IntoUrl>(base_url: U, api_key: &str) -> ThiccClient {
        let base_url: Url =
            base_url.into_url().expect("Error parsing base_url");

        let mut headers = HeaderMap::new();
        let mut auth_value =
            HeaderValue::from_str(api_key).expect("Error parsing api_key");
        auth_value.set_sensitive(true);
        headers.insert(AUTHORIZATION, auth_value);

        let client = Client::builder()
            .default_headers(headers)
            .build()
            .expect("Error building client");

        ThiccClient { client, base_url }
    }

    fn join_with_base<U: IntoUrl>(&self, url: U) -> ThiccResult<Url> {
        let url = url.as_str();
        if url.starts_with('/') {
            return Err(ClientErrors::InvalidRelativeUrl(url.to_string()));
        }

        let url = self
            .base_url
            .join(url)
            .with_context(|| format!("Failed parsing relative url {}", url))?;
        Ok(url)
    }

    pub fn post<U: IntoUrl>(&self, url: U) -> ThiccResult<RequestBuilder> {
        let url = self.join_with_base(url)?;
        Ok(self.client.post(url))
    }

    pub fn get<U: IntoUrl>(&self, url: U) -> ThiccResult<RequestBuilder> {
        trace!("ass fart");
        let url = self.join_with_base(url)?;
        trace!("dddd: {url}");
        Ok(self.client.get(url))
    }

    pub fn delete<U: IntoUrl>(&self, url: U) -> ThiccResult<RequestBuilder> {
        let url = self.join_with_base(url)?;
        Ok(self.client.delete(url))
    }

    pub fn put<U: IntoUrl>(&self, url: U) -> ThiccResult<RequestBuilder> {
        let url = self.join_with_base(url)?;
        Ok(self.client.put(url))
    }

    pub async fn delete_helper<U: IntoUrl>(&self, url: U) -> ThiccResult<()> {
        let _ = self.delete(url)?.send().await?.error_for_status();
        Ok(())
    }

    pub async fn get_json<T: DeserializeOwned, U: IntoUrl>(
        &self,
        url: U,
    ) -> ThiccResult<T> {
        let res = self
            .get(url)?
            .send()
            .await?
            .error_for_status()?
            .json::<T>()
            .await?;
        Ok(res)
    }

    pub async fn get_bytes<U: IntoUrl>(&self, url: U) -> ThiccResult<Bytes> {
        let res = self
            .get(url)?
            .send()
            .await?
            .error_for_status()?
            .bytes()
            .await?;
        Ok(res)
    }

    pub async fn post_json<
        Payload: Serialize + ?Sized,
        Res: DeserializeOwned,
        U: IntoUrl,
    >(
        &self,
        url: U,
        payload: &Payload,
    ) -> ThiccResult<Res> {
        let res = self
            .post(url)?
            .json(payload)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        Ok(res)
    }

    pub async fn put_json<
        Payload: Serialize + ?Sized,
        Res: DeserializeOwned,
        U: IntoUrl,
    >(
        &self,
        url: U,
        payload: &Payload,
    ) -> ThiccResult<Res> {
        let res = self
            .put(url)?
            .json(payload)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        Ok(res)
    }

    /// given an [`ThiccResult`], if its a 404 error from [`reqwest::Error`]
    /// return [`None`], otherwise return the passed result
    pub fn swallow_404<T>(result: ThiccResult<T>) -> ThiccResult<Option<T>> {
        match result {
            Ok(value) => Ok(Some(value)),
            Err(ClientErrors::Reqwest(req_error))
                if req_error.status()
                    == Some(reqwest::StatusCode::NOT_FOUND) =>
            {
                Ok(None)
            }
            Err(e) => Err(e),
        }
    }

    pub fn handle_status<
        T,
        F: FnOnce(reqwest::StatusCode) -> Option<ThiccError>,
    >(
        result: ThiccResult<T>,
        err_map: F,
    ) -> ThiccResult<T> {
        match result {
            Ok(value) => Ok(value),
            Err(ClientErrors::Reqwest(req_error)) => match req_error.status() {
                Some(status) => match err_map(status) {
                    Some(err) => Err(err.into()),
                    None => Err(req_error.into()),
                },
                _ => Err(req_error.into()),
            },
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use httptest::{matchers::*, responders::*, Expectation, ServerPool};
    static SERVER_POOL: ServerPool = ServerPool::new(10);

    #[tokio::test]
    async fn test_get_add_base() -> ThiccResult<()> {
        let _ = pretty_env_logger::try_init();
        let server = SERVER_POOL.get_server();
        let url = server.url("").to_string();

        server.expect(
            Expectation::matching(request::method_path("GET", "/fart"))
                .respond_with(status_code(200)),
        );

        let client = ThiccClient::new(url, "poop");

        let resp = client.get("fart")?.send().await?;

        assert!(resp.status().is_success());

        Ok(())
    }
}
