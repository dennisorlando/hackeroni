// @generated automatically by Diesel CLI.

diesel::table! {
    station_info (id) {
        id -> Varchar,
        name -> Varchar,
        coordinate_lat -> Float8,
        coordinate_long -> Float8,
    }
}

diesel::table! {
    station_plugs (id) {
        id -> Varchar,
        station_id -> Varchar,
        name -> Varchar,
        max_power -> Nullable<Float8>,
        max_current -> Nullable<Float8>,
        min_current -> Nullable<Float8>,
        has_fixed_cable -> Nullable<Bool>,
        outlet_type_code -> Nullable<Varchar>,
    }
}

diesel::table! {
    users (id) {
        id -> Int8,
        name -> Varchar,
        password_hash -> Varchar,
        is_admin -> Bool,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    station_info,
    station_plugs,
    users,
);
