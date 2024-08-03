import 'package:flutter/material.dart';
import 'package:flutter_gen/gen_l10n/app_localizations.dart';
import 'package:get_it_mixin/get_it_mixin.dart';
import 'package:evplanner_frontend/networking/backend.dart';
import 'package:evplanner_frontend/networking/data/outlet_type.dart';
import 'package:evplanner_frontend/pref/preferences_keys.dart';
import 'package:evplanner_frontend/provider/location_provider.dart';
import 'package:evplanner_frontend/util/error_text.dart';
import 'package:evplanner_frontend/util/nullable.dart';
import 'package:evplanner_frontend/util/position.dart';
import 'package:evplanner_frontend/util/preferences.dart';
import 'package:evplanner_frontend/util/time.dart';
import 'package:latlong2/latlong.dart';
import 'package:shared_preferences/shared_preferences.dart';

import '../../util/pair.dart';

class RouteParametersPage extends StatefulWidget with GetItStatefulWidgetMixin {
  static const routeName = '/routeParametersPage';

  final RouteParametersPageArgs args;

  RouteParametersPage(this.args, {super.key});

  @override
  State<RouteParametersPage> createState() => _RouteParametersPageState();
}

class RouteParametersPageArgs {
  final LatLng destination;
  final String? destinationName;

  RouteParametersPageArgs(this.destination, this.destinationName);
}

