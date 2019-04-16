//! #Implementing new commands
//! To implement a new command, add an instance of `ChatCommand` to `CHAT_COMMANDS`
//! and provide a handler function

use crate::Server;
use common::{comp, msg::ServerMsg};
use specs::{Entity as EcsEntity, join::Join};
use vek::*;

use scan_fmt::scan_fmt;
use lazy_static::lazy_static;

/// Struct representing a command that a user can run from server chat
pub struct ChatCommand {
    /// The keyword used to invoke the function, omitting the leading '/'
    pub keyword: &'static str,
    /// the format string used by `scan_fmt` to parse arguments
    arg_fmt: &'static str,
    /// message to explain how the command is used
    help_string: &'static str,
    /// handler function called when the command is executed
    handler: fn(&mut Server, EcsEntity, String, &ChatCommand),
}

impl ChatCommand {
    /// Creates a new chat command
    pub fn new(
        keyword: &'static str,
        arg_fmt: &'static str,
        help_string: &'static str,
        handler: fn(&mut Server, EcsEntity, String, &ChatCommand),
    ) -> Self {
        Self {
            keyword,
            arg_fmt,
            help_string,
            handler,
        }
    }
    /// Calls the contained handler function, passing `&self` as the last argument
    pub fn execute(&self, server: &mut Server, entity: EcsEntity, args: String) {
        (self.handler)(server, entity, args, self);
    }
}

lazy_static! {
    /// Static list of chat commands available to the server
    pub static ref CHAT_COMMANDS: Vec<ChatCommand> = vec![
        ChatCommand::new(
            "jump",
            "{d} {d} {d}",
            "jump: offset your current position by a vector\n
                Usage: /jump [x] [y] [z]",
            handle_jump
        ),
        ChatCommand::new(
            "goto",
            "{d} {d} {d}",
            "goto: teleport to a given position\n
                Usage: /goto [x] [y] [z]",
            handle_goto
        ),
        ChatCommand::new(
            "alias",
            "{}",
            "alias: change your player name (cannot contain spaces)\n
                Usage: /alias [name]",
            handle_alias
        ),
        ChatCommand::new(
            "tp",
            "{}",
            "tp: teleport to a named player\n
                Usage: /tp [name]",
            handle_tp
        ),
        ChatCommand::new("help", "", "help: display this message", handle_help)
    ];
}

fn handle_jump(server: &mut Server, entity: EcsEntity, args: String, action: &ChatCommand) {
    let (opt_x, opt_y, opt_z) = scan_fmt!(&args, action.arg_fmt, f32, f32, f32);
    match (opt_x, opt_y, opt_z) {
        (Some(x), Some(y), Some(z)) => {
            if let Some(current_pos) = server
                .state
                .read_component_cloned::<comp::phys::Pos>(entity)
            {
                server
                    .state
                    .write_component(entity, comp::phys::Pos(current_pos.0 + Vec3::new(x, y, z)))
            } else {
                server.clients.notify(
                    entity,
                    ServerMsg::Chat(String::from("Command 'jump' invalid in current state")),
                )
            }
        }
        _ => server
            .clients
            .notify(entity, ServerMsg::Chat(String::from(action.help_string))),
    }
}

fn handle_goto(server: &mut Server, entity: EcsEntity, args: String, action: &ChatCommand) {
    let (opt_x, opt_y, opt_z) = scan_fmt!(&args, action.arg_fmt, f32, f32, f32);
    match (opt_x, opt_y, opt_z) {
        (Some(x), Some(y), Some(z)) => server
            .state
            .write_component(entity, comp::phys::Pos(Vec3::new(x, y, z))),
        _ => server
            .clients
            .notify(entity, ServerMsg::Chat(String::from(action.help_string))),
    }
}

fn handle_alias(server: &mut Server, entity: EcsEntity, args: String, action: &ChatCommand) {
    let opt_alias = scan_fmt!(&args, action.arg_fmt, String);
    match opt_alias {
        Some(alias) => server
            .state
            .write_component(entity, comp::player::Player { alias }),
        None => server
            .clients
            .notify(entity, ServerMsg::Chat(String::from(action.help_string))),
    }
}

fn handle_tp(server: &mut Server, entity: EcsEntity, args: String, action: &ChatCommand) {
    let opt_alias = scan_fmt!(&args, action.arg_fmt, String);
    match opt_alias {
        Some(alias) => {
            let ecs = server.state.ecs().internal();
            let opt_player = (&ecs.entities(), &ecs.read_storage::<comp::player::Player>())
                .join()
                .find(|(_, player)| player.alias == alias)
                .map(|(entity, _)| entity);
            match opt_player {
                Some(player) => match server
                    .state
                    .read_component_cloned::<comp::phys::Pos>(player)
                {
                    Some(pos) => server.state.write_component(entity, pos),
                    None => server.clients.notify(
                        entity,
                        ServerMsg::Chat(format!("Unable to teleport to player '{}'", alias)),
                    ),
                },

                None => {
                    server.clients.notify(
                        entity,
                        ServerMsg::Chat(format!("Player '{}' not found!", alias)),
                    );
                    server
                        .clients
                        .notify(entity, ServerMsg::Chat(String::from(action.help_string)));
                }
            }
        }
        None => server
            .clients
            .notify(entity, ServerMsg::Chat(String::from(action.help_string))),
    }
}

fn handle_help(server: &mut Server, entity: EcsEntity, _args: String, _action: &ChatCommand) {
    for cmd in CHAT_COMMANDS.iter() {
        server
            .clients
            .notify(entity, ServerMsg::Chat(String::from(cmd.help_string)));
    }
}