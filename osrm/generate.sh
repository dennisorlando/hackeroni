sudo docker run -t -v "${PWD}:/data" osrm/osrm-backend osrm-extract -p /opt/car.lua /data/italy-latest.osm.pbf
sudo docker run -t -v "${PWD}:/data" osrm/osrm-backend osrm-partition /data/italy-latest.osrm
sudo docker run -t -v "${PWD}:/data" osrm/osrm-backend osrm-customize /data/italy-latest.osrm
sudo docker run -t -i -p 5000:5000 -v "${PWD}:/data" osrm/osrm-backend osrm-routed --algorithm mld /data/italy-latest.osrm