class _RouteParametersPageState extends State<RouteParametersPage>
    with GetItStateMixin<RouteParametersPage> {
  final LatLng? source = null; // null means "current position"
  final formKey = GlobalKey<FormState>();
  late final SharedPreferences prefs;

  String lastError = "";
  String? sourceString;
  String? destinationString;
  double currentBatteryCharge = 50;
  double wantedBatteryCharge = 100;
  int appointmentDurationInt = 0; // from 0 to 11
  OutletType selectedOutletType = OutletType.any;
  int maxCurrent = 0;
  int batteryCapacity = 0;
  bool loading = false;

  static Duration appointmentIntToDuration(int theAppointmentDurationInt) {
    switch (theAppointmentDurationInt) {
      case 0:
        return const Duration(minutes: 5);
      case 1:
        return const Duration(minutes: 10);
      case 2:
        return const Duration(minutes: 15);
      case 3:
        return const Duration(minutes: 30);
      case 4:
        return const Duration(minutes: 45);
      case 5:
        return const Duration(hours: 1);
      case 6:
        return const Duration(hours: 1, minutes: 30);
      case 7:
        return const Duration(hours: 2);
      case 8:
        return const Duration(hours: 3);
      case 9:
        return const Duration(hours: 4);
      case 10:
        return const Duration(hours: 6);
    }
    return const Duration(hours: 12);
  }

  @override
  void initState() {
    super.initState();
    prefs = get<SharedPreferences>();
    final selectedOutletTypeStr = prefs.tryGetString(lastPlugType);
    selectedOutletType = OutletType.values.firstWhere(
            (v) => v.name == selectedOutletTypeStr, orElse: () => OutletType.any);
  }

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;
    final theme = Theme.of(context);
    return Scaffold(
      appBar: AppBar(title: Text(l10n.planYourRoute)),
      body: SingleChildScrollView(
        child: Padding(
          padding: const EdgeInsets.all(16),
          child: Form(
            key: formKey,
            child: Column(
              mainAxisAlignment: MainAxisAlignment.center,
              mainAxisSize: MainAxisSize.min,
              crossAxisAlignment: CrossAxisAlignment.center,
              children: [
                TextFormField(
                  initialValue: l10n.yourLocation,
                  validator: (value) {
                    if (value?.isEmpty ?? true) {
                      return l10n.insertLocation;
                    } else {
                      return null;
                    }
                  },
                  decoration: InputDecoration(
                    prefixIcon: const Icon(Icons.gps_fixed),
                    labelText: l10n.routeSource,
                  ),
                  onSaved: (value) => sourceString = value,
                  textInputAction: TextInputAction.next,
                ),
                const SizedBox(height: 8),
                TextFormField(
                  initialValue:
                      widget.args.destinationName ?? widget.args.destination.toNiceString(),
                  validator: (value) {
                    if (value?.isEmpty ?? true) {
                      return l10n.insertLocation;
                    } else {
                      return null;
                    }
                  },
                  decoration: InputDecoration(
                      prefixIcon: const Icon(Icons.location_on),
                      labelText: l10n.routeDestination,
                  ),
                  onSaved: (value) => destinationString = value,
                  textInputAction: TextInputAction.done,
                ),
                const SizedBox(height: 24),
                Text(l10n.appointmentDuration),
                Slider(
                  value: appointmentDurationInt.toDouble(),
                  min: 0,
                  max: 11,
                  divisions: 11,
                  label: prettyDuration(appointmentIntToDuration(appointmentDurationInt)),
                  onChanged: (double value) {
                    setState(() {
                      appointmentDurationInt = value.round();
                    });
                  },
                ),
                const SizedBox(height: 8),
                Text(l10n.currentBatteryCharge),
                Slider(
                  value: currentBatteryCharge,
                  max: 100,
                  divisions: 100,
                  label: "${currentBatteryCharge.round()}%",
                  onChanged: (double value) {
                    setState(() {
                      currentBatteryCharge = value;
                    });
                  },
                ),
                const SizedBox(height: 8),
                Text(l10n.wantedBatteryCharge),
                Slider(
                  value: wantedBatteryCharge,
                  max: 100,
                  divisions: 100,
                  label: "${wantedBatteryCharge.round()}%",
                  onChanged: (double value) {
                    setState(() {
                      wantedBatteryCharge = value;
                    });
                  },
                ),
                const SizedBox(height: 8),
                DropdownMenu<OutletType>(
                  initialSelection: selectedOutletType == OutletType.any ? null : selectedOutletType,
                  enableFilter: true,
                  requestFocusOnTap: true,
                  leadingIcon: const Icon(Icons.power),
                  label: Text(l10n.outletType),
                  inputDecorationTheme: const InputDecorationTheme(
                    filled: true,
                    contentPadding: EdgeInsets.symmetric(vertical: 5.0),
                  ),
                  onSelected: (OutletType? outletType) {
                    setState(() {
                      if (outletType != null) {
                        selectedOutletType = outletType;
                      }
                    });
                  },
                  dropdownMenuEntries: OutletType.values
                      .map(
                        (outletType) => DropdownMenuEntry(
                          value: outletType,
                          label: outletType.name,
                        ),
                      )
                      .toList(),
                ),
                const SizedBox(height: 8),
                TextFormField(
                  initialValue: prefs.tryGetInt(lastMaxCurrent)?.toString(),
                  validator: (value) {
                    if ((value?.isEmpty ?? true) || int.tryParse(value ?? "") == null) {
                      return l10n.insertNumber;
                    } else {
                      return null;
                    }
                  },
                  decoration: InputDecoration(labelText: l10n.maxCurrent),
                  onSaved: (value) => maxCurrent = int.parse(value!),
                  textInputAction: TextInputAction.done,
                ),
                const SizedBox(height: 8),
                TextFormField(
                  initialValue: prefs.tryGetInt(lastBatteryCapacity)?.toString(),
                  validator: (value) {
                    if ((value?.isEmpty ?? true) || int.tryParse(value ?? "") == null) {
                      return l10n.insertNumber;
                    } else {
                      return null;
                    }
                  },
                  decoration: InputDecoration(labelText: l10n.batteryCapacity),
                  onSaved: (value) => batteryCapacity = int.parse(value!),
                  textInputAction: TextInputAction.done,
                ),
                const SizedBox(height: 16),
                ErrorText(lastError == "" ? null : lastError, (s) => s, topPadding: 8),
                const SizedBox(height: 8),
                if (loading)
                  const CircularProgressIndicator()
                else
                  FilledButton(
                    onPressed: () => submitForm(l10n),
                    child: Text(l10n.calculateRoutes),
                  ),
              ],
            ),
          ),
        ),
      ),
    );
  }

  Pair<LatLng?, String> lastLocationInfoOrError(AppLocalizations l10n) {
    return (get<LocationProvider>().lastLocationInfo().position?.toLatLng())
            ?.map((a) => Pair(a, "")) ??
        Pair(null, l10n.locationNotAvailable);
  }

  Future<Pair<LatLng?, String>> resolveLocationOrError(
      String? str, AppLocalizations l10n, bool destinationIfNull) async {
    if (str == null) {
      // print("str is null");
      if (destinationIfNull) {
        return Pair(widget.args.destination, "");
      } else {
        return lastLocationInfoOrError(l10n);
      }
    } else if (str.trim() == widget.args.destinationName ||
        str.trim() == widget.args.destination.toNiceString()) {
      // print("using destination");
      return Pair(widget.args.destination, "");
    } else if (str == l10n.yourLocation) {
      // print("using location");
      return lastLocationInfoOrError(l10n);
    } else {
      // print("using nominatim");
      final location = await get<Backend>().loadNominatimEntries(str, 1);
      if (location.isEmpty) {
        return Pair(null, l10n.noNominatimMatch(str));
      } else {
        return Pair(location.first.toLatLng(), "");
      }
    }
  }

  void submitForm(AppLocalizations l10n) async {
    setState(() {
      lastError = "";
      loading = true;
    });

    if (formKey.currentState?.validate() != true) {
      setState(() {
        lastError = "";
        loading = false;
      });
      return;
    }

    formKey.currentState?.save();
    final source = await resolveLocationOrError(sourceString, l10n, false);
    if (source.first == null) {
      setState(() {
        lastError = source.second;
        loading = false;
      });
      return;
    }

    final destination = await resolveLocationOrError(destinationString, l10n, true);
    if (destination.first == null) {
      setState(() {
        lastError = destination.second;
        loading = false;
      });
      return;
    }

    print("source=${source.first}, destination=${destination.first}");
    get<Backend>()
        .loadRoutes(
      source.first!,
      destination.first!,
      appointmentIntToDuration(appointmentDurationInt),
      currentBatteryCharge.round(),
      wantedBatteryCharge.round(),
      const Duration(hours: 10000),
      selectedOutletType,
      maxCurrent,
      batteryCapacity,
    )
        .then((routes) {
      if (mounted) {
        prefs.setString(lastPlugType, selectedOutletType.name);
        prefs.setInt(lastMaxCurrent, maxCurrent);
        prefs.setInt(lastBatteryCapacity, batteryCapacity);
        Navigator.pop(context, routes);
      }
    }, onError: (e) {
      setState(() {
        lastError = e.toString();
        loading = false;
      });
    });
  }
}
