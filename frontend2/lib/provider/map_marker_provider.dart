import 'package:collection/collection.dart';
import 'package:flutter/material.dart';
import 'package:flutter_map/flutter_map.dart';
import 'package:evplanner_frontend/networking/backend.dart';
import 'package:evplanner_frontend/networking/data/charging_station.dart';
import 'package:evplanner_frontend/networking/data/map_marker.dart';
import 'package:evplanner_frontend/networking/data/marker_type.dart';
import 'package:evplanner_frontend/page/map/marker_filters_dialog.dart';
import 'package:latlong2/latlong.dart';

class MapMarkerProvider {
  static const double markersZoomThreshold = 11.0;

  final Backend _backend;
  final Function _onStateChanged;
  final Distance _distance = const Distance();

  LatLng? _lastLoadMarkersPos;
  bool _lastLoadMarkersIncludeResolved = false;
  var _markerFilters = MarkerFilters(Set.unmodifiable(MarkerType.values), false);
  List<ChargingStation> _markers = [];

  MapMarkerProvider(this._backend, this._onStateChanged);

  void connectToMapEventStream(Stream<MapEvent> eventStream) {
    eventStream
        .where((event) =>
    event.camera.zoom >= markersZoomThreshold &&
        (_lastLoadMarkersPos == null ||
            _distance.distance(_lastLoadMarkersPos!, event.camera.center) > 5000))
        .forEach((event) => loadMarkers(event.camera.center));
  }

  void loadMarkers(final LatLng latLng) async {
    _lastLoadMarkersPos = latLng;
    _backend
        .loadChargingStations(latLng.latitude, latLng.longitude)
        .then((value) {
      if (latLng == _lastLoadMarkersPos) {
        debugPrint("Loaded markers at $latLng");
        _markers = value;
        _onStateChanged();
      } else {
        debugPrint("Ignoring outdated loaded markers at $latLng");
      }
    });
    // ignore errors when loading map markers (TODO maybe show a button to view errors somewhere?)
  }

  ChargingStation? getClosestMarker(final LatLng latLng) {
    return minBy(getVisibleMarkers(), (ChargingStation marker) => _distance(latLng, marker.getLatLng()));
  }

  Iterable<ChargingStation> getVisibleMarkers() {
    return _markers;
  }

  void addOrReplace(final MapMarker marker) {
    // _markers.removeWhere((element) => element.id == marker.id);
    // if (marker.resolutionDate == null || _lastLoadMarkersIncludeResolved) {
    //   // only add it back if it is not resolved or if the user wants to see resolved markers
    //   _markers.add(marker);
    // }
  }

  void openMarkerFiltersDialog(BuildContext context, LatLng currentMapCenter) {
    showDialog(
      context: context,
      builder: (ctx) => MarkerFiltersDialog(_markerFilters),
    ).then((newFilters) {
      if (newFilters is MarkerFilters) {
        _markerFilters = newFilters;
        _onStateChanged();
        if (newFilters.includeResolved && !_lastLoadMarkersIncludeResolved) {
          loadMarkers(currentMapCenter);
        }
      }
    });
  }
}
