FROM osrm/osrm-backend AS Extractor
WORKDIR /data
RUN echo "deb http://archive.debian.org/debian stretch main contrib non-free" > /etc/apt/sources.list
RUN apt-get -y update && apt-get -y install curl
#ADD . /data

RUN curl -O https://download.geofabrik.de/europe/italy-latest.osm.pbf 
RUN osrm-extract -p /opt/car.lua /data/italy-latest.osm.pbf
RUN osrm-partition /data/italy-latest.osrm
RUN osrm-customize /data/italy-latest.osrm
