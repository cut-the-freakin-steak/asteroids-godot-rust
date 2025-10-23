extends Asteroid

var rotation_speed: int = randi_range(4, 5)

func _ready() -> void:
	super()
	randomize()
	horizontal_speed = randi_range(40, 55) * direction.x


func _physics_process(delta: float) -> void:
	super(delta)

	if "is_paused" in main:
		if main.is_paused:
			return

	rotation += rotation_speed * delta

	position.x += horizontal_speed * delta
	

func split_in_two() -> void:
	main.score += 3
	explosion_parts.emitting = true
	sprite.visible = false
	collision.disabled = true
	SFXManager.explosion.set_parameter("WhichSound", "SmallMed")
	SFXManager.explosion.play()
	explosion_to_queue_free.start()


func _on_explosion_to_queue_free_timeout() -> void:
	queue_free()
