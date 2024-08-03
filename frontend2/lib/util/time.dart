String formatDuration(Duration duration) {
  final negative = duration.isNegative;
  if (negative) {
    duration = -duration;
  }

  final hours = (duration.inHours == 0) ? "" : "${duration.inHours.toString()}:";
  final minutes = duration.inMinutes.remainder(60).toString();
  final seconds = duration.inSeconds.remainder(60).toString().padLeft(2, '0');
  return "${negative ? "-" : ""}$hours${duration.inHours == 0 ? minutes : minutes.padLeft(2, '0')}:$seconds";
}

String prettyDuration(Duration duration) {
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
