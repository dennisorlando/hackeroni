import "package:collection/collection.dart";
import "package:flutter/foundation.dart";
import "package:http/http.dart" as http;
import "package:http_parser/http_parser.dart";
import "package:injectable/injectable.dart";
import "package:evplanner_frontend/networking/authentication.dart";
import "package:evplanner_frontend/networking/data/authenticated_user.dart";
import "package:evplanner_frontend/networking/data/charging_station.dart";
import "package:evplanner_frontend/networking/data/image_verification.dart";
import "package:evplanner_frontend/networking/data/marker.dart";
import "package:evplanner_frontend/networking/data/marker_image.dart";
import "package:evplanner_frontend/networking/data/osm_nominatim_entry.dart";
import "package:evplanner_frontend/networking/data/outlet_type.dart";
import "package:evplanner_frontend/networking/data/pill.dart";
import "package:evplanner_frontend/networking/data/review_verdict.dart";
import "package:evplanner_frontend/networking/data/route.dart";
import "package:evplanner_frontend/networking/data/user.dart";
import "package:evplanner_frontend/networking/error.dart";
import "package:evplanner_frontend/networking/parsers.dart";
import "package:evplanner_frontend/networking/server_host_handler.dart";
import "package:evplanner_frontend/util/future.dart";
import "package:evplanner_frontend/util/nullable.dart";
import "package:evplanner_frontend/util/pair.dart";
import "package:latlong2/latlong.dart";
import "package:package_info_plus/package_info_plus.dart";
import 'package:path/path.dart' as path;

import "data/map_marker.dart";
import "data/marker_type.dart";
import "data/marker_update.dart";

@lazySingleton
class Backend {
  final http.Client _client;
  final Authentication _auth;
  final ServerHostHandler _serverHostHandler;

  Backend(this._client, this._auth, this._serverHostHandler);

  Future<dynamic> _getJson(String path, {Map<String, dynamic>? params}) {
    return _client //
        .get(
          _serverHostHandler.getUri(path, params: params),
          // still send the authentication cookie so that the backend can send specialized responses
          // when logged in
          headers: _auth.maybeCookie().map((cookie) => {"Cookie": cookie}),
        )
        .throwErrors()
        .mapParseJson();
  }

  Future<dynamic> _getJsonWithPath(String path, {Map<String, dynamic>? params}) {
    return _client //
        .get(
      Uri.parse(path),
      // still send the authentication cookie so that the backend can send specialized responses
      // when logged in
      headers: _auth.maybeCookie().map((cookie) => {"Cookie": cookie}),
    )
        .throwErrors()
        .mapParseJson();
  }

  Future<dynamic> _getJsonAuthenticated(String path, {Map<String, dynamic>? params}) async {
    final String? cookie = _auth.maybeCookie();
    if (cookie == null) {
      throw UnauthorizedException(401, "Cookie is null");
    }

    final response = await _client.get(
      _serverHostHandler.getUri(path, params: params),
      headers: {"Cookie": cookie},
    );

    if (response.statusCode == 401) {
      // the authentication token is not valid anymore, so remove it and ask the user to re-login
      _auth.removeStoredCookie();
    }
    return (await response.throwErrors()).mapParseJson();
  }

  Future<http.StreamedResponse> _postAuthenticated(String path,
      {Map<String, String>? fields, List<http.MultipartFile>? files}) async {
    final String? cookie = _auth.maybeCookie();
    if (cookie == null) {
      throw UnauthorizedException(401, "Cookie is null");
    }

    final request = http.MultipartRequest(
      "POST",
      _serverHostHandler.getUri(path),
    );
    request.headers["Cookie"] = cookie;

    if (fields != null) {
      request.fields.addAll(fields);
    }

    if (files != null) {
      request.files.addAll(files);
    }

    final response = await _client.send(request);
    if (response.statusCode == 401) {
      // the authentication token is not valid anymore, so remove it and ask the user to re-login
      //_auth.removeStoredCookie();
    }
    return await response.throwErrors();
  }

  Future<dynamic> _postJsonAuthenticated(String path,
      {Map<String, String>? fields, List<http.MultipartFile>? files}) {
    return _postAuthenticated(path, fields: fields, files: files).mapParseJson();
  }

