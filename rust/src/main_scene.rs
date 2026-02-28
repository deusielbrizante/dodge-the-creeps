use core::fmt;
use std::f64::consts::PI;

use godot::{
    classes::{
        AudioStreamPlayer2D, ColorRect, Marker2D, Path2D, PathFollow2D, ResourceLoader, Timer,
    },
    global::{randf, randf_range, randi_range, randomize},
    prelude::*,
};

use crate::{enemy::EnemyBase, hud::HUDBase, player::PlayerBase};

enum MainSceneChild {
    HUD,
    SpawnEnemy,
    PathEnemy,
    StarterPosition,
    EnemyTimer,
    ScoreTimer,
    StarterTimer,
    Player,
    Background,
    Music,
    SoundDeath,
}

impl fmt::Display for MainSceneChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MainSceneChild::HUD => write!(f, "HUD"),
            MainSceneChild::SpawnEnemy => write!(f, "SpawnEnemy"),
            MainSceneChild::PathEnemy => write!(f, "PathEnemy"),
            MainSceneChild::StarterPosition => write!(f, "StarterPosition"),
            MainSceneChild::EnemyTimer => write!(f, "EnemyTimer"),
            MainSceneChild::ScoreTimer => write!(f, "ScoreTimer"),
            MainSceneChild::StarterTimer => write!(f, "StarterTimer"),
            MainSceneChild::Player => write!(f, "Player"),
            MainSceneChild::Background => write!(f, "BG"),
            MainSceneChild::Music => write!(f, "Music"),
            MainSceneChild::SoundDeath => write!(f, "SoundDeath"),
        }
    }
}

#[derive(GodotClass)]
#[class(base=Node, tool)]
pub struct MainScene {
    #[base]
    base: Base<Node>,
    #[export]
    enemy: Option<Gd<PackedScene>>,
    score: i64,
    path_enemy: OnReady<Gd<Path2D>>,
    path_follow: OnReady<Gd<PathFollow2D>>,
    starter_position: OnReady<Gd<Marker2D>>,
    enemy_timer: OnReady<Gd<Timer>>,
    score_timer: OnReady<Gd<Timer>>,
    starter_timer: OnReady<Gd<Timer>>,
    player: OnReady<Gd<PlayerBase>>,
    hud: OnReady<Gd<HUDBase>>,
    color_rect: OnReady<Gd<ColorRect>>,
    music: OnReady<Gd<AudioStreamPlayer2D>>,
    sound_death: OnReady<Gd<AudioStreamPlayer2D>>,
}

