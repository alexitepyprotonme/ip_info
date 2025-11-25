use serde::{Deserialize, Serialize};
use std::fs;

// Define the structures for the response data
#[derive(Debug, Deserialize, Serialize)]
struct BGPViewInfo {
    status: String,
    status_message: String,
    data: IPData,
    #[serde(rename = "@meta")]
    meta: Meta,
}

#[derive(Debug, Deserialize, Serialize)]
struct IPData {
    ip: String,
    ptr_record: String,
    prefixes: Vec<Prefix>,
    rir_allocation: RIRAllocation,
    iana_assignment: IANAAssignment,
    maxmind: MaxMind,
}

#[derive(Debug, Deserialize, Serialize)]
struct Prefix {
    prefix: String,
    ip: String,
    cidr: u8,
    asn: Option<ASN>,
    rir_name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct ASN {
    asn: u32,
    name: String,
    description: String,
    country_code: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct RIRAllocation {
    rir_name: String,
    country_code: Option<String>,
    ip: String,
    cidr: u8,
    prefix: String,
    date_allocated: String,
    allocation_status: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct IANAAssignment {
    assignment_status: String,
    description: String,
    whois_server: String,
    // Add other fields from IANA if needed
}

#[derive(Debug, Deserialize, Serialize)]
struct MaxMind {
    country_code: Option<String>,
    city: Option<String>,
    // Add other fields from MaxMind if needed
}

#[derive(Debug, Deserialize, Serialize)]
struct Meta {
    time_zone: String,
    api_version: u8,
    execution_time: String,
}

// Entry point
fn main() {
    if let Ok(ip_addresses) = read_ip_addresses_from_file("ip_addresses.txt") {
        for ip in ip_addresses {
            match get_ip_info(&ip) {
                Ok(info) => {
                    println!("Information for {}: {:?}", ip, info);
                }
                Err(err) => {
                    eprintln!("Error getting information for {}: {}", ip, err);
                }
            }
        }
    } else {
        eprintln!("Error reading IP addresses from file.");
    }
}

fn read_ip_addresses_from_file(filename: &str) -> Result<Vec<String>, std::io::Error> {
    let contents = fs::read_to_string(filename)?;
    let ips: Vec<String> = contents.lines().map(String::from).collect();
    Ok(ips)
}

// Function to fetch IP information from BGPView API
fn get_ip_info(ip_address: &str) -> Result<BGPViewInfo, Box<dyn std::error::Error>> {
    let api_url = format!("https://api.bgpview.io/ip/{}/", ip_address);

    let client = reqwest::blocking::Client::new();
    let response = client.get(&api_url).send()?;

    if response.status().is_success() {
        let body = response.text()?;
        let info: BGPViewInfo = serde_json::from_str(&body)?;
        Ok(info)
    } else {
        Err("Request failed".into())
    }
}
