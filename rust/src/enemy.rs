use godot::{
    classes::{
        AnimatedSprite2D, CollisionShape2D, Engine, IRigidBody2D, RigidBody2D,
        VisibleOnScreenNotifier2D,
    },
    global::randi,
    prelude::*,
};

enum EnemyChild {
    AnimatedSprite2D,
    CollisionShape2D,
    VisibleOnScreenNotifier2D,
}

impl EnemyChild {
    fn as_str(&self) -> &'static str {
        match self {
            EnemyChild::AnimatedSprite2D => "AnimatedSprite2D",
            EnemyChild::CollisionShape2D => "CollisionShape2D",
            EnemyChild::VisibleOnScreenNotifier2D => "VisibleOnScreenNotifier2D",
        }
    }
}

#[derive(GodotClass)]
#[class(base=RigidBody2D, tool)]
pub struct EnemyBase {
    #[base]
    base: Base<RigidBody2D>,
    #[export]
    pub min_speed: i64,
    #[export]
    pub max_speed: i64,
    animated_sprite: OnReady<Gd<AnimatedSprite2D>>,
    collision_shape: OnReady<Gd<CollisionShape2D>>,
    visible_notifier: OnReady<Gd<VisibleOnScreenNotifier2D>>,
}

#[godot_api]
impl IRigidBody2D for EnemyBase {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            base,
            min_speed: 150,
            max_speed: 350,
            animated_sprite: OnReady::from_node(EnemyChild::AnimatedSprite2D.as_str()),
            collision_shape: OnReady::from_node(EnemyChild::CollisionShape2D.as_str()),
            visible_notifier: OnReady::from_node(EnemyChild::VisibleOnScreenNotifier2D.as_str()),
        }
    }

    fn enter_tree(&mut self) {
        self.base()
            .try_get_node_as::<AnimatedSprite2D>(EnemyChild::AnimatedSprite2D.as_str())
            .unwrap_or_else(|| {
                let mut sprite = AnimatedSprite2D::new_alloc();
                sprite.set_name(EnemyChild::AnimatedSprite2D.as_str());
                self.base_mut().add_child(&sprite);
                sprite.set_owner(self.base().to_godot());
                sprite
            });

        self.base()
            .try_get_node_as::<CollisionShape2D>(EnemyChild::CollisionShape2D.as_str())
            .unwrap_or_else(|| {
                let mut collision_shape = CollisionShape2D::new_alloc();
                collision_shape.set_name(EnemyChild::CollisionShape2D.as_str());
                self.base_mut().add_child(&collision_shape);
                collision_shape.set_owner(self.base().to_godot());
                collision_shape
            });

        self.base()
            .try_get_node_as::<VisibleOnScreenNotifier2D>(
                EnemyChild::VisibleOnScreenNotifier2D.as_str(),
            )
            .unwrap_or_else(|| {
                let mut visible_notifier = VisibleOnScreenNotifier2D::new_alloc();
                visible_notifier.set_name(EnemyChild::VisibleOnScreenNotifier2D.as_str());
                self.base_mut().add_child(&visible_notifier);
                visible_notifier.set_owner(self.base().to_godot());
                visible_notifier
            });
    }

    fn ready(&mut self) {
        self.base_mut().set_gravity_scale(0f32);

        if let Some(sprite_frames) = self.animated_sprite.get_sprite_frames() {
            let type_enemies = sprite_frames.get_animation_names();

            self.animated_sprite
                .set_animation(type_enemies[randi() as usize % type_enemies.len()].arg());
            self.animated_sprite.play();
        } else if !Engine::singleton().is_editor_hint() {
            godot_error!(
                "Erro ao buscar os frames do AnimatedSprite2D! Não foi possível acessar a propriedade 'sprite_frames'"
            )
        }

        self.visible_notifier
            .signals()
            .screen_exited()
            .connect_other(self, Self::on_screen_exited);
    }

    fn process(&mut self, _delta: f64) {
        if Engine::singleton().is_editor_hint() {
            self.base_mut().set_process(false);
            return;
        }
    }
}

#[godot_api]
impl EnemyBase {
    #[func]
    fn on_screen_exited(&mut self) {
        self.base_mut().queue_free();
    }
}
