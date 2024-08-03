import 'package:flutter/material.dart';
import 'package:latlong2/latlong.dart';
import 'package:flutter_gen/gen_l10n/app_localizations.dart';

enum RouteAlgorithm {
  balanced(Icons.balance),
  lessWalking(Icons.directions_walk),
  moreCharging(Icons.power),
  lessDriving(Icons.directions_car),
  ;

  const RouteAlgorithm(this.icon);

  final IconData icon;

  String label(AppLocalizations l10n) {
    switch (this) {
      case RouteAlgorithm.lessWalking: return l10n.lessWalking;
      case RouteAlgorithm.moreCharging: return l10n.moreCharging;
      case RouteAlgorithm.lessDriving: return l10n.lessDriving;
      case RouteAlgorithm.balanced: return l10n.balanced;
    }
  }
}

class RouteData {

  RouteData(this.finalCharge, this.walkingDistance, this.drivingDistance, this.walkingPath, this.drivingPath);

  int finalCharge;
  Duration walkingDistance;
  Duration drivingDistance;
  int cost = -1;

  List<LatLng> walkingPath;
  List<LatLng> drivingPath;
}
