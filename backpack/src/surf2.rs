#[derive(Default)]
pub struct Req {
    headers: std::collections::HashMap<String, String>,
}

pub struct Resp {
    body: String,
    headers: std::collections::HashMap<String, String>,
    status: u16,
}

pub trait Request {
    fn header(&mut self, key: &str, value: &str);
    fn get(&self, url: &str) -> Result<Resp, Box<dyn std::error::Error>>;
}

pub trait Response {
    fn body(&self) -> String;
    fn headers(&self) -> std::collections::HashMap<String, String>;
    fn status(&self) -> u16;
}

impl Request for Req {
    fn header(&mut self, key: &str, value: &str) {
        self.headers.insert(key.to_string(), value.to_string());
    }

    fn get(&self, url: &str) -> Result<Resp, Box<dyn std::error::Error>> {
        /*
        let mut res = ureq::get(url);

        // Set request headers
        for (key, value) in self.headers.iter() {
            res.header(key.as_str(), value.as_str());
        }

        // Make HTTP call
        let mut body = res.call()?;
        */
        let mut res = ureq::get(url).call()?;

        // Extact response headers
        let mut headers = std::collections::HashMap::<String, String>::new();
        for (key, value) in res.headers().iter() {
            headers.insert(key.to_string().clone(), String::from(value.to_str().unwrap()));
        }

        // Create response object
        let resp = Resp {
            body: res.body_mut().read_to_string()?,
            headers:  headers,
            status: 0,
        };

        Ok(resp)
    }
}

impl Response for Resp {
    fn body(&self) -> String {
        self.body.clone()
    }

    fn headers(&self) -> std::collections::HashMap<String, String> {
        self.headers.clone()
    }

    fn status(&self) -> u16 {
        self.status.clone()
    }
}

pub fn get(url: &str) -> Result<Resp, Box<dyn std::error::Error>> {
    let req = Req { headers: Default::default() };
    req.get(url)
}
