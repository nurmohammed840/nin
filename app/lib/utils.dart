import 'package:flutter/services.dart';

class IPv4Formatter extends TextInputFormatter {
  @override
  TextEditingValue formatEditUpdate(old, val) {
    if (val.text.contains(' ')) return old;
    if (val.text.contains('..')) return old;

    final parts = val.text.split('.');

    if (parts.length > 4) return old;

    for (final part in parts) {
      if (part.startsWith('0') && part.length > 1) return old;
      if (int.tryParse(part) != null && int.parse(part) > 255) return old;
    }

    return val;
  }
}
