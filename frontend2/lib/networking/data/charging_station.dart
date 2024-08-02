import 'package:latlong2/latlong.dart';

class ChargingStation {
  final String id;
  final double latitude;
  final double longitude;

  ChargingStation(this.id, this.latitude, this.longitude);

  LatLng getLatLng() {
    return LatLng(latitude, longitude);
  }
}
