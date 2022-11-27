use bevy::prelude::{Plugin, App, Commands, Component};
use kayak_ui::{prelude::{KayakContextPlugin, KayakRootContext}, widgets::KayakWidgets, UICameraBundle};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
      /*  app
        .add_plugin(KayakContextPlugin)
        .add_plugin(KayakWidgets)
        .add_startup_system(jwst_ui);*/ //TODO
    }
}

fn jwst_ui(
    mut commands: Commands
) {
  //  commands.spawn(UICameraBundle::new());
  //  let mut context = KayakRootContext::new();
//    context.add_plugin(KayakW)
   // let parent_id = None;    
}

#[derive(Debug, Clone, PartialEq, Eq, Component)]
pub enum Menu {
    Main,
    Body,
}