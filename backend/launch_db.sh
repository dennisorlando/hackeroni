sudo docker rm -f local_pg && sudo docker run  --name local_pg -e POSTGRES_PASSWORD=test  \
    -e POSTGRES_DB=hackeroni -e PGDATA=/var/lib/postgresql/data/pgdata \
	-v db-vol:/var/lib/postgresql/data \-p 5432:5432 -d postgres
#sudo docker rm -f local_osrm && sudo docker run -t -v "${PWD}:/data" ghcr.io/project-osrm/osrm-backend:v5.27.1 -p /opt/car.lua /data/berlin-latest.osm.pbf || echo "osrm-extract failed"