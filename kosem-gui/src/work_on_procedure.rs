use actix::prelude::*;
use gtk::prelude::*;

use kosem_webapi::Uuid;
use kosem_webapi::pairing_messages;
use kosem_webapi::phase_control_messages;

use crate::internal_messages::gui_control::{
    ProcedureScreenAttach,
    UserClickedButton,
    ShowJoinMenu,
};

#[derive(woab::Factories)]
pub struct WorkOnProcedureFactories {
    pub app_work_on_procedure_window: woab::Factory<WorkOnProcedureActor, WorkOnProcedureWidgets, WorkOnProcedureSignal>,
    row_phase: woab::Factory<(), PhaseWidgets, ()>,
    cld_caption: woab::Factory<(), ComponentCaptionWidgets, ()>,
    cld_button: woab::Factory<(), ComponentButtonWidgets, ComponentSignal>,
}

#[derive(typed_builder::TypedBuilder)]
pub struct WorkOnProcedureActor {
    factories: crate::Factories,
    widgets: WorkOnProcedureWidgets,
    join_menu: Addr<crate::join_menu::JoinMenuActor>,
    gui_client: Addr<crate::client::GuiClientActor>,
    server_idx: usize,
    procedure: pairing_messages::AvailableProcedure,
    #[builder(default, setter(skip))]
    phases: std::collections::HashMap<Uuid, PhaseRow>,
}

impl Actor for WorkOnProcedureActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.widgets.lbl_title.set_text(&self.procedure.name);
        self.widgets.app_work_on_procedure_window.show_all();
        self.gui_client.do_send(ProcedureScreenAttach {
            server_idx: self.server_idx,
            request_uid: self.procedure.uid,
            addr: ctx.address(),
        });
    }
}

#[derive(woab::WidgetsFromBuilder)]
pub struct WorkOnProcedureWidgets {
    app_work_on_procedure_window: gtk::ApplicationWindow,
    lst_phases: gtk::ListBox,
    lbl_title: gtk::Label,
}

#[derive(woab::BuilderSignal)]
pub enum WorkOnProcedureSignal {
    WindowDestroyed,
}

impl StreamHandler<WorkOnProcedureSignal> for WorkOnProcedureActor {
    fn handle(&mut self, signal: WorkOnProcedureSignal, _ctx: &mut Self::Context) {
        match signal {
            WorkOnProcedureSignal::WindowDestroyed => {
                self.join_menu.do_send(ShowJoinMenu);
            }
        }
    }
}

impl Handler<phase_control_messages::PhasePushed> for WorkOnProcedureActor {
    type Result = ();

    fn handle(&mut self, msg: phase_control_messages::PhasePushed, ctx: &mut Self::Context) -> Self::Result {
        let phase_widgets = self.factories.work_on_procedure.row_phase.build().widgets().unwrap();
        self.widgets.lst_phases.add(&phase_widgets.row_phase);
        for (i, component) in msg.components.iter().enumerate() {
            match &component.params {
                phase_control_messages::ComponentParams::Caption { text } => {
                    let widgets = self.factories.work_on_procedure.cld_caption.build().widgets().unwrap();
                    widgets.lbl_caption.set_text(&text);
                    phase_widgets.box_components.add(&widgets.cld_caption);
                }
                phase_control_messages::ComponentParams::Button { text } => {
                    let widgets = self.factories.work_on_procedure.cld_button.build()
                        .connect_tagged_builder_signals(ctx, (msg.phase_uid, i))
                        .widgets().unwrap();
                    widgets.btn_button.set_label(&text);
                    phase_widgets.box_components.add(&widgets.cld_button);
                }
            }
        }
        self.phases.insert(msg.phase_uid, PhaseRow {
            widgets: phase_widgets,
            msg,
        });
    }
}

impl Handler<phase_control_messages::PhasePopped> for WorkOnProcedureActor {
    type Result = ();

    fn handle(&mut self, msg: phase_control_messages::PhasePopped, _ctx: &mut Self::Context) -> Self::Result {
        let phase_row = if let Some(p) = self.phases.get(&msg.phase_uid) {
            p
        } else {
            log::warn!("Unknown phase {}", msg.phase_uid);
            return;
        };
        self.widgets.lst_phases.remove(&phase_row.widgets.row_phase);
    }
}

impl Handler<pairing_messages::ProcedureFinished> for WorkOnProcedureActor {
    type Result = ();

    fn handle(&mut self, _msg: pairing_messages::ProcedureFinished, _ctx: &mut Self::Context) -> Self::Result {
        self.widgets.app_work_on_procedure_window.close();
    }
}

struct PhaseRow {
    widgets: PhaseWidgets,
    msg: phase_control_messages::PhasePushed,
}

#[derive(woab::WidgetsFromBuilder)]
struct PhaseWidgets {
    row_phase: gtk::ListBoxRow,
    box_components: gtk::FlowBox,
}

#[derive(woab::WidgetsFromBuilder)]
struct ComponentCaptionWidgets {
    cld_caption: gtk::FlowBoxChild,
    lbl_caption: gtk::Label,
}

#[derive(woab::WidgetsFromBuilder)]
struct ComponentButtonWidgets {
    cld_button: gtk::FlowBoxChild,
    btn_button: gtk::Button,
}

#[derive(woab::BuilderSignal)]
enum ComponentSignal {
    ButtonClicked,
}

impl StreamHandler<((Uuid, usize), ComponentSignal)> for WorkOnProcedureActor {
    fn handle(&mut self, ((phase_uid, component_ordinal), signal): ((Uuid, usize), ComponentSignal), _ctx: &mut Self::Context) {
        let phase_row = if let Some(p) = self.phases.get(&phase_uid) {
            p
        } else {
            log::warn!("Unknown phase {}", phase_uid);
            return;
        };
        let component = if let Some(c) = phase_row.msg.components.get(component_ordinal) {
            c
        } else {
            log::warn!("Phase {} only has {} components - cannot access component {}", phase_uid, phase_row.msg.components.len(), component_ordinal);
            return;
        };
        match signal {
            ComponentSignal::ButtonClicked => {
                self.gui_client.do_send(UserClickedButton {
                    server_idx: self.server_idx,
                    request_uid: self.procedure.uid,
                    phase_uid,
                    button_name: component.name.clone(),
                });
            }
        }
    }

    fn finished(&mut self, _ctx: &mut Self::Context) {}
}