#[godot_api]
impl INode for MainScene {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            base,
            score: 0,
            enemy: None,
            path_enemy: OnReady::from_node(&MainSceneChild::PathEnemy.to_string()),
            path_follow: OnReady::from_node(&format!(
                "{}/{}",
                MainSceneChild::PathEnemy.to_string(),
                MainSceneChild::SpawnEnemy.to_string()
            )),
            starter_position: OnReady::from_node(&MainSceneChild::StarterPosition.to_string()),
            enemy_timer: OnReady::from_node(&MainSceneChild::EnemyTimer.to_string()),
            score_timer: OnReady::from_node(&MainSceneChild::ScoreTimer.to_string()),
            starter_timer: OnReady::from_node(&MainSceneChild::StarterTimer.to_string()),
            player: OnReady::from_node(&MainSceneChild::Player.to_string()),
            hud: OnReady::from_node(&MainSceneChild::HUD.to_string()),
            color_rect: OnReady::from_node(&MainSceneChild::Background.to_string()),
            music: OnReady::from_node(&MainSceneChild::Music.to_string()),
            sound_death: OnReady::from_node(&MainSceneChild::SoundDeath.to_string()),
        }
    }

    fn enter_tree(&mut self) {
        self.base()
            .try_get_node_as::<ColorRect>(&MainSceneChild::Background.to_string())
            .unwrap_or_else(|| {
                let mut background = ColorRect::new_alloc();
                background.set_name(&MainSceneChild::Background.to_string());
                self.base_mut().add_child(&background);
                background.set_owner(self.base().to_godot());
                background
            });

        self
            .base()
            .try_get_node_as::<PlayerBase>(&MainSceneChild::Player.to_string())
            .unwrap_or_else(|| {
                let mut loader = ResourceLoader::singleton();

                let scene = loader
                    .load("res://player.tscn")
                    .expect("Falha ao carregar player.tscn. Verifique se a cena se encontra no caminho: 'res://player.tscn'")
                    .cast::<PackedScene>();

                let mut player_instance = scene.try_instantiate_as::<PlayerBase>().expect("Falha ao instanciar player");

                player_instance.set_name(&MainSceneChild::Player.to_string());
                self.base_mut().add_child(&player_instance);
                player_instance.set_owner(self.base().to_godot());

                player_instance
            });

        self
            .base()
            .try_get_node_as::<HUDBase>(&MainSceneChild::HUD.to_string())
            .unwrap_or_else(|| {
                let mut loader = ResourceLoader::singleton();
                let scene = loader
                    .load("res://HUD.tscn")
                    .expect("Falha ao carregar hud.tscn. Verifique se a cena se encontra no caminho: 'res://hud.tscn'")
                    .cast::<PackedScene>();

                let mut hud_instance = scene.try_instantiate_as::<HUDBase>().expect("Falha ao instanciar o HUD");
                hud_instance.set_name(&MainSceneChild::HUD.to_string());
                self.base_mut().add_child(&hud_instance);
                hud_instance.set_owner(self.base().to_godot());

                hud_instance
            });

        self.base()
            .try_get_node_as::<Timer>(&MainSceneChild::StarterTimer.to_string())
            .unwrap_or_else(|| {
                let mut timer = Timer::new_alloc();
                timer.set_name(&MainSceneChild::StarterTimer.to_string());
                self.base_mut().add_child(&timer);
                timer.set_owner(self.base().to_godot());
                timer
            });

        self.base()
            .try_get_node_as::<Timer>(&MainSceneChild::ScoreTimer.to_string())
            .unwrap_or_else(|| {
                let mut timer = Timer::new_alloc();
                timer.set_name(&MainSceneChild::ScoreTimer.to_string());
                self.base_mut().add_child(&timer);
                timer.set_owner(self.base().to_godot());
                timer
            });

        self.base()
            .try_get_node_as::<Timer>(&MainSceneChild::EnemyTimer.to_string())
            .unwrap_or_else(|| {
                let mut timer = Timer::new_alloc();
                timer.set_name(&MainSceneChild::EnemyTimer.to_string());
                self.base_mut().add_child(&timer);
                timer.set_owner(self.base().to_godot());
                timer
            });

        self.base()
            .try_get_node_as::<Marker2D>(&MainSceneChild::StarterPosition.to_string())
            .unwrap_or_else(|| {
                let mut marker = Marker2D::new_alloc();
                marker.set_name(&MainSceneChild::StarterPosition.to_string());
                self.base_mut().add_child(&marker);
                marker.set_owner(self.base().to_godot());
                marker
            });

        let mut path_enemy = self
            .base()
            .try_get_node_as::<Path2D>(&MainSceneChild::PathEnemy.to_string())
            .unwrap_or_else(|| {
                let mut path = Path2D::new_alloc();
                path.set_name(&MainSceneChild::PathEnemy.to_string());
                self.base_mut().add_child(&path);
                path.set_owner(self.base().to_godot());
                path
            });

        if path_enemy.is_instance_valid() {
            path_enemy
                .try_get_node_as::<PathFollow2D>(&MainSceneChild::SpawnEnemy.to_string())
                .unwrap_or_else(|| {
                    let mut path_follow = PathFollow2D::new_alloc();
                    path_follow.set_name(&MainSceneChild::SpawnEnemy.to_string());
                    path_enemy.add_child(&path_follow);
                    path_follow.set_owner(self.base().to_godot());
                    path_follow
                });
        }

        self.base()
            .try_get_node_as::<AudioStreamPlayer2D>(&MainSceneChild::Music.to_string())
            .unwrap_or_else(|| {
                let mut audio = AudioStreamPlayer2D::new_alloc();
                audio.set_name(&MainSceneChild::Music.to_string());
                self.base_mut().add_child(&audio);
                audio.set_owner(self.base().to_godot());
                audio
            });

        self.base()
            .try_get_node_as::<AudioStreamPlayer2D>(&MainSceneChild::SoundDeath.to_string())
            .unwrap_or_else(|| {
                let mut audio = AudioStreamPlayer2D::new_alloc();
                audio.set_name(&MainSceneChild::SoundDeath.to_string());
                self.base_mut().add_child(&audio);
                audio.set_owner(self.base().to_godot());
                audio
            });
    }

    fn ready(&mut self) {
        randomize();

        self.player
            .signals()
            .hit()
            .connect_other(self, Self::game_over);

        self.starter_timer
            .signals()
            .timeout()
            .connect_other(self, Self::on_starter_timer_timeout);

        self.score_timer
            .signals()
            .timeout()
            .connect_other(self, Self::on_score_timer_timeout);

        self.enemy_timer
            .signals()
            .timeout()
            .connect_other(self, Self::on_enemy_timer_timeout);

        self.hud
            .signals()
            .start_game()
            .connect_other(self, Self::new_game);
    }
}

#[godot_api]
impl MainScene {
    #[func]
    fn game_over(&mut self) {
        self.score_timer.stop();
        self.enemy_timer.stop();
        self.music.stop();
        self.sound_death.play();

        let hud = self.hud.clone();
        godot::task::spawn(async move {
            HUDBase::game_over(hud).await;
        });
    }

    #[func]
    fn on_starter_timer_timeout(&mut self) {
        self.enemy_timer.start();
        self.score_timer.start();
    }

    #[func]
    fn on_score_timer_timeout(&mut self) {
        self.score += 1;
        self.hud.bind_mut().update_score(self.score);
    }

    #[func]
    fn on_enemy_timer_timeout(&mut self) {
        self.path_follow.set_progress_ratio(randf() as f32);
        let mut enemy = self
            .enemy
            .as_ref()
            .expect("Erro ao instanciar inimigo na cena")
            .try_instantiate_as::<EnemyBase>()
            .expect("Erro ao instanciar inimigo na cena");

        self.base_mut().add_child(&enemy);

        let linear_velocity = Vector2::new(
            randi_range(enemy.bind().min_speed, enemy.bind().max_speed) as f32,
            0f32,
        );

        enemy.set_position(self.path_follow.get_position());

        let mut direction = self.path_follow.get_rotation() as f64 + PI / 2f64;
        direction += randf_range(-PI / 4f64, PI / 4f64);
        enemy.set_rotation(direction as f32);

        enemy.set_linear_velocity(linear_velocity.rotated(direction as f32));
    }

    fn new_game(&mut self) {
        self.score = 0;
        self.player
            .bind_mut()
            .start(self.starter_position.get_position());

        self.starter_timer.start();
        self.hud.bind_mut().show_message("Prepare-se");
        self.hud.bind_mut().update_score(self.score);
        self.music.play();
    }
}
