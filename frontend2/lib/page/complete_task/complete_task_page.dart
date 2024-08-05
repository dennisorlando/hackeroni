import 'dart:typed_data';

import 'package:evplanner_frontend/networking/backend.dart';
import 'package:evplanner_frontend/networking/data/marker_type.dart';
import 'package:evplanner_frontend/page/map/target.dart';
import 'package:evplanner_frontend/page/marker/add_images_widget.dart';
import 'package:flutter/material.dart';
import 'package:flutter_gen/gen_l10n/app_localizations.dart';
import 'package:get_it_mixin/get_it_mixin.dart';
import 'package:latlong2/latlong.dart';

import '../../util/pair.dart';

class CompleteTaskPage extends StatefulWidget with GetItStatefulWidgetMixin {
  static const routeName = '/completeTaskPage';

  final Target target;

  CompleteTaskPage(this.target, {super.key});

  @override
  State<CompleteTaskPage> createState() => _CompleteTaskPageState();
}

class _CompleteTaskPageState extends State<CompleteTaskPage>
    with GetItStateMixin<CompleteTaskPage> {
  List<Pair<Uint8List, String?>> images = [];
  MarkerType? markerType;
  bool loading = false;
  String? error;

  @override
  void initState() {
    super.initState();
  }

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;
    final theme = Theme.of(context);
    return Scaffold(
      appBar: AppBar(
        centerTitle: true,
        title: Text(l10n.report),
      ),
      body: Center(
        child: SingleChildScrollView(
          child: Column(
            mainAxisSize: MainAxisSize.min,
            children: [
              AddImagesWidget(
                images,
                loading ? null : (image) => setState(() => images.add(image)),
                loading ? null : (index) => setState(() => images.removeAt(index)),
              ),
              const SizedBox(height: 16),
              TextButton(
                onPressed: (images.isEmpty ? null : sendData),
                child: const Text("Carica"),
              )
            ],
          ),
        ),
      ),
    );
  }

  void sendData() async {
    for (final image in images) {
      await get<Backend>().addMarkerImage2(image.first, image.second, widget.target);
    }
    if (mounted) {
      Navigator.pop(context, true);
    }
  }
}
