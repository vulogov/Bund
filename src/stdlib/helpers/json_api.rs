extern crate log;
use serde::{Serialize, Deserialize};
use curl::easy::{Easy2, Handler, List, WriteError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}


#[time_graph::instrument]
pub fn json_api_post(full_url: String, headers: List, payload: serde_json::Value) -> Option<serde_json::Value>  {
    struct Collector(Vec<u8>);

    impl Handler for Collector {
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
            self.0.extend_from_slice(data);
            Ok(data.len())
        }
    }

    let mut easy = Easy2::new(Collector(Vec::new()));
    let _ = easy.useragent("BUND");

    easy.post(true).unwrap();
    easy.url(&full_url).unwrap();
    easy.http_headers(headers).unwrap();
    easy.post_fields_copy(format!("{}", &payload).as_bytes()).unwrap();
    match easy.perform() {
        Ok(_) => {},
        Err(err) => {
            log::error!("JSON_API_POST perform returns: {}", err);
            return None;
        },
    }


    let contents = easy.get_ref();
    let raw_data = String::from_utf8_lossy(&contents.0).to_string();

    let json_data: serde_json::Value = match serde_json::from_str(&raw_data) {
        Ok(json_data) => json_data,
        Err(err) => {
            log::error!("JSON_API_POST can not produce JSON: {}", err);
            return None
        },
    };

    Some(json_data)
}
