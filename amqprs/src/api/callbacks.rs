use std::str::from_utf8;

use crate::{frame::{Ack, Blocked, Close, CloseChannel, Flow, Nack, Unblocked, Return}, BasicProperties};

use super::{channel::Channel, connection::Connection};
use async_trait::async_trait;
use tracing::{error, info};

/////////////////////////////////////////////////////////////////////////////
#[async_trait]
pub trait ConnectionCallback {
    async fn close(&mut self, connection: &Connection, close: Close);
    async fn blocked(&mut self, connection: &Connection, blocked: Blocked);
    async fn unblocked(&mut self, connection: &Connection, blocked: Unblocked);
}
pub struct DefaultConnectionCallback;

#[async_trait]
impl ConnectionCallback for DefaultConnectionCallback {
    async fn close(&mut self, _connection: &Connection, close: Close) {
        error!("{}", close);
    }

    async fn blocked(&mut self, _connection: &Connection, blocked: Blocked) {
        info!("connection blocked by server, reason: {}", blocked.reason());
    }
    async fn unblocked(&mut self, _connection: &Connection, _blocked: Unblocked) {
        info!("connection unblocked by server");
    }
}

/////////////////////////////////////////////////////////////////////////////
#[async_trait]
pub trait ChannelCallback {
    async fn close(&mut self, channel: &Channel, close: CloseChannel);
    async fn flow(&mut self, channel: &Channel, flow: Flow);
    async fn publish_ack(&mut self, channel: &Channel, ack: Ack);
    async fn publish_nack(&mut self, channel: &Channel, nack: Nack);
    async fn publish_return(
        &mut self,
        channel: &Channel,
        ret: Return,
        basic_properties: BasicProperties,
        content: Vec<u8>,
    );
}

pub struct DefaultChannelCallback;

#[async_trait]
impl ChannelCallback for DefaultChannelCallback {
    async fn close(&mut self, _channel: &Channel, close: CloseChannel) {
        error!("{}", close);
    }
    async fn flow(&mut self, channel: &Channel, flow: Flow) {
        info!("channel flow request from server, {}", flow.active());
    }
    async fn publish_ack(&mut self, channel: &Channel, ack: Ack) {
        info!("channel publish ack from server, {}", ack.delivery_tag());

    }
    async fn publish_nack(&mut self, channel: &Channel, nack: Nack) {
        info!("channel publish nack from server, {}", nack.delivery_tag());

    }
    async fn publish_return(
        &mut self,
        channel: &Channel,
        ret: Return,
        basic_properties: BasicProperties,
        content: Vec<u8>,
    ) {
        info!(">>>>> Publish Return Start <<<<<");
        info!("{}", ret);
        info!("{}", basic_properties,);
        info!("{}", from_utf8(&content).unwrap());
        info!(">>>>> Publish Return End <<<<<");

    }
}
