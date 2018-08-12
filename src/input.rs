use ggez::event::{KeyCode, KeyMods, MouseButton};
use ggez::input::keyboard;
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
    XY(i32, i32),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Command {
    Click,
    ContextMenu,
    Quit,
}

pub struct InputHandler {
    bindings: HashMap<Input, Vec<(KeyMods, Command)>>,
}

impl Default for InputHandler {
    fn default() -> InputHandler {
        let mut handler = InputHandler::new();
        handler
            .bind(
                Input::Mouse(MouseButton::Left),
                KeyMods::NONE,
                Command::Click,
            )
            .bind(
                Input::Mouse(MouseButton::Right),
                KeyMods::NONE,
                Command::ContextMenu,
            )
            .bind(
                Input::Mouse(MouseButton::Left),
                KeyMods::ALT,
                Command::ContextMenu,
            );
        handler
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
                        other => {
                            trace!("Command: {:?} ({:?})", other, extra);
                            return Some((other, extra));
                        }
                    }
                }
            }
        }
        None
    }

    pub fn mouse_button_up_event(
        &mut self,
        ctx: &mut Context,
        button: MouseButton,
        x: f32,
        y: f32,
    ) -> Option<(Command, InputExtra)> {
        let mods = keyboard::get_active_mods(ctx);
        self.resolve(
            ctx,
            Input::Mouse(button),
            mods,
            InputExtra::XY(x as i32, y as i32),
        )
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
