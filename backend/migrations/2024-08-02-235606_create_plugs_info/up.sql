-- Your SQL goes here


CREATE TABLE "station_plugs"(
	"id" VARCHAR NOT NULL PRIMARY KEY,
	"station_id" VARCHAR NOT NULL,
	"name" VARCHAR NOT NULL,
	"max_power" FLOAT,
	"max_current" FLOAT ,
	"min_current" FLOAT,
	"has_fixed_cable" BOOL,
	"outlet_type_code" VARCHAR 
);

