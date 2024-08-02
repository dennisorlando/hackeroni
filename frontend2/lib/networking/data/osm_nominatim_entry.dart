import 'dart:ffi';

class OsmNominatimEntry {

  final int placeID;
  final double lat;
  final double lon;
  final String name;
  final String displayName;

  OsmNominatimEntry(this.placeID, this.lat, this.lon, this.name, this.displayName);
}
