import 'package:geolocator/geolocator.dart';
import 'package:latlong2/latlong.dart';

extension PositionExtension on Position {
  LatLng toLatLng() {
    return LatLng(latitude, longitude);
  }
}

String _doubleToString(double d) {
  final split = d.toString().split(".").toList();
  if (split.length == 2 && split[1].length > 5) {
    return "${split[0]}.${split[1].substring(0, 5)}";
  } else {
    return d.toString();
  }
}

extension LatLngExtension on LatLng {
  String toNiceString() {
    return "${_doubleToString(latitude)}, ${_doubleToString(longitude)}";
  }
}
