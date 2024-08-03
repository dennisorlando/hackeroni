import 'package:flutter/material.dart';
import 'package:get_it_mixin/get_it_mixin.dart';

import '../../networking/data/route.dart';

class BottomChip extends StatefulWidget with GetItStatefulWidgetMixin {
  final Map<RouteAlgorithm, RouteData>? data;

  final Function() close;

  BottomChip(this.data, this.close, {super.key});

  @override
  State<StatefulWidget> createState() => BottomChipState();
}

class BottomChipState extends State<BottomChip>
    with GetItStateMixin<BottomChip> {

  RouteAlgorithm selectedAlgorithm = RouteAlgorithm.lessWalking;

  @override
  Widget build(BuildContext context) {
    return Visibility(
      visible: widget.data != null,
      child: Card(
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
                    child: SegmentedButton<RouteAlgorithm>(
                        segments: const <ButtonSegment<RouteAlgorithm>>[
                          ButtonSegment(
                            value: RouteAlgorithm.lessWalking,
                            label: Text("Fast"),
                            icon: Icon(Icons.ice_skating),
                          ),
                          ButtonSegment(
                              value: RouteAlgorithm.moreCharging,
                              label: Text("And"),
                              icon: Icon(Icons.run_circle)),
                          ButtonSegment(
                              value: RouteAlgorithm.lessDriving,
                              label: Text("Furious"),
                              icon: Icon(Icons.fire_truck))
                        ],
                        onSelectionChanged: (Set<RouteAlgorithm> newSelection) {
                          setState(() {
                            selectedAlgorithm = newSelection.first;
                          });
                        },
                        selected: <RouteAlgorithm>{selectedAlgorithm}
                    ),
                  ),
                  const Padding(padding: EdgeInsets.all(4)),
                  OutlinedButton(onPressed: () {
                    widget.close();
                  }, child: const Icon(Icons.close)),
                ],
              ),
              const Padding(padding: EdgeInsets.all(8),),
              Align(
                  alignment: Alignment.centerLeft,
                  child: Text(
                    " • Final Charge: ${widget.data?[selectedAlgorithm]?.finalCharge}%\n"
                        " • Walking Distance: ${_prettyDuration(widget.data?[selectedAlgorithm]?.walkingDistance ?? Duration.zero)}\n"
                        " • Driving Distance: ${_prettyDuration(widget.data?[selectedAlgorithm]?.drivingDistance ?? Duration.zero)}\n"
                        " • Cost: ${widget.data?[selectedAlgorithm]?.cost}€",
                    textAlign: TextAlign.left,
                    style: const TextStyle(
                      fontSize: 20,
                    ),
                  )
              )
            ]),
          ),
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
    var centiseconds =
        (duration.inMilliseconds % 1000) ~/ 10;
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
