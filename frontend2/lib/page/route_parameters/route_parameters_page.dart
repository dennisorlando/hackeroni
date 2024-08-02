import 'package:flutter/material.dart';
import 'package:flutter_gen/gen_l10n/app_localizations.dart';
import 'package:get_it_mixin/get_it_mixin.dart';
import 'package:latlong2/latlong.dart';

class RouteParametersPage extends StatefulWidget with GetItStatefulWidgetMixin {
  static const routeName = '/routeParametersPage';

  RouteParametersPage(RouteParametersPageArgs args, {super.key});

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

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;
    final theme = Theme.of(context);
    return Scaffold(
      appBar: AppBar(
        title: Text(l10n.planYourRoute)
      ),
      body: Center(
        child: SingleChildScrollView(
          child: Column(

          ),
        ),
      ),
    );
  }
}
