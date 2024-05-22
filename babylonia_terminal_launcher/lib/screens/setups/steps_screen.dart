import 'package:flutter/material.dart';
import 'package:flutter/widgets.dart';
import 'package:provider/provider.dart';

import './../../messages/game_state.pb.dart';
import './../../providers/providers.dart';
import './../../widgets/selectable_yaru_expansion_panel.dart';
import './../../widgets/simple_button.dart';

class StepsScreen extends StatelessWidget {
  StepsScreen({super.key});
  final SectionController controller = SectionController(selectedItem: 0);

  @override
  Widget build(BuildContext context) {
    final gameStateProvider = Provider.of<GameStateProvider>(context);
    switch (gameStateProvider.gameState) {
      case States.ProtonNotInstalled:
        controller.updateSection(0);
        break;
      case States.DXVKNotInstalled:
        controller.updateSection(1);
        break;
      case States.DependecieNotInstalled:
        controller.updateSection(2);
        break;
    }

    return Center(
      child: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          ConstrainedBox(
            constraints: const BoxConstraints(
              maxHeight: 600,
              maxWidth: 750,
            ),
            child: Padding(
              padding: const EdgeInsets.symmetric(horizontal: 25.0),
              child: SelectableYaruExpansionPanel(
                controller: controller,
                headers: const [
                  Text("Proton"),
                  Text("DXVK"),
                  Text("Dependecies"),
                ],
                children: const [
                  Padding(
                    padding: EdgeInsets.all(40.0),
                    child: Text("Proton"),
                  ),
                  Padding(
                    padding: EdgeInsets.all(40.0),
                    child: Text("DXVK"),
                  ),
                  Padding(
                    padding: EdgeInsets.all(40.0),
                    child: Text("Dependecies"),
                  ),
                ],
              ),
            ),
          ),
          SimpleButton(
            onPressed: () => gameStateProvider.updateGameState(),
            child: const Text('update'),
          ),
        ],
      ),
    );
  }
}