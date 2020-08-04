use actix::prelude::*;
use std::boxed::Box;
use std::println;

use crate::messages::*;
use crate::actor2::Actor2;
use crate::actor3::Actor3;

pub struct Actor1;

impl Default for Actor1 {
    fn default() -> Self {
        Self
    }
}

impl Actor for Actor1 {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Context<Self>) {
        println!("Actor1: started");
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
        println!("Actor1: stopped");
    }
}

impl Supervised for Actor1 {
    fn restarting(&mut self, _ctx: &mut Context<Self>) {
        println!("Actor1: restarting");
    }
}

impl ArbiterService for Actor1 {
    fn service_started(&mut self, _ctx: &mut Context<Self>) {
        println!("Actor1: service started");
    }
}

impl Handler<MsgToActor1> for Actor1 {
    type Result = ResponseFuture<futures::channel::oneshot::Receiver<u8>>;

    fn handle(&mut self, _msg: MsgToActor1, ctx: &mut Context<Self>) -> Self::Result {
        println!("Actor1: MsgToActor1 received");
        let (tx, rx) = futures::channel::oneshot::channel::<u8>();

        let fut = async move {
            let val1 = Actor2::from_registry()
                .recipient()
                .send(MsgToActor2)
                .await
                .unwrap();
            println!("Actor2 response: {}", val1);

            let val2 = Actor3::from_registry()
                .recipient()
                .send(MsgToActor3 {
                    n: val1,
                })
                .await
                .unwrap();
            println!("Actor3 response: {}", val2);

            tx.send(val2).unwrap();
        };

        fut.into_actor(self).spawn(ctx);
        Box::pin(async move { rx })
    }
}
