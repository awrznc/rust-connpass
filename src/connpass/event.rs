#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub url: String,
    pub query: String,
    pub head: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub status: u16,
    pub body: Option<Body>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Body {
    pub results_returned: u32,
    pub results_available: u32,
    pub results_start: u32,
    pub events: Vec<Event>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    pub event_id: u32,
    pub title: String,
    pub catch: String,
    pub description: String,
    pub event_url: String,
    pub hash_tag: String,
    pub started_at: String,
    pub ended_at: String,
    pub limit: Option<u32>,
    pub event_type: String,
    pub series: Option<Series>,
    pub address: Option<String>,
    pub place: Option<String>,
    pub lat: Option<String>,
    pub lon: Option<String>,
    pub owner_id: u32,
    pub owner_nickname: String,
    pub owner_display_name: String,
    pub accepted: u32,
    pub waiting: u32,
    pub updated_at: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Series {
    pub id: u32,
    pub title: String,
    pub url: String
}

use std::error::Error;

#[cfg(test)]
use std::fs::File;
#[cfg(test)]
use std::io::BufReader;
#[cfg(test)]
use std::path::Path;


#[cfg(test)]
fn read_file<P: AsRef<Path>>(path: P) -> Result<Response, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let status: u16 = 200;
    let body: Body = serde_json::from_reader(reader)?;

    let response: Response = Response {
        status: status,
        body: Some(body),
    };
    Ok(response)
}


#[cfg(not(test))]
fn get_response(uri: &str, head: &str) -> Result<Response, Box<dyn Error>> {
    let url = reqwest::Url::parse(uri).unwrap();

    let client = reqwest::blocking::Client::builder()
        .user_agent(head)
        .build()?;

    let response_object = client.get(url)
        .send()
        .expect("http get error");

    let status: u16 = response_object.status().as_u16();
    let text: String = response_object.text().expect("get response text error");
    let body: Body = serde_json::from_str( &text ).expect("json parse error");

    let response: Response = Response {
        status: status,
        body: Some(body),
    };
    Ok(response)
}

/// Initialize event instance.
pub fn new() -> Request {
    Request::new()
}


impl Request {
    fn new() -> Request {
        Request {
            url: "https://connpass.com/api/v1/event/".to_string(),
            query: "".to_string(),
            head: "rust-connpass".to_string()
        }
    }

    pub fn get(&self) -> Result<Response, Box<dyn Error>> {
        #[cfg(test)]
        let response = read_file("./mock/200.json")?;

        #[cfg(not(test))]
        let response = get_response( &format!("{}?{}", &self.url, &self.query), &self.head )?;
    
        Ok(response)
    }

    pub fn query(self, set: &mut [(&str, &str)]) -> Request {

        let mut string = set.iter().fold(String::new(), |acc, &(l, r)| {
            acc + l + "=" + r + "&"
        });
        string.pop();
        Request {
            query: string,
            .. self
        }
    }
}
