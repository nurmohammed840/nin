import 'package:flutter/material.dart';
import 'package:localpost/src/rust/api/simple.dart';
import 'package:localpost/src/rust/frb_generated.dart';
import 'package:localpost/utils.dart';

Future<void> main() async {
  await RustLib.init();
  final addr = await getLocalIp();
  runApp(MyApp(addr: addr));
}

class MyApp extends StatelessWidget {
  const MyApp({super.key, required this.addr});

  final String addr;

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      themeMode: .dark,
      theme: .light(),
      darkTheme: .dark(),
      title: 'LocalPost',
      debugShowCheckedModeBanner: false,
      home: HomePage(addr: addr),
    );
  }
}

class HomePage extends StatelessWidget {
  const HomePage({super.key, required this.addr});

  final String addr;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Column(
          crossAxisAlignment: .start,
          children: [
            const Text('LocalPost'),
            Text(
              addr,
              style: const TextStyle(fontSize: 12, color: Colors.grey),
            ),
          ],
        ),
        actionsPadding: const .only(right: 10),
        actions: [
          IconButton(
            tooltip: 'Settings',
            icon: const Icon(Icons.settings_outlined),
            onPressed: () {},
          ),
        ],
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: () async {
          final ip = await promptIPAddr(context);
          if (ip == null) {
            return;
          }
        },
        tooltip: 'Connect',
        child: const Icon(Icons.link),
      ),
    );
  }
}

Future<String?> promptIPAddr(BuildContext context) async {
  final controller = TextEditingController(text: '192.168.0.1');

  return showDialog<String>(
    context: context,
    builder: (_) => AlertDialog(
      title: const Text('Connect Device'),
      content: TextField(
        controller: controller,
        autofocus: true,
        keyboardType: .number,
        inputFormatters: [IPv4Formatter()],
        decoration: const InputDecoration(labelText: 'IP address'),
      ),
      actions: [
        TextButton(
          onPressed: () => Navigator.pop(context, controller.text),
          child: const Text('Connect'),
        ),
      ],
    ),
  );
}
