import 'dart:math';

import 'package:evplanner_frontend/networking/data/route.dart';
import 'package:evplanner_frontend/page/complete_task/complete_task_page.dart';
import 'package:evplanner_frontend/page/map/animated_message_box.dart';
import 'package:evplanner_frontend/page/map/fast_markers_layer.dart';
import 'package:evplanner_frontend/page/map/map_controls_widget.dart';
import 'package:evplanner_frontend/page/map/route_bottom_sheet.dart';
import 'package:evplanner_frontend/page/map/settings_controls_widget.dart';
import 'package:evplanner_frontend/page/route_parameters/route_parameters_page.dart';
import 'package:evplanner_frontend/pref/preferences_keys.dart';
import 'package:evplanner_frontend/provider/location_provider.dart';
import 'package:evplanner_frontend/provider/map_marker_provider.dart';
import 'package:evplanner_frontend/util/preferences.dart';
import 'package:flutter/material.dart';
import 'package:flutter_map/flutter_map.dart';
import 'package:flutter_svg/svg.dart';
import 'package:get_it_mixin/get_it_mixin.dart';
import 'package:latlong2/latlong.dart';
import 'package:shared_preferences/shared_preferences.dart';

import '../../networking/backend.dart';

class MapPage extends StatefulWidget with GetItStatefulWidgetMixin {
  MapPage({super.key});

  @override
  State<MapPage> createState() => _MapPageState();
}

const LatLng defaultInitialCoordinates = LatLng(46.47855, 11.33203);
const double defaultInitialZoom = 16.0;

