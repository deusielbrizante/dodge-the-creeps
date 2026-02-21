use godot::{
    classes::{AnimatedSprite2D, Area2D, CollisionShape2D, Engine, IArea2D, Input},
    obj::WithBaseField,
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct PlayerBase {
    #[base]
    base: Base<Area2D>,
    #[export]
    speed: i32,
    screen_size: Vector2,
}

#[godot_api]
impl IArea2D for PlayerBase {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            base,
            speed: 400,
            screen_size: Vector2::new(0f32, 0f32),
        }
    }

    fn ready(&mut self) {
        self.screen_size = self.base().get_viewport_rect().size;

        if !Engine::singleton().is_editor_hint() {
            return;
        }

        if self.base().get_node_or_null("AnimatedSprite2D").is_none() {
            let mut sprite = AnimatedSprite2D::new_alloc();
            sprite.set_name("AnimatedSprite2D");
            self.base_mut().add_child(&sprite);
            sprite.set_owner(self.base().to_godot());
        }

        if self.base().get_node_or_null("CollisionShape2D").is_none() {
            let mut collision_shape = CollisionShape2D::new_alloc();
            collision_shape.set_name("CollisionShape2D");
            self.base_mut().add_child(&collision_shape);
            collision_shape.set_owner(self.base().to_godot());
        }
    }

    fn process(&mut self, delta: f64) {
        let mut velocity = Vector2::ZERO;

        if Input::singleton().is_action_pressed("ui_right") {
            velocity.x += 1f32;
        }
        if Input::singleton().is_action_pressed("ui_left") {
            velocity.x -= 1f32;
        }
        if Input::singleton().is_action_pressed("ui_up") {
            velocity.y -= 1f32;
        }
        if Input::singleton().is_action_pressed("ui_down") {
            velocity.y += 1f32;
        }

        let mut size_texture = Vector2::new(0f32, 0f32);

        if let Some(collision) = self
            .base()
            .try_get_node_as::<CollisionShape2D>("CollisionShape2D")
        {
            size_texture = collision.get_shape().unwrap().get_rect().size;
            godot_print!("{:?}", size_texture);
        }

        if velocity.length() > 0f32 {
            velocity = velocity.normalized() * self.speed as f32;

            if let Some(mut animation) = self
                .base()
                .try_get_node_as::<AnimatedSprite2D>("AnimatedSprite2D")
            {
                animation.play();
            } else {
                godot_error!(
                    "Erro ao iniciar a animação! Não foi localizado o AnimatedSprite2D com o nome 'AnimatedSprite2D' como filho do 'PlayerBase'"
                )
            }
        } else {
            if let Some(mut animation) = self
                .base()
                .try_get_node_as::<AnimatedSprite2D>("AnimatedSprite2D")
            {
                animation.stop();
            } else {
                godot_error!(
                    "Erro ao parar a animação! Não foi localizado o AnimatedSprite2D com o nome 'AnimatedSprite2D' como filho do 'PlayerBase'"
                )
            }
        }

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
}
