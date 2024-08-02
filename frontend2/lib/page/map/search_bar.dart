import 'package:flutter/material.dart';
import 'package:get_it_mixin/get_it_mixin.dart';
import 'package:insigno_frontend/networking/backend.dart';
import 'package:insigno_frontend/networking/data/osm_nominatim_entry.dart';

/// Flutter code sample for [SearchBar].

class SearchBarApp extends StatefulWidget with GetItStatefulWidgetMixin {
  SearchBarApp({super.key});

  @override
  State<SearchBarApp> createState() => _SearchBarAppState();
}

class _SearchBarAppState extends State<SearchBarApp> with GetItStateMixin<SearchBarApp> {
  bool isDark = false;
  List<ListTile> lastSuggestions = List.empty();

  @override
  Widget build(BuildContext context) {
    final mediaQuery = MediaQuery.of(context);

    return Padding(
      padding: EdgeInsets.fromLTRB(
        8 + mediaQuery.padding.left,
        8 + mediaQuery.padding.top,
        8 + mediaQuery.padding.right,
        0,
      ),
      child: SearchAnchor(builder: (BuildContext context, SearchController controller) {
        return SearchBar(
          controller: controller,
          padding: const WidgetStatePropertyAll<EdgeInsets>(EdgeInsets.symmetric(horizontal: 16.0)),
          onTap: () {
            controller.openView();
          },
          onChanged: (_) {
            controller.openView();
          },
          leading: const Icon(Icons.search),
          // trailing: <Widget>[
          //   Tooltip(
          //     message: 'Change brightness mode',
          //     child: IconButton(
          //       isSelected: isDark,
          //       onPressed: () {
          //         setState(() {
          //           isDark = !isDark;
          //         });
          //       },
          //       icon: const Icon(Icons.wb_sunny_outlined),
          //       selectedIcon: const Icon(Icons.brightness_2_outlined),
          //     ),
          //   )
          // ],
        );
      }, suggestionsBuilder: (BuildContext context, SearchController controller) async {
        final initialText = controller.text;
        await Future.delayed(const Duration(milliseconds: 300));

        //print("controller=${controller.text} initial=$initialText");
        if (initialText != controller.text) {
          return lastSuggestions;
        }

        //print("Faccio la richiesta");
        final newSuggestions = await get<Backend>().loadNominatimEntries(controller.text);
        if (initialText != controller.text) {
          return lastSuggestions;
        }
        
        lastSuggestions = newSuggestions.map<ListTile>((item) => ListTile(
              title: Text(item.displayName),
              onTap: () {
                setState(() {
                  controller.closeView(item.displayName);
                });
              },
            )).toList();

        return lastSuggestions;
        // return List<ListTile>.generate(entries.length, (int index) {
        //   final String item = entries[index].displayName;
        //   return ListTile(
        //     title: Text(item),
        //     onTap: () {
        //       setState(() {
        //         controller.closeView(item);
        //       });
        //     },
        //   );
        // });
      }),
    );
  }
}

List<ListTile> searchBarSuggestionBuilder() {
  List<ListTile> list = List<ListTile>.empty(growable: true);

  return list;
}
