use std::collections::HashMap;

use actix::prelude::*;
use gtk4::prelude::*;

use kosem_webapi::pairing_messages;
use kosem_webapi::phase_control_messages;
use kosem_webapi::Uuid;

use crate::internal_messages::gui_control::{
    ProcedureScreenAttach, ShowJoinMenu, UserClickedButton,
};

#[derive(woab::Factories)]
pub struct WorkOnProcedureFactories {
    pub app_work_on_procedure_window: woab::BuilderFactory,
    row_phase: woab::BuilderFactory,
    cld_caption: woab::BuilderFactory,
    cld_button: woab::BuilderFactory,
    cld_textbox: woab::BuilderFactory,
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
        self.widgets.app_work_on_procedure_window.set_visible(true);
        self.gui_client.do_send(ProcedureScreenAttach {
            server_idx: self.server_idx,
            request_uid: self.procedure.uid,
            addr: ctx.address(),
        });
    }
}

#[derive(woab::WidgetsFromBuilder)]
pub struct WorkOnProcedureWidgets {
    app_work_on_procedure_window: gtk4::ApplicationWindow,
    lst_phases: gtk4::ListBox,
    lbl_title: gtk4::Label,
}

impl actix::Handler<woab::Signal> for WorkOnProcedureActor {
    type Result = woab::SignalResult;

    fn handle(&mut self, msg: woab::Signal, _ctx: &mut Self::Context) -> Self::Result {
        Ok(match msg.name() {
            _ => msg.cant_handle()?,
        })
    }
}

impl Handler<phase_control_messages::PhasePushed> for WorkOnProcedureActor {
    type Result = ();

    fn handle(
        &mut self,
        msg: phase_control_messages::PhasePushed,
        ctx: &mut Self::Context,
    ) -> Self::Result {
        let phase_widgets: PhaseWidgets = self
            .factories
            .work_on_procedure
            .row_phase
            .instantiate_without_routing_signals()
            .widgets()
            .unwrap();
        self.widgets.lst_phases.append(&phase_widgets.row_phase);
        let mut readable_components = HashMap::new();
        for (i, component) in msg.components.iter().enumerate() {
            match &component.params {
                phase_control_messages::ComponentParams::Caption { text } => {
                    let widgets: ComponentCaptionWidgets = self
                        .factories
                        .work_on_procedure
                        .cld_caption
                        .instantiate_without_routing_signals()
                        .widgets()
                        .unwrap();
                    widgets.lbl_caption.set_text(text);
                    phase_widgets.box_components.append(&widgets.cld_caption);
                }
                phase_control_messages::ComponentParams::Button { text } => {
                    let widgets: ComponentButtonWidgets = self
                        .factories
                        .work_on_procedure
                        .cld_button
                        .instantiate_route_to(((msg.phase_uid, i), ctx.address()))
                        .widgets()
                        .unwrap();
                    widgets.btn_button.set_label(text);
                    phase_widgets.box_components.append(&widgets.cld_button);
                }
                phase_control_messages::ComponentParams::Textbox { text } => {
                    let widgets: ComponentTextboxWidgets = self
                        .factories
                        .work_on_procedure
                        .cld_textbox
                        .instantiate_route_to(((msg.phase_uid, i), ctx.address()))
                        .widgets()
                        .unwrap();
                    widgets.txt_textbox.set_text(text);
                    if let Some(name) = component.name.as_ref() {
                        widgets.txt_textbox.set_editable(true);
                        readable_components.insert(name.to_owned(), ReadableComponent::Textbox(widgets.txt_textbox.clone()));
                    } else {
                        widgets.txt_textbox.set_editable(false);
                    }
                    phase_widgets.box_components.append(&widgets.cld_textbox);
                }
            }
        }
        self.phases.insert(
            msg.phase_uid,
            PhaseRow {
                widgets: phase_widgets,
                msg,
                readable_components,
            },
        );
    }
}

impl Handler<phase_control_messages::PhasePopped> for WorkOnProcedureActor {
    type Result = ();

