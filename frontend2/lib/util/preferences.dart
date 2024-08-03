import 'package:shared_preferences/shared_preferences.dart';

extension SharedPreferencesExt on SharedPreferences {
  int? tryGetInt(String key) {
    try {
      return getInt(key);
    } catch (e) {
      return null;
    }
  }
  String? tryGetString(String key) {
    try {
      return getString(key);
    } catch (e) {
      return null;
    }
  }
}
