use diesel::prelude::*;
use diesel::RunQueryDsl;
use serde::{Deserialize, Serialize};

use crate::db::PgConnection;

use super::DBError;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = super::schema::station_info)]
pub struct StationInfo {
    pub id: String,
    pub name: String,
    pub coordinate_lat: f64,
    pub coordinate_long: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = super::schema::station_plugs)]
pub struct PlugsInfo {
    pub id: String,
    pub station_id: String,
    pub name: String,
    pub max_power: Option<f64>,
    pub max_current: Option<f64>,
    pub min_current: Option<f64>,
    pub has_fixed_cable: Option<bool>,
    pub outlet_type_code: Option<String>,
}
pub fn update_stations(conn: &mut PgConnection, stations: Vec<StationInfo>) -> Result<(), DBError> {
    use crate::db::schema::station_info::dsl::*;

    diesel::insert_into(station_info)
        .values(&stations)
        .on_conflict_do_nothing()
        .execute(conn)?;

    Ok(())
}

pub fn update_station_plugs(conn: &mut PgConnection, plugs: Vec<PlugsInfo>) -> Result<(), DBError> {
    use crate::db::schema::station_plugs::dsl::*;
    for value in plugs.chunks(1000) {
        diesel::insert_into(station_plugs)
            .values(value)
            .on_conflict_do_nothing()
            .execute(conn)?;
    }
    diesel::update(station_plugs).filter(
        outlet_type_code.eq("OTHER")
        .or(outlet_type_code.eq("UNKNOWN"))
        .or(outlet_type_code.eq("700 bar small vehicles"))
        .or(outlet_type_code.eq("Schuko"))
        ).set(outlet_type_code.eq(None::<String>)).execute(conn)?;
    diesel::update(station_plugs).filter(
        outlet_type_code.eq("Type2Mennekes")
        .or(outlet_type_code.eq("Type2 - 400Vac"))
        ).set(outlet_type_code.eq("Type2_400")).execute(conn)?;
    diesel::update(station_plugs).filter(
        outlet_type_code.eq("Type2 - 230Vac")
        ).set(outlet_type_code.eq("Type2_230")).execute(conn)?;
    Ok(())
}
pub fn get_all_plugs(conn: &mut PgConnection) -> Result<Vec<PlugsInfo>, DBError> {
    use crate::db::schema::station_plugs::dsl::*;
    Ok(station_plugs.get_results(conn)?)
}
pub fn read_all_stations(conn: &mut PgConnection) -> Result<Vec<StationInfo>, DBError> {
    use crate::db::schema::station_info::dsl::*;
    Ok(station_info.get_results(conn)?)
}

pub fn get_plugs(conn: &mut PgConnection, s: &StationInfo)->Result<Vec<PlugsInfo>, DBError>{
    use crate::db::schema::station_plugs::dsl::*;
    let res = station_plugs.filter(station_id.eq(s.id.clone())).get_results(conn)?;
    Ok(res)
}