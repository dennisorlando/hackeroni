import 'package:flutter/material.dart';
import 'package:flutter_gen/gen_l10n/app_localizations.dart';

enum MarkerType {
  // the comments have the alternative icons
  // useful icons for the future: oil_barrel
  unknown(1, Colors.grey, Icons.help_outline),
  alperia(2, Colors.indigo, Icons.charging_station); // electric_bolt

  final int id;
  final Color color;
  final IconData icon;

  const MarkerType(this.id, this.color, this.icon);

  Icon getThemedIcon(final BuildContext context) {
    return Icon(icon,
        color: HSLColor.fromColor(color)
            .withLightness(Theme.of(context).brightness == Brightness.dark ? 0.7 : 0.3)
            .toColor());
  }

  String getName(final BuildContext context) {
    final l10n = AppLocalizations.of(context)!;
    switch (this) {
      case MarkerType.unknown:
        return l10n.markerTypeUnknown;
      case MarkerType.alperia:
        return l10n.markerTypeAlperia;
    }
  }
}
