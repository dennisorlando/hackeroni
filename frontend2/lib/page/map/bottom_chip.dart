import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:get_it_mixin/get_it_mixin.dart';

class BottomChip extends StatefulWidget with GetItStatefulWidgetMixin {
  BottomChip({super.key});

  @override
  State<StatefulWidget> createState() => BottomChipState();
}

enum SelectedAlgorithm { fast, and, furious }

class RouteData {
  
  RouteData(this.finalCharge, this.walkingDistanceMinutes, this.drivingDistanceMinutes);
  
  int finalCharge;
  int walkingDistanceMinutes;
  int drivingDistanceMinutes;
  int cost = -1;
  
}

class BottomChipState extends State<BottomChip>
    with GetItStateMixin<BottomChip> {

  bool visible = true;

  RouteData data = RouteData(0, 0, 0);

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
          child: Column(children: [
            SegmentedButton<SelectedAlgorithm>(
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
              selected: <SelectedAlgorithm>{selectedAlgorithm}),
              Align(
                alignment: Alignment.centerLeft,
                child: Text(
                  "Final Charge: ${data.finalCharge}\n"
                      "Walking Distance: ${data.walkingDistanceMinutes}m\n"
                      "Driving Distance: ${data.drivingDistanceMinutes}m\n"
                      "Cost: €€€",
                  textAlign: TextAlign.left,
                  style: const TextStyle(
                    fontSize: 20,
                  ),
                )
              )
          ]),
        ),
      ),
    );
  }
}
