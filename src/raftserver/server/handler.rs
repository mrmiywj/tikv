#![allow(dead_code)]
#![allow(unused_variables)]

use std::vec::Vec;

use mio::Token;

use raftserver::Result;
use super::{ConnData, Sender, TimerMsg};

// ServerHandler is for server logic, we must implement it for our raft server.
// We use a event loop to handle all events, when an event is triggered,
// event loop will call its associated event handler, after we do something
// in this event handler, we can call server handler for outer logic use.
// The event flow is event loop -> event handler (Server) -> server handler.
// E.g, event loop finds a connection is ready to read, so it calls event handler
// to read data, after the event handler reads whole data, it calls server handler
// for outer use.
pub trait ServerHandler :Sized {
    // Handles messages reading from connection with Token.
    // Returns some messages for later writing to this connection.
    // You can use sender to communicate with event loop.
    fn handle_read_data(&mut self,
                        sender: &Sender,
                        token: Token,
                        msgs: Vec<ConnData>)
                        -> Result<(Vec<ConnData>)> {
        Ok((msgs))
    }

    // Handles tick.
    fn handle_tick(&mut self, sender: &Sender) -> Result<()> {
        Ok(())
    }

    // Handles customized timer.
    fn handle_timer(&mut self, sender: &Sender, msg: TimerMsg) -> Result<()> {
        Ok(())
    }

    // Handle quit, this will be called when we quit the server.
    // We should do our cleanup in this function here.
    fn handle_quit(&mut self) {}
}