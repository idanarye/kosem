use serde::Deserialize;
use actix::prelude::*;
use actix_web_actors::ws;

use kosem_webapi::protocols::{JrpcMessage, JrpcResponse, JrpcError};

use crate::role_actors;
use crate::internal_messages::connection::{RpcMessage, SetRole, AddHumanActor};

pub struct WsJrpc {
    pub state: role_actors::ActorRoleState,
}

impl actix::Actor for WsJrpc {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.state = role_actors::ActorRoleState::start_not_yet_identified_actor(ctx.address());
    }
}

fn format_deserialization_error(method: Option<String>, error: serde_json::Error) -> JrpcError  {
    use serde_json::error::Category;
    match error.classify() {
         Category::Data => if let Some(method) = method {
             JrpcError {
                 code: -32602,
                 message: "Invalid params".to_owned(),
                 data: Some(serde_json::json!({
                     "method_name": method,
                     "error": error.to_string(),
                     "line": error.line(),
                     "column": error.column(),
                 })),
             }
         } else {
             JrpcError {
                 code: -32600,
                 message: "Invalid Request".to_owned(),
                 data: Some(serde_json::json!({
                     "error": error.to_string(),
                     "line": error.line(),
                     "column": error.column(),
                 })),
             }
         }
         Category::Syntax | Category::Io | Category::Eof => JrpcError {
             code: -32700,
             message: "Parse error".to_owned(),
             data: Some(serde_json::json!({
                 "error": error.to_string(),
                 "line": error.line(),
                 "column": error.column(),
             })),
         }
    }
}

impl actix::StreamHandler<ws::Message, ws::ProtocolError> for WsJrpc {
    fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
        match msg {
            ws::Message::Ping(msg) => ctx.pong(&msg),
            ws::Message::Text(txt) => {
                let request: JrpcMessage = match serde_json::from_str(&txt) {
                    Ok(request) => request,
                    Err(error) => {
                        let response = JrpcResponse {
                            jsonrpc: "2.0".into(),
                            id: None,
                            payload: Err(format_deserialization_error(None, error)),
                        };
                        ctx.text(serde_json::to_string(&response).unwrap());
                        return;
                    }
                };
                log::warn!("got {:?}", request);
                let request_id = request.id.clone();
                let future = self.state.send_request_from_connection(&request.method, request.params, |_method, _error| {
                });
                let future = match future {
                    Ok(future) => future,
                    Err(error) => {
                        use crate::role_actors::RoutingError;
                        let jrpc_error = match error {
                            RoutingError::MethodNotFound(method) =>  JrpcError {
                                code: -32601,
                                message: "Method not found".to_owned(),
                                data: Some(serde_json::json!({
                                    "method_name": method,
                                })),
                            },
                            RoutingError::MethodNotAllowedForRole { method, current_role, allowed_roles } => JrpcError {
                                code: -32601,
                                message: "Method not allowed for current role".to_owned(),
                                data: Some(serde_json::json!({
                                    "method_name": method,
                                    "current_role": current_role,
                                    "allowed_roles": allowed_roles,
                                })),
                            },
                            RoutingError::DeserializationError { method, error } => format_deserialization_error(method, error),
                        };
                        let response = JrpcResponse {
                            jsonrpc: "2.0".into(),
                            id: request_id,
                            payload: Err(jrpc_error),
                        };
                        ctx.text(serde_json::to_string(&response).unwrap());
                        return;
                    }
                };
                let future = future.into_actor(self);
                let future = future.map_err({
                    let request_id = request_id.clone();
                    move |error, _, ctx| {
                        let jrpc_error = JrpcError {
                            code: -32000,
                            message: "Server error".to_owned(),
                            data: Some(serde_json::json!({
                                "error": error.to_string(),
                            })),
                        };
                        let response = JrpcResponse {
                            jsonrpc: "2.0".into(),
                            id: request_id,
                            payload: Err(jrpc_error),
                        };
                        ctx.text(serde_json::to_string(&response).unwrap());
                    }
                });
                let future = future.map(move |result, _, ctx| {
                    match result {
                        Ok(result) => {
                            if request_id.is_some() {
                                let response = JrpcResponse {
                                    jsonrpc: "2.0".into(),
                                    id: request_id,
                                    payload: Ok(Deserialize::deserialize(result).unwrap()),
                                };
                                ctx.text(serde_json::to_string(&response).unwrap());
                            }
                        },
                        Err(error) => {
                            let response = JrpcResponse {
                                jsonrpc: "2.0".into(),
                                id: request_id,
                                payload: Err(JrpcError {
                                    code: 1,
                                    message: error.message,
                                    data: Some(serde_json::value::to_value(error.data_fields).unwrap()),
                                }),
                            };
                            ctx.text(serde_json::to_string(&response).unwrap());
                        }
                    }
                });
                ctx.spawn(future);
            },
            ws::Message::Close(_) => {
                ctx.close(Some(ws::CloseReason {
                    code: ws::CloseCode::Normal,
                    description: None,
                }));
            },
            _ => (),
        }
    }

    fn finished(&mut self, ctx: &mut Self::Context) {
        self.state.notify_connection_is_closed();
        ctx.stop()
    }
}

impl actix::Handler<RpcMessage> for WsJrpc {
    type Result = <RpcMessage as actix::Message>::Result;

    fn handle(&mut self, msg: RpcMessage, ctx: &mut Self::Context) -> Self::Result {
        let response = JrpcMessage {
            jsonrpc: "2.0".into(),
            method: msg.method,
            id: None,
            params: Deserialize::deserialize(msg.params).unwrap(),
        };
        ctx.text(serde_json::to_string(&response).unwrap());
    }
}

impl actix::Handler<SetRole> for WsJrpc {
    type Result = <SetRole as actix::Message>::Result;

    fn handle(&mut self, msg: SetRole, _ctx: &mut Self::Context) -> Self::Result {
        match msg {
            SetRole::Procedure(addr) => {
                self.state = role_actors::ActorRoleState::ProcedureActor(addr);
            }
            SetRole::Human(addr) => {
                self.state = role_actors::ActorRoleState::HumanActor {
                    joiner: addr,
                    procedures: Default::default(),
                };
            }
        }
    }
}

impl actix::Handler<AddHumanActor> for WsJrpc {
    type Result = <AddHumanActor as actix::Message>::Result;

    fn handle(&mut self, msg: AddHumanActor, _ctx: &mut Self::Context) -> Self::Result {
        if let role_actors::ActorRoleState::HumanActor { ref mut procedures, .. } = self.state {
            procedures.insert(msg.request_uid, msg.addr);
        } else {
            panic!("Expected human role");
        }
    }
}
