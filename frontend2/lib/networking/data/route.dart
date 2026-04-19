import 'package:flutter/material.dart';
import 'package:latlong2/latlong.dart';
import 'package:flutter_gen/gen_l10n/app_localizations.dart';

enum RouteAlgorithm {
  balanced(Icons.balance),
  leastWalking(Icons.directions_walk),
  leastDriving(Icons.directions_car),
  leastCost(Icons.euro),
  ;

  const RouteAlgorithm(this.icon);

  final IconData icon;

  String label(AppLocalizations l10n) {
    switch (this) {
      case RouteAlgorithm.leastWalking: return l10n.leastWalking;
      case RouteAlgorithm.leastDriving: return l10n.leastDriving;
      case RouteAlgorithm.leastCost: return l10n.leastCost;
      case RouteAlgorithm.balanced: return l10n.balanced;
    }
  }
}

class RouteData {

  RouteData(this.finalCharge, this.walkingDistance, this.drivingDistance, this.cost, this.walkingPath, this.drivingPath);

  int finalCharge;
  Duration walkingDistance;
  Duration drivingDistance;
  int cost;

  List<LatLng> walkingPath;
  List<LatLng> drivingPath;
}
