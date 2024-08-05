import 'package:evplanner_frontend/page/complete_task/complete_task_page.dart';
import 'package:flutter/material.dart';
import 'package:flutter_gen/gen_l10n/app_localizations.dart';
import 'package:evplanner_frontend/networking/data/map_marker.dart';
import 'package:evplanner_frontend/networking/data/pill.dart';
import 'package:evplanner_frontend/page/error_page.dart';
import 'package:evplanner_frontend/page/introduction_page.dart';
import 'package:evplanner_frontend/page/map/map_page.dart';
import 'package:evplanner_frontend/page/pill_page.dart';
import 'package:evplanner_frontend/page/route_parameters/route_parameters_page.dart';
import 'package:evplanner_frontend/page/settings/settings_page.dart';
import 'package:latlong2/latlong.dart';

class InsignoApp extends StatefulWidget {
  final GlobalKey<NavigatorState> navigatorKey;
  final GlobalKey<ScaffoldMessengerState> scaffoldMessengerKey;

  const InsignoApp(this.navigatorKey, this.scaffoldMessengerKey, {super.key});

  @override
  State<InsignoApp> createState() => _InsignoAppState();
}

class _InsignoAppState extends State<InsignoApp> {
  @override
  Widget build(BuildContext context) {
    final lightYellowTheme = ColorScheme.fromSeed(
      seedColor: Colors.yellow,
      brightness: Brightness.light,
    );
    final darkYellowTheme = ColorScheme.fromSeed(
      seedColor: Colors.yellow,
      brightness: Brightness.dark,
    );

    return MaterialApp(
      scaffoldMessengerKey: widget.scaffoldMessengerKey,
      debugShowCheckedModeBanner: false,
      navigatorKey: widget.navigatorKey,
      title: "EVPlanner.rs",
      theme: ThemeData(
        useMaterial3: true,
        colorScheme: ColorScheme.fromSeed(
          seedColor: Colors.green,
          brightness: Brightness.light,
          tertiaryContainer: lightYellowTheme.primaryContainer,
          onTertiaryContainer: lightYellowTheme.onPrimaryContainer,
        ),
      ),
      darkTheme: ThemeData(
        useMaterial3: true,
        colorScheme: ColorScheme.fromSeed(
          seedColor: Colors.green,
          brightness: Brightness.dark,
          tertiaryContainer: darkYellowTheme.primaryContainer,
          onTertiaryContainer: darkYellowTheme.onPrimaryContainer,
        ),
      ),
      home: MapPage(),
      onGenerateRoute: (RouteSettings settings) {
        var routes = <String, WidgetBuilder>{
          ErrorPage.routeName: (ctx) => ErrorPage(settings.arguments as FlutterErrorDetails),
          SettingsPage.routeName: (ctx) => SettingsPage(),
          IntroductionPage.routeName: (ctx) =>
              IntroductionPage(onDone: (context) => Navigator.pop(context)),
          RouteParametersPage.routeName: (ctx) => RouteParametersPage(settings.arguments as RouteParametersPageArgs),
          CompleteTaskPage.routeName: (ctx) => CompleteTaskPage(settings.arguments as CompleteTaskPageArgs),
        };
        WidgetBuilder builder = routes[settings.name]!;
        return MaterialPageRoute(builder: (ctx) => builder(ctx));
      },
      localizationsDelegates: AppLocalizations.localizationsDelegates,
      supportedLocales: AppLocalizations.supportedLocales,
    );
  }
}
