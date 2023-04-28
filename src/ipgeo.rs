//! This module parses the IP Geo Location response from ipgeolocation.io 
//! See https://ipgeolocation.io/documentation/ip-geolocation-api.html 
//!


use serde::{Serialize, Deserialize};
use hyperactive::{client::get, err::GenericError};
use reqwest;

pub struct IpGeoClient {
    api_key: String 
}


impl IpGeoClient {

    // Create a new client by providing the api key 
    pub fn new(api_key: &str) -> Self {
        let api_key = api_key.to_string();
        IpGeoClient{api_key}
    }

    fn geoloc_url(&self, ip_address: &str) -> String {
        let url = format!("https://api.ipgeolocation.io/ipgeo?apiKey={}&ip={}", &self.api_key, &ip_address);
        url 
    }


    pub async fn geoloc_ip(&self, ip_address: &str) -> Result<IpGeoLoc, GenericError> {
        let url = self.geoloc_url(ip_address);
        let resp: IpGeoLoc = reqwest::get(&url)
            .await?
            .json::<IpGeoLoc>()
            .await?;
        Ok(resp)
    }



}


#[derive(Serialize, Deserialize, Debug)]
pub struct Currency {
    pub code: String,  // i.e. USD",
    pub name: String,  // i.e. US Dollar",
    pub symbol: String,  // i.e. $"
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TimeZone {
    pub name: String,  // i.e. America/Los_Angeles",
    pub offset: i8,
    pub current_time: String,  // i.e. 2020-12-17 07:49:45.872-0800",
    pub current_time_unix: f64,     // i.e. 1608220185.872,
    pub is_dst: bool,     // i.e. false,
    pub dst_savings: u8,    // i.e. 1
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IpGeoLoc {
    pub ip: String,  // i.e. 8.8.8.8",
    pub hostname: Option<String>,  // i.e. dns.google",
    pub continent_code: String,  // i.e. NA",
    pub continent_name: String,  // i.e. North America",
    pub country_code2: String,  // i.e. US",
    pub country_code3: String,  // i.e. USA",
    pub country_name: String,  // i.e. United States",
    pub country_capital: String,  // i.e. Washington, D.C.",
    pub state_prov: String,  // i.e. California",
    pub district: String,  // i.e. Santa Clara",
    pub city: String,  // i.e. Mountain View",
    pub zipcode: String,  // i.e. 94043-1351",
    pub latitude: String,  // i.e. 37.42240",
    pub longitude: String,  // i.e. -122.08421",
    pub is_eu: bool,
    pub calling_code: String,  // i.e. +1",
    pub country_tld: String,  // i.e. .us",
    pub languages: String,  // i.e. en-US,es-US,haw,fr",
    pub country_flag: String,  // i.e. https://ipgeolocation.io/static/flags/us_64.png",
    pub geoname_id: String,  // i.e. 6301403",
    pub isp: String,  // i.e. Google LLC",
    pub connection_type: String,  // i.e. ",
    pub organization: String,  // i.e. Google LLC",
    pub asn: Option<String>,  // i.e. AS15169",
    pub currency: Currency,
    pub time_zone: TimeZone,
}