use std::f64::consts::PI;

use godot::{
    classes::{Marker2D, Path2D, PathFollow2D, ResourceLoader, Timer},
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
}

impl MainSceneChild {
    fn as_str(&self) -> &'static str {
        match self {
            MainSceneChild::HUD => "HUD",
            MainSceneChild::SpawnEnemy => "PathEnemy/SpawnEnemy",
            MainSceneChild::PathEnemy => "PathEnemy",
            MainSceneChild::StarterPosition => "StarterPosition",
            MainSceneChild::EnemyTimer => "EnemyTimer",
            MainSceneChild::ScoreTimer => "ScoreTimer",
            MainSceneChild::StarterTimer => "StarterTimer",
            MainSceneChild::Player => "Player",
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
}

#[godot_api]
impl INode for MainScene {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            base,
            score: 0,
            enemy: None,
            path_enemy: OnReady::from_node(MainSceneChild::PathEnemy.as_str()),
            path_follow: OnReady::from_node(MainSceneChild::SpawnEnemy.as_str()),
            starter_position: OnReady::from_node(MainSceneChild::StarterPosition.as_str()),
            enemy_timer: OnReady::from_node(MainSceneChild::EnemyTimer.as_str()),
            score_timer: OnReady::from_node(MainSceneChild::ScoreTimer.as_str()),
            starter_timer: OnReady::from_node(MainSceneChild::StarterTimer.as_str()),
            player: OnReady::from_node(MainSceneChild::Player.as_str()),
            hud: OnReady::from_node(MainSceneChild::HUD.as_str()),
        }
    }

    fn enter_tree(&mut self) {
        self
            .base()
            .try_get_node_as::<PlayerBase>(MainSceneChild::Player.as_str())
            .unwrap_or_else(|| {
                let mut loader = ResourceLoader::singleton();

                let scene = loader
                    .load("res://player.tscn")
                    .expect("Falha ao carregar player.tscn. Verifique se a cena se encontra no caminho: 'res://player.tscn'")
                    .cast::<PackedScene>();

                let mut player_instance = scene.try_instantiate_as::<PlayerBase>().expect("Falha ao instanciar player");

                player_instance.set_name(MainSceneChild::Player.as_str());
                self.base_mut().add_child(&player_instance);
                player_instance.set_owner(self.base().to_godot());

                player_instance
            });

        self
            .base()
            .try_get_node_as::<HUDBase>(MainSceneChild::HUD.as_str())
            .unwrap_or_else(|| {
                let mut loader = ResourceLoader::singleton();
                let scene = loader
                    .load("res://HUD.tscn")
                    .expect("Falha ao carregar hud.tscn. Verifique se a cena se encontra no caminho: 'res://hud.tscn'")
                    .cast::<PackedScene>();

                let mut hud_instance = scene.try_instantiate_as::<HUDBase>().expect("Falha ao instanciar o HUD");
                hud_instance.set_name(MainSceneChild::HUD.as_str());
                self.base_mut().add_child(&hud_instance);
                hud_instance.set_owner(self.base().to_godot());

                hud_instance
            });

        self.base()
            .try_get_node_as::<Timer>(MainSceneChild::StarterTimer.as_str())
            .unwrap_or_else(|| {
                let mut timer = Timer::new_alloc();
                timer.set_name(MainSceneChild::StarterTimer.as_str());
                self.base_mut().add_child(&timer);
                timer.set_owner(self.base().to_godot());
                timer
            });

        self.base()
            .try_get_node_as::<Timer>(MainSceneChild::ScoreTimer.as_str())
            .unwrap_or_else(|| {
                let mut timer = Timer::new_alloc();
                timer.set_name(MainSceneChild::ScoreTimer.as_str());
                self.base_mut().add_child(&timer);
                timer.set_owner(self.base().to_godot());
                timer
            });

        self.base()
            .try_get_node_as::<Timer>(MainSceneChild::EnemyTimer.as_str())
            .unwrap_or_else(|| {
                let mut timer = Timer::new_alloc();
                timer.set_name(MainSceneChild::EnemyTimer.as_str());
                self.base_mut().add_child(&timer);
                timer.set_owner(self.base().to_godot());
                timer
            });

        self.base()
            .try_get_node_as::<Marker2D>(MainSceneChild::StarterPosition.as_str())
            .unwrap_or_else(|| {
                let mut marker = Marker2D::new_alloc();
                marker.set_name(MainSceneChild::StarterPosition.as_str());
                self.base_mut().add_child(&marker);
                marker.set_owner(self.base().to_godot());
                marker
            });

        let mut path_enemy = self
            .base()
            .try_get_node_as::<Path2D>(MainSceneChild::PathEnemy.as_str())
            .unwrap_or_else(|| {
                let mut path = Path2D::new_alloc();
                path.set_name(MainSceneChild::PathEnemy.as_str());
                self.base_mut().add_child(&path);
                path.set_owner(self.base().to_godot());

                path
            });

        path_enemy
            .try_get_node_as::<PathFollow2D>(MainSceneChild::SpawnEnemy.as_str())
            .unwrap_or_else(|| {
                let mut path_follow = PathFollow2D::new_alloc();
                path_follow.set_name(MainSceneChild::SpawnEnemy.as_str());
                path_enemy.add_child(&path_follow);
                path_follow.set_owner(self.base().to_godot());
                path_follow
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
    }
}
