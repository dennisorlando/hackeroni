import 'package:latlong2/latlong.dart';

enum RouteAlgorithm { lessWalking, moreCharging, lessDriving, balanced }

class RouteData {

  RouteData(this.finalCharge, this.walkingDistance, this.drivingDistance, this.walkingPath, this.drivingPath);

  int finalCharge;
  Duration walkingDistance;
  Duration drivingDistance;
  int cost = -1;

  List<LatLng> walkingPath;
  List<LatLng> drivingPath;
}
