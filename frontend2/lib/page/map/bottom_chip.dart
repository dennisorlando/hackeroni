import 'package:flutter/material.dart';
import 'package:get_it_mixin/get_it_mixin.dart';
import 'package:latlong2/latlong.dart';

class BottomChip extends StatefulWidget with GetItStatefulWidgetMixin {
  BottomChip({super.key});

  @override
  State<StatefulWidget> createState() => BottomChipState();
}

enum SelectedAlgorithm { fast, and, furious }

class RouteData {
  
  RouteData(this.finalCharge, this.walkingDistance, this.drivingDistance, this.walkingPath, this.drivingPath);
  
  int finalCharge;
  Duration walkingDistance;
  Duration drivingDistance;
  int cost = -1;

  List<LatLng> walkingPath;
  List<LatLng> drivingPath;

}

class BottomChipState extends State<BottomChip>
    with GetItStateMixin<BottomChip> {

  void loadDataAndRefresh(Map<SelectedAlgorithm, RouteData> newData) {
    setState(() {
      visible = true;
      data = newData;
    });
  }

  bool visible = false;

  Map<SelectedAlgorithm, RouteData> data = {};

  SelectedAlgorithm selectedAlgorithm = SelectedAlgorithm.fast;

  @override
  Widget build(BuildContext context) {
    return Visibility(
      visible: visible,
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
                    child: SegmentedButton<SelectedAlgorithm>(
                        segments: const <ButtonSegment<SelectedAlgorithm>>[
                          ButtonSegment(
                            value: SelectedAlgorithm.fast,
                            label: Text("Fast"),
                            icon: Icon(Icons.ice_skating),
                          ),
                          ButtonSegment(
                              value: SelectedAlgorithm.and,
                              label: Text("And"),
                              icon: Icon(Icons.run_circle)),
                          ButtonSegment(
                              value: SelectedAlgorithm.furious,
                              label: Text("Furious"),
                              icon: Icon(Icons.fire_truck))
                        ],
                        onSelectionChanged: (Set<SelectedAlgorithm> newSelection) {
                          setState(() {
                            selectedAlgorithm = newSelection.first;
                          });
                        },
                        selected: <SelectedAlgorithm>{selectedAlgorithm}
                    ),
                  ),
                  const Padding(padding: EdgeInsets.all(4)),
                  OutlinedButton(onPressed: () {
                    setState(() {
                      visible = false;
                    });
                  }, child: const Icon(Icons.close)),
                ],
              ),
              const Padding(padding: EdgeInsets.all(8),),
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
