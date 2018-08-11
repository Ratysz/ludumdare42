use ggez::event::{KeyCode, KeyMods, MouseButton};
use ggez::Context;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Input {
    Key(KeyCode),
    Mouse(MouseButton),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum InputExtra {
    None,
    RepeatedKey(bool),
    XY(u32, u32),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Command {
    Quit,
}

pub struct InputHandler {
    bindings: HashMap<Input, Vec<(KeyMods, Command)>>,
}

impl Default for InputHandler {
    fn default() -> InputHandler {
        InputHandler::new()
    }
}

impl InputHandler {
    pub fn new() -> InputHandler {
        InputHandler {
            bindings: HashMap::new(),
        }
    }

    pub fn bind(&mut self, input: Input, keymods: KeyMods, action: Command) -> &mut InputHandler {
        {
            let mut done = false;
            for mut bound_action in self.bindings.entry(input).or_insert_with(Vec::new) {
                if bound_action.0 == keymods {
                    bound_action.1 = action;
                    done = true;
                    break;
                }
            }
            if !done {
                let bound_action_bunch = self.bindings.entry(input).or_insert_with(Vec::new);
                let count = keymods.bits().count_ones();
                let mut index = 0;
                for bound_action in bound_action_bunch.iter() {
                    if count >= bound_action.0.bits().count_ones() {
                        break;
                    }
                    index += 1;
                }
                bound_action_bunch.insert(index, (keymods, action));
            }
        }
        self
    }

    fn resolve(
        &self,
        ctx: &mut Context,
        input: Input,
        keymods: KeyMods,
        extra: InputExtra,
    ) -> Option<(Command, InputExtra)> {
        if let Some(bound_action_bunch) = self.bindings.get(&input) {
            for bound_action in bound_action_bunch {
                if keymods.contains(bound_action.0) {
                    match bound_action.1 {
                        Command::Quit => {
                            trace!("Quitting.");
                            ctx.quit();
                        }
                        other => return Some((other, extra)),
                    }
                }
            }
        }
        None
    }

    pub fn key_down_event(
        &mut self,
        ctx: &mut Context,
        key: KeyCode,
        mods: KeyMods,
        repeat: bool,
    ) -> Option<(Command, InputExtra)> {
        self.resolve(ctx, Input::Key(key), mods, InputExtra::RepeatedKey(repeat))
    }
}
