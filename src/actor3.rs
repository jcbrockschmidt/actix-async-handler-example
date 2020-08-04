use actix::prelude::*;
use std::println;

use crate::messages::MsgToActor3;

pub struct Actor3;

impl Default for Actor3 {
    fn default() -> Self {
        Self
    }
}

impl Actor for Actor3 {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Context<Self>) {
        println!("Actor3: started");
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
        println!("Actor3: stopped");
    }
}

impl Supervised for Actor3 {
    fn restarting(&mut self, _ctx: &mut Context<Self>) {
        println!("Actor3: restarting");
    }
}

impl ArbiterService for Actor3 {
    fn service_started(&mut self, _ctx: &mut Context<Self>) {
        println!("Actor3: service started");
    }
}

impl Handler<MsgToActor3> for Actor3 {
    type Result = u8;

    fn handle(&mut self, msg: MsgToActor3, _ctx: &mut Context<Self>) -> Self::Result {
        println!("Actor3: MsgToActor3 received");
        msg.n * 3
    }
}
