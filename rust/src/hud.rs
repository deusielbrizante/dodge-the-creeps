use core::fmt;

use godot::{
    classes::{
        Button, CanvasLayer, ICanvasLayer, InputEventAction, Label, Shortcut, Timer,
        class_macros::private::virtuals::Os::array,
    },
    meta::ToGodot,
    obj::{Base, Gd, NewAlloc, NewGd, OnReady, WithBaseField, WithUserSignals},
    prelude::{GodotClass, godot_api},
};

enum HudChild {
    ScoreLabel,
    MessageLabel,
    MessageTimer,
    StartButton,
}

impl fmt::Display for HudChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HudChild::ScoreLabel => write!(f, "ScoreLabel"),
            HudChild::MessageLabel => write!(f, "MessageLabel"),
            HudChild::MessageTimer => write!(f, "MessageTimer"),
            HudChild::StartButton => write!(f, "StartButton"),
        }
    }
}

#[derive(GodotClass)]
#[class(base=CanvasLayer, tool)]
pub struct HUDBase {
    #[base]
    base: Base<CanvasLayer>,
    score_label: OnReady<Gd<Label>>,
    message_label: OnReady<Gd<Label>>,
    message_timer: OnReady<Gd<Timer>>,
    start_button: OnReady<Gd<Button>>,
}

#[godot_api]
impl ICanvasLayer for HUDBase {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            base,
            score_label: OnReady::from_node(&HudChild::ScoreLabel.to_string()),
            message_label: OnReady::from_node(&HudChild::MessageLabel.to_string()),
            message_timer: OnReady::from_node(&HudChild::MessageTimer.to_string()),
            start_button: OnReady::from_node(&HudChild::StartButton.to_string()),
        }
    }

    fn enter_tree(&mut self) {
        self.base()
            .try_get_node_as::<Label>(&HudChild::ScoreLabel.to_string())
            .unwrap_or_else(|| {
                let mut label = Label::new_alloc();
                label.set_name(&HudChild::ScoreLabel.to_string());
                self.base_mut().add_child(&label);
                label.set_owner(self.base().to_godot());
                label
            });

        self.base()
            .try_get_node_as::<Label>(&HudChild::MessageLabel.to_string())
            .unwrap_or_else(|| {
                let mut label = Label::new_alloc();
                label.set_name(&HudChild::MessageLabel.to_string());
                self.base_mut().add_child(&label);
                label.set_owner(self.base().to_godot());
                label
            });

        self.base()
            .try_get_node_as::<Timer>(&HudChild::MessageTimer.to_string())
            .unwrap_or_else(|| {
                let mut timer = Timer::new_alloc();
                timer.set_name(&HudChild::MessageTimer.to_string());
                self.base_mut().add_child(&timer);
                timer.set_owner(self.base().to_godot());
                timer
            });

        self.base()
            .try_get_node_as::<Button>(&HudChild::StartButton.to_string())
            .unwrap_or_else(|| {
                let mut button = Button::new_alloc();
                button.set_name(&HudChild::StartButton.to_string());
                self.base_mut().add_child(&button);
                button.set_owner(self.base().to_godot());
                button
            });
    }

    fn ready(&mut self) {
        self.start_button
            .signals()
            .pressed()
            .connect_other(self, Self::on_start_button_pressed);

        let input_start = {
            let input_action = {
                let mut input = InputEventAction::new_gd();
                input.set_action("ui_select");
                input
            };

            let mut shortcut = Shortcut::new_gd();
            shortcut.set_events(&array![&input_action.to_variant()]);
            shortcut
        };

        self.start_button.set_shortcut(input_start.to_godot());

        self.message_timer
            .signals()
            .timeout()
            .connect_other(self, Self::on_message_timer_timeout);
    }
}

#[godot_api]
impl HUDBase {
    #[signal]
    pub fn start_game();

    pub async fn game_over(mut hud: Gd<Self>) {
        hud.bind_mut().show_message("Fim de Jogo!");

        let timer = hud.bind().message_timer.signals().timeout().to_future();
        timer.await;

        hud.bind_mut()
            .message_label
            .set_text("Desvie e sobreviva aos monstros!");
        hud.bind_mut().message_label.show();
        hud.bind()
            .base()
            .get_tree()
            .expect("Erro ao buscar a cena!")
            .create_timer(1f64)
            .expect("Erro ao criar o timer de reinício!")
            .signals()
            .timeout()
            .to_future()
            .await;

        hud.bind_mut().start_button.show();
    }

    pub fn show_message(&mut self, text: &str) {
        self.message_label.set_text(text);
        self.message_label.show();
        self.message_timer.start();
    }

    pub fn update_score(&mut self, score: i64) {
        self.score_label.set_text(&score.to_string());
    }

    #[func]
    fn on_start_button_pressed(&mut self) {
        self.start_button.hide();
        self.signals().start_game().emit();
    }

    #[func]
    fn on_message_timer_timeout(&mut self) {
        self.message_label.hide();
    }
}
