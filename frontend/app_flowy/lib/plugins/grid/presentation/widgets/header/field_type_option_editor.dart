import 'dart:typed_data';
import 'package:app_flowy/plugins/grid/application/field/type_option/type_option_data_controller.dart';
import 'package:appflowy_popover/popover.dart';
import 'package:dartz/dartz.dart' show Either;
import 'package:flowy_infra/image.dart';
import 'package:flowy_infra/theme.dart';
import 'package:flowy_infra_ui/flowy_infra_ui.dart';
import 'package:flowy_infra_ui/style_widget/button.dart';
import 'package:flowy_infra_ui/style_widget/text.dart';
import 'package:flowy_sdk/protobuf/flowy-error/errors.pb.dart';
import 'package:flowy_sdk/protobuf/flowy-grid/field_entities.pb.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:app_flowy/plugins/grid/application/prelude.dart';
import '../../layout/sizes.dart';
import 'field_type_extension.dart';
import 'field_type_list.dart';
import 'type_option/builder.dart';

typedef UpdateFieldCallback = void Function(FieldPB, Uint8List);
typedef SwitchToFieldCallback
    = Future<Either<FieldTypeOptionDataPB, FlowyError>> Function(
  String fieldId,
  FieldType fieldType,
);

class FieldTypeOptionEditor extends StatelessWidget {
  final TypeOptionDataController dataController;
  final PopoverMutex popoverMutex;

  const FieldTypeOptionEditor({
    required this.dataController,
    required this.popoverMutex,
    Key? key,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return BlocProvider(
      create: (context) => FieldTypeOptionEditBloc(dataController)
        ..add(const FieldTypeOptionEditEvent.initial()),
      child: BlocBuilder<FieldTypeOptionEditBloc, FieldTypeOptionEditState>(
        builder: (context, state) {
          List<Widget> children = [
            _switchFieldTypeButton(context, dataController.field)
          ];
          final typeOptionWidget =
              _typeOptionWidget(context: context, state: state);

          if (typeOptionWidget != null) {
            children.add(typeOptionWidget);
          }

          return ListView(
            shrinkWrap: true,
            children: children,
          );
        },
      ),
    );
  }

  Widget _switchFieldTypeButton(BuildContext context, FieldPB field) {
    final theme = context.watch<AppTheme>();
    return SizedBox(
      height: GridSize.typeOptionItemHeight,
      child: Popover(
        triggerActions:
            PopoverTriggerActionFlags.hover | PopoverTriggerActionFlags.click,
        mutex: popoverMutex,
        offset: const Offset(20, 0),
        targetAnchor: Alignment.topRight,
        followerAnchor: Alignment.topLeft,
        popupBuilder: (context) {
          final list = FieldTypeList(onSelectField: (newFieldType) {
            dataController.switchToField(newFieldType);
          });
          return OverlayContainer(
            constraints: BoxConstraints.loose(const Size(460, 440)),
            child: list,
          );
        },
        child: FlowyButton(
          text: FlowyText.medium(field.fieldType.title(), fontSize: 12),
          margin: const EdgeInsets.symmetric(horizontal: 6, vertical: 2),
          hoverColor: theme.hover,
          leftIcon:
              svgWidget(field.fieldType.iconName(), color: theme.iconColor),
          rightIcon: svgWidget("grid/more", color: theme.iconColor),
        ),
      ),
    );
  }

  Widget? _typeOptionWidget({
    required BuildContext context,
    required FieldTypeOptionEditState state,
  }) {
    final overlayDelegate = TypeOptionOverlayDelegate(
      showOverlay: _showOverlay,
      hideOverlay: _hideOverlay,
    );

    return makeTypeOptionWidget(
      context: context,
      overlayDelegate: overlayDelegate,
      dataController: dataController,
      popoverMutex: popoverMutex,
    );
  }

  void _showOverlay(BuildContext context, Widget child,
      {VoidCallback? onRemoved}) {
    // FlowyPopover.show(
    //   context,
    //   constraints: BoxConstraints.loose(const Size(460, 440)),
    //   anchorContext: context,
    //   anchorDirection: AnchorDirection.rightWithCenterAligned,
    //   anchorOffset: const Offset(20, 0),
    //   builder: (BuildContext context) {
    //     return child;
    //   },
    // );
  }

  void _hideOverlay(BuildContext context) {
    //
  }
}

abstract class TypeOptionWidget extends StatelessWidget {
  const TypeOptionWidget({Key? key}) : super(key: key);
}
