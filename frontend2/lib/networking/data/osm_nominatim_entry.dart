import 'dart:ffi';

import 'package:latlong2/latlong.dart';

class OsmNominatimEntry {

  final int placeID;
  final double lat;
  final double lon;
  final String name;
  final String displayName;

  OsmNominatimEntry(this.placeID, this.lat, this.lon, this.name, this.displayName);

  LatLng toLatLng() {
    return LatLng(lat, lon);
  }
}
