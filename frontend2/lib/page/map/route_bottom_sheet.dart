import 'package:flutter/material.dart';
import 'package:flutter_gen/gen_l10n/app_localizations.dart';
import 'package:get_it_mixin/get_it_mixin.dart';

import '../../networking/data/route.dart';

class RouteBottomSheet extends StatefulWidget with GetItStatefulWidgetMixin {
  final Map<RouteAlgorithm, RouteData>? data;

  final Function() close;

  RouteBottomSheet(this.data, this.close, {super.key});

  @override
  State<StatefulWidget> createState() => RouteBottomSheetState();
}

class RouteBottomSheetState extends State<RouteBottomSheet> with GetItStateMixin<RouteBottomSheet> {
  RouteAlgorithm selectedAlgorithm = RouteAlgorithm.balanced;

  @override
  Widget build(BuildContext context) {
    final Map<RouteAlgorithm, RouteData>? data = widget.data;
    if (data == null) {
      return const SizedBox.shrink();
    }
    final l10n = AppLocalizations.of(context)!;

    return Card(
      elevation: 1,
      child: FractionallySizedBox(
        heightFactor: 0.3,
        widthFactor: 0.975,
        child: Padding(
          padding: const EdgeInsets.all(8),
          child: Column(children: [
            Row(
              children: [
                Expanded(
                  child: SingleChildScrollView(
                    scrollDirection: Axis.horizontal,
                    child: Row(
                      mainAxisSize: MainAxisSize.min,
                      children: data.keys
                          .map(
                            (key) => (key != selectedAlgorithm)
                                ? IconButton(
                                    onPressed: () => setState(() => selectedAlgorithm = key),
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
            const Padding(
              padding: EdgeInsets.all(8),
            ),
            Align(
                alignment: Alignment.centerLeft,
                child: Text(
                  " • Final Charge: ${data[selectedAlgorithm]?.finalCharge}%\n"
                  " • Walking Distance: ${_prettyDuration(data[selectedAlgorithm]?.walkingDistance ?? Duration.zero)}\n"
                  " • Driving Distance: ${_prettyDuration(data[selectedAlgorithm]?.drivingDistance ?? Duration.zero)}\n"
                  " • Cost: ${data[selectedAlgorithm]?.cost}€",
                  textAlign: TextAlign.left,
                  style: const TextStyle(
                    fontSize: 20,
                  ),
                ))
          ]),
        ),
      ),
    );
  }

  static String _prettyDuration(Duration duration) {
    var components = <String>[];

    var days = duration.inDays;
    if (days != 0) {
      components.add('${days}d');
    }
    var hours = duration.inHours % 24;
    if (hours != 0) {
      components.add('${hours}h');
    }
    var minutes = duration.inMinutes % 60;
    if (minutes != 0) {
      components.add('${minutes}m');
    }

    var seconds = duration.inSeconds % 60;
    var centiseconds = (duration.inMilliseconds % 1000) ~/ 10;
    if (components.isEmpty || seconds != 0 || centiseconds != 0) {
      components.add('$seconds');
      if (centiseconds != 0) {
        components.add('.');
        components.add(centiseconds.toString().padLeft(2, '0'));
      }
      components.add('s');
    }
    return components.join();
  }
}
