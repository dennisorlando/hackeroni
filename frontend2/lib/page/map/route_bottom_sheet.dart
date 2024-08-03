import 'package:flutter/material.dart';
import 'package:flutter_gen/gen_l10n/app_localizations.dart';
import 'package:get_it_mixin/get_it_mixin.dart';
import 'package:evplanner_frontend/util/time.dart';

import '../../networking/data/route.dart';

class RouteBottomSheet extends StatefulWidget with GetItStatefulWidgetMixin {
  final Map<RouteAlgorithm, RouteData>? data;
  final Function() close;
  final RouteAlgorithm selectedRouteAlgorithm;
  final Function(RouteAlgorithm p1) setSelectedRouteAlgorithm;

  RouteBottomSheet(
      this.data, this.close, this.selectedRouteAlgorithm, this.setSelectedRouteAlgorithm,
      {super.key});

  @override
  State<StatefulWidget> createState() => RouteBottomSheetState();
}

class RouteBottomSheetState extends State<RouteBottomSheet> with GetItStateMixin<RouteBottomSheet> {
  @override
  Widget build(BuildContext context) {
    final Map<RouteAlgorithm, RouteData>? data = widget.data;
    if (data == null) {
      return const SizedBox.shrink();
    }
    final l10n = AppLocalizations.of(context)!;
    final selectedData = data[widget.selectedRouteAlgorithm];

    return Wrap(
      children: [
        Card(
          elevation: 1,
          child: Padding(
            padding: const EdgeInsets.all(8),
            child: Column(
              children: [
                Row(
                  children: [
                    Expanded(
                      child: SingleChildScrollView(
                        scrollDirection: Axis.horizontal,
                        child: Row(
                          mainAxisSize: MainAxisSize.min,
                          children: data.keys
                              .map(
                                (key) => (key != widget.selectedRouteAlgorithm)
                                    ? IconButton(
                                        onPressed: () => widget.setSelectedRouteAlgorithm(key),
                                        icon: Icon(key.icon),
                                      )
                                    : ElevatedButton.icon(
                                        label: Text(key.label(l10n)),
                                        icon: Icon(key.icon),
                                        onPressed: () {},
                                      ),
                              )
                              .toList(),
                        ),
                      ),
                    ),
                    const Padding(padding: EdgeInsets.all(4)),
                    IconButton(
                      onPressed: () {
                        widget.close();
                      },
                      icon: const Icon(Icons.close),
                    ),
                  ],
                ),
                const Divider(),
                const Padding(
                  padding: EdgeInsets.all(8),
                ),
                Row(
                  children: [
                    const Padding(
                      padding: EdgeInsets.all(6),
                    ),
                    Expanded(
                      child: Column(
                        children: [
                          Chip(
                            avatar: const Icon(Icons.battery_charging_full),
                            label: Center(
                              child: Text(
                                "${selectedData?.finalCharge ?? 0}%",
                                textScaler: const TextScaler.linear(1.2),
                              ),
                            ),
                          ),
                          const Padding(
                            padding: EdgeInsets.all(6),
                          ),
                          Chip(
                            avatar: const Icon(Icons.directions_walk),
                            label: Center(
                              child: Text(
                                prettyDuration(selectedData?.walkingDistance ?? Duration.zero),
                                textScaler: const TextScaler.linear(1.2),
                              ),
                            ),
                          ),
                        ],
                      ),
                    ),
                    const Padding(
                      padding: EdgeInsets.all(8),
                    ),
                    Expanded(
                      child: Column(
                        children: [
                          Chip(
                            avatar: const Icon(Icons.euro),
                            label: Center(
                              child: Text(
                                "${selectedData?.cost ?? 0}€",
                                textScaler: const TextScaler.linear(1.2),
                              ),
                            ),
                          ),
                          const Padding(
                            padding: EdgeInsets.all(6),
                          ),
                          Chip(
                            avatar: const Icon(Icons.directions_car),
                            label: Center(
                              child: Text(
                                prettyDuration(selectedData?.drivingDistance ?? Duration.zero),
                                textScaler: const TextScaler.linear(1.2),
                              ),
                            ),
                          ),
                        ],
                      ),
                    ),
                    const Padding(
                      padding: EdgeInsets.all(6),
                    ),
                  ],
                ),
                const Padding(
                  padding: EdgeInsets.all(8),
                ),
              ],
            ),
          ),
        ),
      ],
    );
  }
}
