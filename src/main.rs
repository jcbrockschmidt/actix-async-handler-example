use actix::prelude::*;
use actix_rt;
use actix_test::actor1::Actor1;
use actix_test::messages::MsgToActor1;
use std::println;

#[actix_rt::main]
async fn main() {
    // It is not necessary to await the result or the receiver.
    let rx = Actor1::from_registry().send(MsgToActor1).await.unwrap();
    println!("Actor1 response: {}", rx.await.unwrap());
}
