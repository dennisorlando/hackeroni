
use log::error;
use serde::{de::{DeserializeOwned, Error}, Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;
use geo::{point, prelude::*};

use crate::osrm::StationInfo;
pub struct ODHBuilder{
    url: String,

}
impl Default for ODHBuilder{
    fn default() -> Self {
        Self { url: "https://mobility.api.opendatahub.com".to_string()}
    }
}

#[derive(Error, Debug)]
pub enum ODHError {
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Build failed. Field {0} not provided")]
    Build(&'static str ),
    #[error("Error while parsing json: {0}")]
    JsonError(#[from] serde_json::Error)
}

impl From<ODHError> for actix_web::Error {
    fn from(value: ODHError) -> Self {
        error!("{value}");
        actix_web::error::ErrorInternalServerError("Internal server error")
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Coordinate{pub x: f64, pub y: f64, srid: u32}
#[derive(Deserialize, Serialize, Debug)]
pub struct EChargingStation{
    sactive: bool,
    savailable: bool,
    pub scode: String,
    pub scoordinate: Coordinate,
    smetadata: Value,
    pub sname: String,
    sorigin: String,
    stype: String,
}
/*
{
      "pactive": false,
      "pavailable": true,
      "pcode": "00001",
      "pcoordinate": {
        "x": 13.595222,
        "y": 47.948,
        "srid": 4326
      },
      "pmetadata": {"city": "--", "state": "ACTIVE", "address": "Hauptstraße", "capacity": 2, "provider": "AP_GEN"},
      "pname": "Chargingstation 00001_1",
      "porigin": "ALPERIA",
      "ptype": "EChargingStation",
      "sactive": false,
      "savailable": true,
      "scode": "00001-0",
      "scoordinate": {
        "x": 13.595222,
        "y": 47.948,
        "srid": 4326
      },
      "smetadata": {"outlets": [{"id": "0", "maxPower": 22, "maxCurrent": 31, "minCurrent": 0, "hasFixedCable": false, "outletTypeCode": "Type2Mennekes"}]},
      "sname": "Chargingstation 00001_1-1",
      "sorigin": "ALPERIA",
      "stype": "EChargingPlug"
    },
 */
#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct Outlets{
    id: String,
    maxPower: i32,
    maxCurrent: i32,
    minCurrent: i32,
    hasFixedCable: bool,
    outletTypeCode: String,
}
#[derive(Deserialize, Debug)]
struct PlugMetadata{
    outlets: Vec<Outlets>
}

#[derive(Deserialize, Debug)]
pub struct EChargingPlug{
    pactive: bool,
    pavailable: bool,
    pcode: String,
    pcoordinate: Coordinate,
    pmetadata: Value,
    pname: String,
    porigin: String,
    ptype: String,
    sactive: bool,
    savailable: bool,
    scode: String,
    smetadata: PlugMetadata

}



pub trait Station{
    fn get_uri()->&'static str;
}
impl Station for EChargingStation{
    fn get_uri()->&'static str {
        "EChargingStation"
    }
}

impl Station for EChargingPlug{
    fn get_uri()->&'static str {
        "EChargingPlug"
    }
}

impl ODHBuilder{
    pub async fn run<T: Station + DeserializeOwned>(self)->Result<Vec<T>, ODHError> {
        let url = self.url + "/v2/flat/" + T::get_uri();
        let content = reqwest::get(url)
        .await?
        .text()
        .await?;
        let x: Value = serde_json::from_str(&content)?;
        let t = x.get("data").ok_or(serde_json::Error::missing_field("can not find data field"))?;
        let t: Vec<T> = serde_json::from_value(t.clone())?;

        Ok(t)
        
    }
}

fn distance_in_meters(p1: (f64, f64), p2: (f64, f64))->f64{
    let p1 = point!(x: p1.0, y: p1.1);
    let p2 = point!(x: p2.0, y: p2.1);

    p1.haversine_distance(&p2)
}

pub async fn get_near_stations(p: (f64, f64), dist: f64)->Result<Vec<StationInfo>, ODHError>{
    let result: Vec<EChargingStation> = ODHBuilder{
        ..Default::default()
    }.run().await?;
    let res = result.into_iter().filter_map(|x|{
        if distance_in_meters((x.scoordinate.x, x.scoordinate.y), p)<dist{
            Some(StationInfo{
                coordinate: (x.scoordinate.x, x.scoordinate.y),
                id: x.scode,
                name: x.sname,
            })
        }else{
            None
        }
    }).collect();
    Ok(res)

}
#[tokio::test]
async fn test_request(){
    let result: Vec<EChargingPlug> = ODHBuilder{
        ..Default::default()
    }.run().await.unwrap();
    //println!("{:?}", result);
    
    println!("{:?}", result[0]);

        
}
#[tokio::test]
async fn test_station(){
    let result: Vec<EChargingStation> = ODHBuilder{
        ..Default::default()
    }.run().await.unwrap();
    //println!("{:?}", result);
    
    println!("{:?}", result[0]);

        
}