  Future<http.Response> _post(String path,
      {Map<String, String>? fields, List<http.MultipartFile>? files}) async {

    final response = await _client.post(_serverHostHandler.getUri(path), body: fields);
    if (response.statusCode == 401) {
      // the authentication token is not valid anymore, so remove it and ask the user to re-login
      //_auth.removeStoredCookie();
    }
    return await response.throwErrors();
  }

  Future<dynamic> _postJson(String path,
      {Map<String, String>? fields, List<http.MultipartFile>? files}) {
    return _post(path, fields: fields, files: files).mapParseJson();
  }

  Future<void> deleteAccount() {
    return _postAuthenticated("/delete_account");
  }

  Future<Pill> loadRandomPill() async {
    return _getJson("/pills/random").map(pillFromJson);
  }

  Future<List<OsmNominatimEntry>> loadNominatimEntries(String phrase, [int limit = 10]){
    return _getJsonWithPath("https://nominatim.openstreetmap.org/search?q=%22$phrase%22&format=jsonv2&limit=$limit")
        .map((entries) => entries.map<OsmNominatimEntry>(nominEntryFromJson).toList());
  }

  Future<List<ChargingStation>> loadChargingStations(double latitude, double longitude) async {
    return _getJson("/get_all_stations", params: {
      "lat": latitude.toString(),
      "lon": longitude.toString(),
    }).map((stations) => stations.map<ChargingStation>(chargingStationFromJson).toList());
  }

  Future<Map<RouteAlgorithm, RouteData>> loadRoutes(LatLng source, LatLng destination,
      Duration duration, int chargeLeft, int chargeRequested, Duration maxWalkingTime,
      OutletType plugType, int maxCurrent, int batteryCapacity) async {
    return _postJson("/get_routes", fields: {
      "source_lat": source.longitude.toString(),
      "source_long": source.latitude.toString(),
      "destination_lat": destination.latitude.toString(),
      "destination_long": destination.longitude.toString(),
      "duration": duration.inSeconds.toString(),
      "charge_left": chargeLeft.toString(),
      "charge_requested": chargeRequested.toString(),
      "max_walking_time": maxWalkingTime.inSeconds.toString(),
      "capacity": batteryCapacity.toString(),
      "max_current": maxCurrent.toString(),
      if (plugType != OutletType.any) "plug": plugType.rustId,
    }).map((routes) {
      print("ROUTS $routes");
      Map<RouteAlgorithm, RouteData> routemap = {};
      if (routes["balanced"] != null) {
        routemap[RouteAlgorithm.balanced] = routeDataFromJson(routes["balanced"]);
      }
      if (routes["least_walking_time"] != null) {
        routemap[RouteAlgorithm.leastWalking] = routeDataFromJson(routes["least_walking_time"]);
      }
      if (routes["least_driving_time"] != null) {
        routemap[RouteAlgorithm.leastDriving] = routeDataFromJson(routes["least_driving_time"]);
      }
      if (routes["least_cost"] != null) {
        routemap[RouteAlgorithm.leastCost] = routeDataFromJson(routes["least_cost"]);
      }
      return routemap;
    });
  }

  Future<List<MapMarker>> loadMapMarkers(
      double latitude, double longitude, bool includeResolved) async {
    return _getJson("/map/get_near", params: {
      "y": latitude.toString(),
      "x": longitude.toString(),
      "srid": "4326", // gps
      "include_resolved": includeResolved ? "true" : "false",
    }).map((markers) => markers.map<MapMarker>(mapMarkerFromJson).toList());
  }

  Future<MarkerUpdate> addMarker(double latitude, double longitude, MarkerType markerType) {
    return _postJsonAuthenticated("/map/add", fields: {
      "y": latitude.toString(),
      "x": longitude.toString(),
      "marker_types_id": markerType.id.toString(),
    }).map(markerUpdateFromJson);
  }

  Future<void> addMarkerImage(int markerId, Uint8List image, String? mimeType) async {
    await _postAuthenticated("/map/image/add", fields: {
      "refers_to_id": markerId.toString(),
    }, files: [
      http.MultipartFile.fromBytes(
        "image",
        image,
        contentType: mimeType?.map(MediaType.parse) ?? MediaType("image", ""),
      ),
    ]);
  }

