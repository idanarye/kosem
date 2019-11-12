use std::rc::Rc;
use std::collections::HashMap;

use actix::prelude::*;
use gtk::prelude::*;

use kosem_webapi::Uuid;
use kosem_webapi::phase_control_messages::{
    self,
    ComponentParams,
};

use crate::actors::gui::GuiActor;
use crate::gtk_gui::GladeFactories;
use crate::internal_messages::gui_control::{
    MessageToProcedureScreen,
    WindowClosed,
    UserClickedButton,
};

pub struct WorkOnProcedureWindow {
    #[allow(unused)]
    gui_actor: Addr<GuiActor>,
    window: gtk::ApplicationWindow,
    #[allow(unused)]
    factories: Rc<GladeFactories>,
    server_idx: usize,
    #[allow(unused)]
    request_uid: Uuid,
    phases_list: gtk::ListBox,
    phases: HashMap<Uuid, Phase>,
}

impl WorkOnProcedureWindow {
    pub fn create(
        gui_actor: Addr<GuiActor>,
        factories: Rc<GladeFactories>,
        available_procedure_msg: kosem_webapi::pairing_messages::AvailableProcedure,
        server_idx: usize,
    ) -> Self {
        let window_builder = factories.work_on_procedure.window.build();

        window_builder.get_object::<gtk::Label>("title").set_text(&available_procedure_msg.name);

        let phases_list = window_builder.get_object("phases_list");

        let window: gtk::ApplicationWindow = window_builder.get();

        window.connect_delete_event({
            let gui_actor = gui_actor.clone();
            move |_window, _evt| {
                gui_actor.do_send(WindowClosed::ProcedureScreen {
                    server_idx,
                });
                Inhibit(false)
            }
        });

        Self {
            gui_actor,
            window,
            factories,
            server_idx,
            request_uid: available_procedure_msg.uid,
            phases_list,
            phases: Default::default(),
        }
    }

    pub fn activate(&self) {
        self.window.show_all();
    }

    pub fn message_received(&mut self, msg: MessageToProcedureScreen) {
        log::info!("Procedure screen got message {:?}", msg);
        match msg {
            MessageToProcedureScreen::PhasePushed(msg) => {
                log::info!("Add phase {}", msg.phase_uid);
                let row_builder = self.factories.work_on_procedure.phase_row.build();
                let components_box = row_builder.get_object("components_box");
                let row = row_builder.get();
                self.phases_list.add(&row);
                let mut phase = Phase {
                    uid: msg.phase_uid,
                    row,
                    components: Vec::with_capacity(msg.components.len()),
                    components_box,
                };
                self.set_components(&mut phase, msg.components);
                self.phases.insert(phase.uid, phase);
            },
            MessageToProcedureScreen::PhasePopped(msg) => {
                if let Some(phase) = self.phases.remove(&msg.phase_uid) {
                    self.phases_list.remove(&phase.row);
                } else {
                    log::warn!("Requested to remove phase {}, but it does not exist", msg.phase_uid);
                    return;
                }
            },
            MessageToProcedureScreen::ProcedureFinished(_msg) => {
                TODO
            },
        }
    }

    fn set_components(&self, phase: &mut Phase, components: Vec<phase_control_messages::Component>) {
        phase.components.reserve_exact(components.len());
        for component in components.into_iter() {
            let gui_component_builder = match &component.params {
                ComponentParams::Caption { text } => {
                    let builder = self.factories.work_on_procedure.components.caption.build();
                    builder.get_object::<gtk::Label>("caption_label").set_text(text);
                    builder
                },
                ComponentParams::Button { text } => {
                    let builder = self.factories.work_on_procedure.components.button.build();
                    let button: gtk::Button = builder.get_object("button");
                    button.set_label(text);
                    let name = component.name.clone();
                    let server_idx = self.server_idx;
                    let phase_uid = phase.uid;
                    let gui_actor = self.gui_actor.clone();
                    button.connect_clicked(move |_| {
                        gui_actor.do_send(UserClickedButton {
                            server_idx,
                            phase_uid,
                            button_name: name.clone(),
                        });
                    });
                    builder
                },
            };
            let new_component = Component {
                name: component.name,
                params: component.params,
                gui_component: gui_component_builder.get(),
            };
            phase.components_box.add(&new_component.gui_component);
            phase.components.push(new_component);
        }
    }
}

struct Phase {
    #[allow(unused)]
    uid: Uuid,
    #[allow(unused)]
    row: gtk::ListBoxRow,
    components: Vec<Component>,
    components_box: gtk::FlowBox,
}

struct Component {
    #[allow(unused)]
    name: Option<String>,
    #[allow(unused)]
    params: ComponentParams,
    gui_component: gtk::FlowBoxChild,
}
