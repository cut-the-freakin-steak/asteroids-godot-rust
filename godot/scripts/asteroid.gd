extends Area2D
class_name Asteroid

@onready var main: Node = get_tree().current_scene

@onready var sprite: Sprite2D = $Sprite2D
@onready var collision: CollisionPolygon2D = $CollisionPolygon2D
@onready var explosion_parts: GPUParticles2D = $AsteroidExplosion
@onready var explosion_to_queue_free: Timer = $ExplosionToQueueFree

var direction: Vector2 = Vector2(0, 0)
var vertical_speed: float = 0
var horizontal_speed: float = 0
var use_set_position: bool = false

func _ready() -> void:
	randomize()

	if not use_set_position:
		var asteroid_markers: Array[Node] = main.get_node("AsteroidMarkers").get_children()
		var selected_asteroid_spawn: Marker2D = asteroid_markers[randi() % asteroid_markers.size()]

		position = selected_asteroid_spawn.position

	if position.x <= 50:
		direction.x = 1

	elif position.x >= 150:
		direction.x = -1

	else:
		var ones: Array = [-1, 1]
		direction.x = ones[randi() % ones.size()]

	if position.y <= 50:
		direction.y = 1

	elif position.y >= 150:
		direction.y = -1

	else:
		var ones: Array = [-1, 1]
		direction.y = ones[randi() % ones.size()]

	var vert_speed_coin_flip: int = randi_range(0, 1)

	if vert_speed_coin_flip == 0:
		vertical_speed = randi_range(20, 30) * direction.y

	else:
		vertical_speed = randi_range(35, 45) * direction.y

	body_entered.connect(_on_body_entered)
	
	


func _physics_process(delta: float) -> void:
	if "is_paused" in main:
		if main.is_paused:
			return

	position.y += vertical_speed * delta

	if position.x > 250 or position.x < -50:
		queue_free()

	if position.y > 250 or position.y < -50:
		queue_free()


# handle collisions with player
func _on_body_entered(body: Node2D):
	if body.is_in_group("player"):
		body.damage_taken.emit()
