use crate::ModSearch;
use crate::BASEURL;

impl ModSearch {
    // TODO: the rest of the search functions

    /// (a helper function to make the code more readable, do not use directly)
    fn _inner_request(request: &str, query: &str, api_key: &str) -> Result<String, crate::Error> {
        let body = ureq
            ::get(format!("{BASEURL}?key={api_key}&request={request}&query={query}").as_str())
            .timeout(std::time::Duration::from_secs(60))
            .call();

        match body {
            Ok(body) => Ok(body.into_string().unwrap_or_default()),
            Err(e) => Err(crate::Error::APIRequestError(Box::new(e))),
        }
    }
}
