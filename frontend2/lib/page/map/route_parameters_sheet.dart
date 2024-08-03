showModalBottomSheet(context: context, builder: (BuildContext context) {
return Padding(padding: EdgeInsets.all(16), child: SizedBox(
height: 800,
child: Column(
children: [
Padding(padding: EdgeInsets.all(0), child: TextField(
decoration: InputDecoration(
prefixIcon: Icon(Icons.gps_fixed),
suffix: Icon(Icons.clear),
hintText: "From",
),
),),
const Padding(padding: EdgeInsets.all(4)),
Padding(padding: EdgeInsets.all(0), child: TextField(
decoration: InputDecoration(
prefixIcon: Icon(Icons.location_on),
suffix: Icon(Icons.clear),
hintText: "To",
),
),),
const Padding(padding: EdgeInsets.all(16)),
Text("Appointment duration"),
Slider(
value: 20,
max: 100,
divisions: 5,
label: "appoooointment", onChanged: (double value) {  },
),
Text("Current Charge"),
Slider(
value: 20,
max: 100,
divisions: 5,
label: "appoooointment", onChanged: (double value) {  },
),
Text("Preferred final charge"),
Slider(
value: 20,
max: 100,
divisions: 5,
label: "appoooointment", onChanged: (double value) {  },
),
],
),
),);
});