class _MapPageState extends State<MapPage> with GetItStateMixin<MapPage>, WidgetsBindingObserver,
    SingleTickerProviderStateMixin<MapPage> {
  late final SharedPreferences prefs;
  late final MapMarkerProvider mapMarkerProvider;
  final MapController mapController = MapController();

  late LatLng initialCoordinates;
  late double initialZoom;
  Map<RouteAlgorithm, RouteData>? routeData;
  RouteAlgorithm selectedRouteAlgorithm = RouteAlgorithm.balanced;
  final Distance _distance = const Distance();
  late final AnimationController pillAnim;

  List<LatLng> targets = const [
    LatLng(46.67724, 11.18660),
    LatLng(46.67724, 11.18560),
    LatLng(46.67724, 11.18460),
    LatLng(46.67724, 11.18360),
    LatLng(46.67724, 11.18260),
  ];
  int i = 0;

  @override
  void initState() {
    super.initState();
    WidgetsBinding.instance.addObserver(this); // needed to keep track of app lifecycle

    mapMarkerProvider = MapMarkerProvider(get<Backend>(), () => setState(() {}));
    //mapMarkerProvider.connectToMapEventStream(mapController.mapEventStream);

    prefs = get<SharedPreferences>();
    initialCoordinates = LatLng(
      prefs.getDouble(lastMapLatitude) ?? defaultInitialCoordinates.latitude,
      prefs.getDouble(lastMapLongitude) ?? defaultInitialCoordinates.longitude,
    );
    initialZoom = prefs.getDouble(lastMapZoom) ?? defaultInitialZoom;
    i = prefs.tryGetInt("target") ?? 0;
    if (i >= targets.length - 1) {
      i = 0;
    }

    pillAnim = AnimationController(vsync: this, duration: const Duration(milliseconds: 500));
    pillAnim.forward();
    //mapMarkerProvider.loadMarkers(initialCoordinates);
  }

  @override
  Future<void> dispose() async {
    super.dispose();
    WidgetsBinding.instance.removeObserver(this);
  }

  @override
  void didChangeAppLifecycleState(AppLifecycleState state) async {
    if (state == AppLifecycleState.paused ||
        state == AppLifecycleState.inactive ||
        state == AppLifecycleState.detached) {
      await saveMapPositionToPreferences();
    }
  }

  @override
  Future<bool> didPopRoute() async {
    await saveMapPositionToPreferences();
    return super.didPopRoute();
  }

  Future<void> saveMapPositionToPreferences() async {
    await Future.wait([
      prefs.setDouble(lastMapLatitude, mapController.camera.center.latitude),
      prefs.setDouble(lastMapLongitude, mapController.camera.center.longitude),
      prefs.setDouble(lastMapZoom, mapController.camera.zoom),
    ]);
  }

  @override
  Widget build(BuildContext context) {
    final position = watchStream((LocationProvider location) => location.getLocationStream(),
            get<LocationProvider>().lastLocationInfo())
        .data;

    // Uncomment to test the rendering performance with lots of markers
    /*markers = <MapMarker>[];
    for (int i = 0; i < 100; ++i) {
      for (int j = 0; j < 100; ++j) {
        markers.add(MapMarker(
          0,
          45.7555 + .0009 * i,
          11.0033 + .0009 * j,
          MarkerType.values[(i+j) % MarkerType.values.length],
          DateTime(2023),
          DateTime(2023),
          0,
          0,
        ));
      }
    }*/

    print("distance: ${_distance(position?.toLatLng() ?? initialCoordinates, targets[i])}");
    final theme = Theme.of(context);
    return Scaffold(
      body: Stack(
        children: [
          FlutterMap(
            mapController: mapController,
            options: MapOptions(
                interactionOptions: const InteractionOptions(
                  flags: (InteractiveFlag.all | InteractiveFlag.doubleTapDragZoom) &
                      ~InteractiveFlag.rotate,
                ),
                initialCenter: initialCoordinates,
                initialZoom: initialZoom,
                // OSM supports at most the zoom value 19
                maxZoom: 18.45,
                onTap: (tapPosition, tapLatLng) {
                  final data = routeData;
                  if (data != null) {
                    RouteAlgorithm? closestAlgorithm;
                    double closestDistance = double.infinity;
                    double cameraZoom = pow(2.0, mapController.camera.zoom) / 50.0;
                    for (final e in data.entries) {
                      for (final point in e.value.walkingPath + e.value.drivingPath) {
                        final distance = _distance(point, tapLatLng);
                        if (distance < cameraZoom && distance < closestDistance) {
                          closestDistance = distance;
                          closestAlgorithm = e.key;
                        }
                      }
                    }

                    if (closestAlgorithm != null) {
                      setState(() {
                        selectedRouteAlgorithm = closestAlgorithm!;
                      });
                      return;
                    }
                  }

                  final minMarker = mapMarkerProvider.getClosestMarker(tapLatLng);
                  if (minMarker == null) {
                    return;
                  }

                  final markerScale = markerScaleFromMapZoom(mapController.camera.zoom);
                  final screenPoint =
                      mapController.camera.latLngToScreenPoint(minMarker.getLatLng());
                  final dx = (tapPosition.global.dx - screenPoint.x).abs();
                  final dy = (tapPosition.global.dy - screenPoint.y).abs();
                  if (max(dx, dy) < markerScale * 0.7) {
                    // TODO open charging station
                    //openMarkerPage(minMarker);
                  }
                },
                onLongPress: (tapPosition, tapLatLng) {
                  openRouteParametersPage(tapLatLng);
                }),
            children: [
              TileLayer(
                urlTemplate: "https://tile.openstreetmap.org/{z}/{x}/{y}.png",
              ),
              MarkerLayer(
                markers: [position?.toLatLng()]
                        .whereType<LatLng>()
                        .map<Marker>(
                          (pos) => Marker(
                            rotate: true,
                            point: pos,
                            child: SvgPicture.asset("assets/icons/current_location.svg"),
                          ),
                        )
                        .toList() +
                    <Marker>[
                      Marker(
                        rotate: true,
                        point: targets[i],
                        child: const Icon(Icons.place_outlined, color: Colors.white, size: 40),
                      ),
                      Marker(
                        rotate: true,
                        point: targets[i],
                        child: const Icon(Icons.place, color: Colors.red, size: 40),
                      ),
                    ],
              ),
              if (i < targets.length - 1 &&
                  position != null) // && _distance(position.toLatLng()!, targetPos) < 50)
                Align(
                  alignment: Alignment.bottomRight,
                  child: Padding(
                    padding: const EdgeInsets.all(16),
                    child: FloatingActionButton(
                      onPressed: () {
                        openCompleteTaskPage();
                      },
                      child: const Icon(Icons.photo_camera),
                    ),
                  ),
                ),
              if (i >= targets.length - 1)
                Align(
                  alignment: Alignment.bottomCenter,
                  child: Padding(
                      padding: const EdgeInsets.all(16),
                      child: AnimatedMessageBox(
                        animation: pillAnim,
                        message: 'Hai finito il gioco!',
                        containerColor: theme.colorScheme.secondaryContainer,
                        onContainerColor: theme.colorScheme.onSecondaryContainer,
                      )),
                ),
              const Align(
                alignment: Alignment.bottomLeft,
                child: Text(
                  " © OpenStreetMap contributors",
                  style: TextStyle(
                      color: Color.fromARGB(255, 127, 127, 127)), // theme-independent grey
                ),
              ),
              Align(
                alignment: Alignment.topRight,
                child: MapControlsWidget(mapController),
              ),
              Align(
                alignment: Alignment.topLeft,
                child: SettingsControlsWidget(() => mapMarkerProvider.openMarkerFiltersDialog(
                    context, mapController.camera.center)),
              ),
            ],
          ),
          Align(
            alignment: Alignment.bottomCenter,
            child: RouteBottomSheet(routeData, () => setState(() => routeData = null),
                selectedRouteAlgorithm, (value) => setState(() => selectedRouteAlgorithm = value)),
          ),
        ],
      ),
    );
  }

  void openRouteParametersPage(LatLng destination, [String? destinationName]) {
    showModalBottomSheet(
        context: context,
        builder: (BuildContext context) {
          return RouteParametersPage(RouteParametersPageArgs(destination, destinationName));
        }).then((value) {
      print("YEEEE $value");
      if (value is Map<RouteAlgorithm, RouteData>) {
        setState(() {
          routeData = value;
        });
      }
    });
  }

  void openCompleteTaskPage() {
    Navigator.pushNamed(
      context,
      CompleteTaskPage.routeName,
      arguments: CompleteTaskPageArgs(targets[i]),
    ).then((event) {
      setState(() {
        print("Event $event");
        if (event is bool && event == true) {
          i += 1;
          prefs.setInt("target", i);
        }
      });
    });
  }

  void gotoNextTarget() {}
}
