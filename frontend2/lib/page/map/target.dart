import 'package:latlong2/latlong.dart';

class Target {
  final double lat;
  final double long;
  final String task;
  int seed = 0;

  Target(this.lat, this.long, this.task);

  LatLng toLatLng() {
    return LatLng(lat, long);
  }

  @override
  String toString() {
    return task;
  }
}