    fn handle(
        &mut self,
        msg: phase_control_messages::PhasePopped,
        _ctx: &mut Self::Context,
    ) -> Self::Result {
        let phase_row = if let Some(p) = self.phases.get(&msg.phase_uid) {
            p
        } else {
            log::warn!("Unknown phase {}", msg.phase_uid);
            return;
        };
        self.widgets.lst_phases.remove(&phase_row.widgets.row_phase);
    }
}

impl Handler<phase_control_messages::PhaseDataReadRequest> for WorkOnProcedureActor {
    type Result = <phase_control_messages::PhaseDataReadRequest as Message>::Result;

    fn handle(&mut self, msg: phase_control_messages::PhaseDataReadRequest, _ctx: &mut Self::Context) -> Self::Result {
        log::info!("GUI will read from phase {:?}", msg.phase_uid);
        let Some(phase) = self.phases.get(&msg.phase_uid) else {
            panic!();
            // return Err(KosemError::new("Phase does not exist").with("phase_uid", msg.phase_uid));
        };
        for (name, readable_component) in phase.readable_components.iter() {
            log::info!("  {:?} => {:?}", name, readable_component.read());
        }
    }
}

impl Handler<pairing_messages::ProcedureFinished> for WorkOnProcedureActor {
    type Result = ();

    fn handle(
        &mut self,
        _msg: pairing_messages::ProcedureFinished,
        _ctx: &mut Self::Context,
    ) -> Self::Result {
        self.widgets.app_work_on_procedure_window.close();
        self.join_menu.do_send(ShowJoinMenu);
    }
}

struct PhaseRow {
    widgets: PhaseWidgets,
    msg: phase_control_messages::PhasePushed,
    readable_components: HashMap<String, ReadableComponent>,
}

enum ReadableComponent {
    Textbox(gtk4::Entry),
}

impl ReadableComponent {
    fn read(&self) -> String {
        match self {
            ReadableComponent::Textbox(textbox) => textbox.text().to_string(),
        }
    }
}

#[derive(woab::WidgetsFromBuilder)]
struct PhaseWidgets {
    row_phase: gtk4::ListBoxRow,
    box_components: gtk4::FlowBox,
}

#[derive(woab::WidgetsFromBuilder)]
struct ComponentCaptionWidgets {
    cld_caption: gtk4::FlowBoxChild,
    lbl_caption: gtk4::Label,
}

#[derive(woab::WidgetsFromBuilder)]
struct ComponentButtonWidgets {
    cld_button: gtk4::FlowBoxChild,
    btn_button: gtk4::Button,
}

#[derive(woab::WidgetsFromBuilder)]
struct ComponentTextboxWidgets {
    cld_textbox: gtk4::FlowBoxChild,
    txt_textbox: gtk4::Entry,
}

impl actix::Handler<woab::Signal<(Uuid, usize)>> for WorkOnProcedureActor {
    type Result = woab::SignalResult;

    fn handle(
        &mut self,
        msg: woab::Signal<(Uuid, usize)>,
        _ctx: &mut Self::Context,
    ) -> Self::Result {
        let (phase_uid, component_ordinal) = *msg.tag();
        let phase_row = if let Some(p) = self.phases.get(&phase_uid) {
            p
        } else {
            log::warn!("Unknown phase {}", phase_uid);
            return Ok(None);
        };
        let component = if let Some(c) = phase_row.msg.components.get(component_ordinal) {
            c
        } else {
            log::warn!(
                "Phase {} only has {} components - cannot access component {}",
                phase_uid,
                phase_row.msg.components.len(),
                component_ordinal
            );
            return Ok(None);
        };
        Ok(match msg.name() {
            "button_clicked" => {
                self.gui_client.do_send(UserClickedButton {
                    server_idx: self.server_idx,
                    request_uid: self.procedure.uid,
                    phase_uid,
                    button_name: component.name.clone(),
                });
                None
            }
            _ => msg.cant_handle()?,
        })
    }
}