  Future<List<int>> getImagesForMarker(int markerId) {
    return _getJson("/map/image/list/$markerId").map(intListFromJson);
  }

  Future<Marker> getMarker(int markerId) {
    return (_auth.isLoggedIn()
            ? _getJsonAuthenticated("/map/$markerId")
            : _getJson("/map/$markerId"))
        .map(markerFromJson);
  }

  Future<MarkerUpdate> resolveMarker(int markerId) {
    return _postJsonAuthenticated("/map/resolve/$markerId").map(markerUpdateFromJson);
  }

  Future<AuthenticatedUser> getAuthenticatedUser() {
    return _getJsonAuthenticated("/user").map(authenticatedUserFromJson);
  }

  Future<User> getUser(int userId) {
    return _getJson("/user/$userId").map(userFromJson);
  }

  Future<void> reportAsInappropriate(int markerId) {
    return _postAuthenticated("/map/report/$markerId");
  }

  Future<void> suggestPill(String text, String source) {
    return _postAuthenticated("/pills/add", fields: {"text": text, "source": source});
  }

  Future<bool> isCompatible() async {
    final packageInfo = await PackageInfo.fromPlatform();
    return _getJson("/compatibile", params: {"version_str": packageInfo.version})
        .map((v) => v as bool);
  }

  Future<List<MarkerImage>> getToReview() {
    return _getJsonAuthenticated("/map/image/to_review")
        .map((img) => img.map<MarkerImage>(markerImageFromJson).toList());
  }

  Future<void> review(int imageId, ReviewVerdict verdict) {
    return _postAuthenticated("/map/image/review/$imageId", fields: {"verdict": verdict.verdict});
  }

  Future<VerifyTime> getNextVerifyTime() async {
    try {
      final utcDateTime = await _getJsonAuthenticated("/verify/get_next_verify_time");
      return VerifyTime.date(DateTime.parse(utcDateTime));
    } on UnauthorizedException catch (e) {
      if (e.statusCode != 403) {
        rethrow;
      }
      switch (e.response) {
        case "accepted_to_review_pending":
          return VerifyTime.notAcceptedYet(true);
        case "accepted_to_review_refused":
          return VerifyTime.notAcceptedYet(false);
        default:
          rethrow;
      }
    }
  }

  Future<void> setAcceptedToReview(bool acceptedToReview) {
    return _postAuthenticated("/verify/set_accepted_to_review",
        fields: {"accepted_to_review": acceptedToReview.toString()});
  }

  Future<List<ImageVerification>> getVerifySession() {
    return _getJsonAuthenticated("/verify/get_session").map(sessionFromJson);
  }

  // returns the awarded points iff the session has ended
  Future<double?> setVerifyVerdict(int imageId, bool verdict) {
    return _postJsonAuthenticated("/verify/set_verdict", fields: {
      "image_id": imageId.toString(),
      "verdict": verdict.toString(),
    }).map((p0) => p0 as double?);
  }

  Future<List<User>> getGlobalScoreboard() {
    return _getJson("/scoreboard/global") //
        .map((users) => users.map<User>(userFromJson).toList());
  }

  Future<Pair<String?, List<User>>> getSpecialScoreboard() {
    return _getJson("/scoreboard/special") //
        .map((special) => Pair(special["name"], special["users"].map<User>(userFromJson).toList()));
  }

  Future<List<User>> getGeographicalScoreboard(double latitude, double longitude, double radius) {
    return _getJson("/scoreboard/geographical", params: {
      "y": latitude.toString(),
      "x": longitude.toString(),
      "srid": "4326", // gps
      "radius": radius.toString(),
    }).map((users) => users.map<User>(userFromJson).toList());
  }

  Future<List<String>> getIntroImages() {
    // the server may return relative URLs, parse those correctly too
    final currentPath = _serverHostHandler.getUri("").toString();
    final context = path.Context(style: path.Style.url, current: currentPath);
    return _getJson("/resource/intro")
        .map(stringListFromJson)
        .map((links) => links.map((link) => context.absolute(link)).toList());
  }
}
