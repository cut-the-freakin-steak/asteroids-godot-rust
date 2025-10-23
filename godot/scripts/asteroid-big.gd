extends Asteroid

var camera_manager: Node
var rotation_speed: int = randi_range(1, 2)

func _ready() -> void:
	super()
	randomize()
	horizontal_speed = randi_range(15, 30) * direction.x

	if main.name == "Main":
		camera_manager = main.get_node("CameraManager")


func _physics_process(delta: float) -> void:
	super(delta)

	if "is_paused" in main:
		if main.is_paused:
			return

	rotation += rotation_speed * delta

	position.x += horizontal_speed * delta

func split_in_two() -> void:
	main.score += 1
	main.emit_signal("asteroid_hit", "big", global_position)
	explosion_parts.emitting = true
	sprite.visible = false
	collision.disabled = true
	SFXManager.explosion.set_parameter("WhichSound", "Big")
	SFXManager.explosion.play()
	explosion_to_queue_free.start()
	camera_manager.screen_shake(2.5, 0.3)


func _on_explosion_to_queue_free_timeout() -> void:
	queue_free()
