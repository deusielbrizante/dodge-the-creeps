use godot::{
    classes::{AnimatedSprite2D, Area2D, CollisionShape2D, Engine, IArea2D, Input},
    prelude::*,
};

enum PlayerChild {
    AnimatedSprite2D,
    CollisionShape2D,
}

impl PlayerChild {
    fn as_str(&self) -> &'static str {
        match self {
            PlayerChild::AnimatedSprite2D => "AnimatedSprite2D",
            PlayerChild::CollisionShape2D => "CollisionShape2D",
        }
    }
}

#[derive(GodotClass)]
#[class(base=Area2D, tool)]
pub struct PlayerBase {
    #[base]
    base: Base<Area2D>,
    #[export]
    speed: i32,
    screen_size: Vector2,
    animated_sprite: OnReady<Gd<AnimatedSprite2D>>,
    collision_shape: OnReady<Gd<CollisionShape2D>>,
}

#[godot_api]
impl IArea2D for PlayerBase {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            base,
            speed: 400,
            screen_size: Vector2::ZERO,
            animated_sprite: OnReady::from_node(PlayerChild::AnimatedSprite2D.as_str()),
            collision_shape: OnReady::from_node(PlayerChild::CollisionShape2D.as_str()),
        }
    }

    fn enter_tree(&mut self) {
        self.base()
            .try_get_node_as::<AnimatedSprite2D>(PlayerChild::AnimatedSprite2D.as_str())
            .unwrap_or_else(|| {
                let mut animated_sprite = AnimatedSprite2D::new_alloc();
                animated_sprite.set_name(PlayerChild::AnimatedSprite2D.as_str());
                self.base_mut().add_child(&animated_sprite);
                animated_sprite.set_owner(self.base().to_godot());
                animated_sprite
            });

        self.base()
            .try_get_node_as::<CollisionShape2D>(PlayerChild::CollisionShape2D.as_str())
            .unwrap_or_else(|| {
                let mut collision_shape = CollisionShape2D::new_alloc();
                collision_shape.set_name(PlayerChild::CollisionShape2D.as_str());
                self.base_mut().add_child(&collision_shape);
                collision_shape.set_owner(self.base().to_godot());
                collision_shape
            });
    }

    fn ready(&mut self) {
        self.base_mut().hide();
        self.screen_size = self.base().get_viewport_rect().size;

        self.signals()
            .body_entered()
            .connect_self(Self::on_player_body_entered);
    }

    fn process(&mut self, delta: f64) {
        if Engine::singleton().is_editor_hint() {
            self.base_mut().set_process(false);
            return;
        }

        let mut velocity = self.get_input_velocity();

        if velocity.length() > 0f32 {
            velocity = velocity.normalized() * self.speed as f32;
        }

        self.update_animation(velocity);
        self.set_movement(velocity, delta);
    }
}

#[godot_api]
impl PlayerBase {
    #[signal]
    pub fn hit();

    #[func]
    pub fn on_player_body_entered(&mut self, _body: Gd<Node2D>) {
        self.base_mut().hide();
        self.signals().hit().emit();
        self.collision_shape
            .set_deferred("disabled", &true.to_variant());
    }

    pub fn start(&mut self, position: Vector2) {
        self.base_mut().set_position(position);
        self.base_mut().show();
        self.collision_shape.set_disabled(false);
    }

    fn update_animation(&mut self, velocity: Vector2) {
        let is_up = velocity.y > 0f32;
        let moving_x = velocity.x != 0f32;

        if moving_x {
            self.animated_sprite.set_animation("right");
            self.animated_sprite.set_flip_h(velocity.x < 0f32);
        }

        if velocity.y != 0f32 {
            self.animated_sprite.set_flip_v(is_up);
            if !moving_x {
                self.animated_sprite.set_animation("up");
            }
        }

        if velocity.length() > 0f32 {
            self.animated_sprite.play();
        } else {
            self.animated_sprite.stop();
        }
    }

    fn set_movement(&mut self, velocity: Vector2, delta: f64) {
        let size_texture = self
            .collision_shape
            .get_shape()
            .map(|shape| shape.get_rect().size)
            .unwrap_or(Vector2::ZERO);

        let actual_position = self.base().get_position();
        let new_position = actual_position + (velocity * delta as real);

        let new_position_limit_viewport = Vector2::new(
            new_position.x.clamp(
                size_texture.x / 2f32,
                self.screen_size.x - size_texture.x / 2f32,
            ),
            new_position.y.clamp(
                size_texture.y / 2f32,
                self.screen_size.y - size_texture.y / 2f32,
            ),
        );

        self.base_mut().set_position(new_position_limit_viewport);
    }

    fn get_input_velocity(&self) -> Vector2 {
        let input = Input::singleton();

        Vector2::new(
            input.get_action_strength("ui_right") - input.get_action_strength("ui_left"),
            input.get_action_strength("ui_down") - input.get_action_strength("ui_up"),
        )
    }
}
