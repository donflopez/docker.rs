use api::api_utils::get_formatted_api_request;
use api::DockerApiClient;
use utils;

pub trait Version: DockerApiClient {
    /// Get version info for Docker
    /// Returns a JSON serialized string containing this information
    ///
    /// # Example
    ///
    /// ```rust
    /// extern crate docker_rs;
    ///
    /// use docker_rs::api::containers::Containers;
    /// use docker_rs::api::version::Version;
    /// use docker_rs::client::DockerClient;
    ///
    /// let client = match DockerClient::new("unix:///var/run/docker.sock") {
    ///     Ok(a) => a,
    ///     Err(err) => {
    ///         println!("{}", err);
    ///         std::process::exit(1);
    ///     }
    /// };
    ///
    /// match client.get_version_info() {
    ///     Ok(info) => println!("Version Info : {}", info),
    ///     Err(err) => println!("An error occured : {}", err),
    /// }
    /// ```
    fn get_version_info(&self) -> Result<String, String> {
        let api_endpoint = "/info";
        let method = "GET";

        let req = match get_formatted_api_request(api_endpoint, method, "") {
            Some(req) => req,
            None => return Err("Error while preparing request".to_string()),
        };

        let resp = match self.request(&req) {
            Some(resp) => match utils::parse_http_response_body(resp) {
                Some(body) => body,
                None => return Err("Response body was not valid".to_string()),
            },
            None => return Err("Got no response from docker host.".to_string()),
        };

        Ok(resp)
    }
}
