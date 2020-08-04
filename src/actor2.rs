use actix::prelude::*;
use std::println;

use crate::messages::MsgToActor2;

pub struct Actor2;

impl Default for Actor2 {
    fn default() -> Self {
        Self
    }
}

impl Actor for Actor2 {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Context<Self>) {
        println!("Actor2: started");
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
        println!("Actor2: stopped");
    }
}

impl Supervised for Actor2 {
    fn restarting(&mut self, _ctx: &mut Context<Self>) {
        println!("Actor2: restarting");
    }
}

impl ArbiterService for Actor2 {
    fn service_started(&mut self, _ctx: &mut Context<Self>) {
        println!("Actor2: service started");
    }
}

impl Handler<MsgToActor2> for Actor2 {
    type Result = u8;

    fn handle(&mut self, _msg: MsgToActor2, _ctx: &mut Context<Self>) -> Self::Result {
        println!("Actor2: MsgToActor2 received");
        2
    }
}
