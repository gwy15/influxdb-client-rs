use std::{error::Error, string::ParseError};
use reqwest::{Client as HttpClient, Method, Url};

use crate::models::InfluxError;

use super::models::{Point};

pub struct Client {
    host: Url,
    token: String,
    client: HttpClient,
    bucket: Option<String>,
    org: Option<String>,
    org_id: Option<String>,
}

impl Client {

    pub fn new<T>(host: &str, token: T) -> Client
    where
        T: Into<String>,
    {
        let host_url = reqwest::Url::parse(host).unwrap();

        Client {
            host: host_url,
            token: token.into(),
            client: HttpClient::default(),
            bucket: None,
            org: None,
            org_id: None,
        }
    }

    pub fn with_bucket<T: Into<String>>(mut self, bucket: T) -> Self {
        self.bucket = Some(bucket.into());
        self
    }

    pub fn with_org<T: Into<String>>(mut self, org: T) -> Self {
        self.org = Some(org.into());
        self
    }

    pub fn with_org_id<T: Into<String>>(mut self, org_id: T) -> Self {
        self.org_id = Some(org_id.into());
        self
    }

    pub async fn insert_points<T: Iterator<Item = Point>>(self, points: T) -> Result<(), InfluxError> {

        let body = points.map(|p| format!("{}\n", p.serialize()))
            .collect::<Vec<String>>()
            .join("\n");
        
        let result = self.new_request(Method::POST, "/api/v2/write")
            .body(body)
            .send()
            .await
            .unwrap()
            .error_for_status();

        

        println!("Sending request!");
        // let text = result.text().await.unwrap();
        // let r = result.error_for_status();

        if let Err(err) = result {
            let status = err.status().unwrap().as_u16();
            println!("Status: {:?}", status);
            Err(Client::status_to_influxerror(status, Box::new(err)))
        } else {
            Ok(())
        }
    
    }

    fn new_request(self, method: Method, path: &str) -> reqwest::RequestBuilder {
        // Build query params
        let mut query_params = Vec::<(&str, String)>::new();
        if let Some(bucket) = self.bucket {
            query_params.push(("bucket", bucket));
        }
        
        if let Some(org) = self.org {
            query_params.push(("org", org));
        }
        else if let Some(org_id) = self.org_id {
            query_params.push(("orgID", org_id));
        }

        // Build default request
        let mut url = self.host.clone();
        url.set_path(path);
        
        self.client.request(method, url)
            .header("Content-Type", "text/plain")
            .header("Authorization", format!("{} {}", "Token", self.token))
            .query(&query_params)
    }

    fn status_to_influxerror(status: u16, err: Box<dyn Error>) -> InfluxError {
        match status {
            400 => InfluxError::InvalidSyntax(err.to_string()),
            401 => InfluxError::InvalidCredentials(err.to_string()),
            403 => InfluxError::Forbidden(err.to_string()),
            _ => InfluxError::Unknown(err.to_string()),   
        }
    }
}

