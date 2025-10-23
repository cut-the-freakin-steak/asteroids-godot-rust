extends CharacterBody2D

signal damage_taken

@onready var main: Node2D = get_tree().current_scene
@onready var screen_size: float = get_viewport_rect().size.x
@onready var sprite_dimensions: Vector2 = get_visible_sprite_dimensions(sprite)

@onready var screen_wrap_stuff: Node2D = main.get_node("PlayerStuff/SWStuff")
@onready var opposite_screen_sprite: Sprite2D = main.get_node("PlayerStuff/SWStuff/OtherSideSprite")
@onready var other_side_back_particles: GPUParticles2D = main.get_node("PlayerStuff/SWStuff/OpBackFireParticles")
@onready var game_over_text: Label = main.get_node("UI/GameOver")

@export var sprite: Sprite2D
@export var back_particles: GPUParticles2D

@export var i_frame_timer: Timer
@export var shoot_timer: Timer

var max_speed: int = 100
var acceleration: float = 2.5
var rotation_speed: float = 5.5
var ship_on_screen_border_x: bool = false
var ship_on_screen_border_y: bool = false
var last_direction_faced: Vector2
var alive: bool = true
var hp: int = 3
var can_be_damaged: bool = true
var millisecond_even_or_odd: int = 0

func _ready() -> void:
	damage_taken.connect(_on_damage_taken)


func _process(delta: float) -> void:
	if main.is_game_over or Input.is_action_pressed("decelerate"):
		SFXManager.ship_thruster.stop()
		
	else:
		if Input.is_action_just_pressed("up"):
			SFXManager.ship_thruster.set_parameter("ShouldLoop", "Yes")
			SFXManager.ship_thruster.play()
			
		if Input.is_action_pressed("up"):
			SFXManager.ship_thruster.play(false)
			if SFXManager.ship_thruster.get_parameter("PitchParam") < 0.20:
				SFXManager.ship_thruster.set_parameter("PitchParam", SFXManager.ship_thruster.get_parameter("PitchParam") + (0.10 * delta))
		
		if not Input.is_action_pressed("up") and SFXManager.ship_thruster.get_parameter("ShouldLoop") == "Yes":
			SFXManager.ship_thruster.stop()
			SFXManager.ship_thruster.set_parameter("PitchParam", 0.0)


func _physics_process(delta) -> void:
	if not alive:
		back_particles.emitting = false
		velocity = velocity.move_toward(Vector2.ZERO, 1)

		if velocity == Vector2.ZERO:
			return
		
	if main.is_paused:
		back_particles.speed_scale = 0
		back_particles.emitting = false
		return
	
	else:
		back_particles.speed_scale = 1

	# screen wrapping
	if global_position.x - 5 > screen_size:
		global_position.x = 5

	if global_position.x + 5 < 0:
		global_position.x = screen_size - 5

	if global_position.y - 5 > screen_size:
		global_position.y = 5

	if global_position.y + 5 < 0:
		global_position.y = screen_size - 5

	show_screen_wrapped_ship()

	# movement
	if alive:
		var rotation_direction: float = Input.get_axis("left", "right")
		var movement_vector: Vector2 = Vector2(0, Input.get_axis("down", "up"))
		if Input.is_action_pressed("decelerate"):
			movement_vector.y = -1

		if movement_vector.y == 1:
			velocity += movement_vector.rotated(rotation - deg_to_rad(90)) * acceleration
			back_particles.emitting = true

		if movement_vector.y == 0:
			velocity = velocity.move_toward(Vector2.ZERO, 1.3)
			back_particles.emitting = false

		if movement_vector.y == -1:
			velocity = velocity.move_toward(Vector2.ZERO, 3)
			back_particles.emitting = false

		velocity = velocity.limit_length(max_speed)

		rotate(rotation_speed * rotation_direction * delta)

		# i-frame flashing
		if i_frame_timer.time_left > 0.5:
			millisecond_even_or_odd = int(str(i_frame_timer.time_left).split('.')[1]) % 3
			if millisecond_even_or_odd == 0:
				sprite.visible = false

			else:
				sprite.visible = true

		elif i_frame_timer.time_left > 0:
			millisecond_even_or_odd = int(str(i_frame_timer.time_left).split('.')[1]) % 2
			if millisecond_even_or_odd == 0:
				sprite.visible = false

			else:
				sprite.visible = true

	move_and_slide()


# show ship at other side of screen when player going OOB
func show_screen_wrapped_ship() -> void:
	# x-axis
	# right
	if global_position.x >= screen_size - (sprite_dimensions.x - sprite_dimensions.x / 2):
		ship_on_screen_border_x = true
		screen_wrap_stuff.global_position = Vector2(position.x - screen_size, position.y)
		opposite_screen_sprite.visible = true
		screen_wrap_stuff.rotation_degrees = rotation_degrees + 90

		if Input.is_action_pressed("up"):
			other_side_back_particles.emitting = true

		else:
			other_side_back_particles.emitting = false

		# left
	elif global_position.x <= 0 + (sprite_dimensions.x / 2):
		ship_on_screen_border_x = true
		screen_wrap_stuff.global_position = Vector2(position.x + screen_size, position.y)
		opposite_screen_sprite.visible = true
		screen_wrap_stuff.rotation_degrees = rotation_degrees + 90

		if Input.is_action_pressed("up"):
			other_side_back_particles.emitting = true

		else:
			other_side_back_particles.emitting = false

	else:
		ship_on_screen_border_x = false
		opposite_screen_sprite.visible = false
		other_side_back_particles.emitting = false

	# y-axis
	# bottom
	if global_position.y >= screen_size - (float(sprite.texture.get_height()) / 2 + 8):
		ship_on_screen_border_y = true
		screen_wrap_stuff.global_position = Vector2(position.x, position.y - screen_size)
		opposite_screen_sprite.visible = true
		screen_wrap_stuff.rotation_degrees = rotation_degrees + 90

		if Input.is_action_pressed("up"):
			other_side_back_particles.emitting = true

		else:
			other_side_back_particles.emitting = false

		# top
	elif global_position.y <= 0 + (float(sprite.texture.get_height()) / 2 - 8):
		ship_on_screen_border_y = true
		screen_wrap_stuff.global_position = Vector2(position.x, position.y + screen_size)
		opposite_screen_sprite.visible = true
		screen_wrap_stuff.rotation_degrees = rotation_degrees + 90

		if Input.is_action_pressed("up"):
			other_side_back_particles.emitting = true

		else:
			other_side_back_particles.emitting = false

	else:
		ship_on_screen_border_y = false
		if not ship_on_screen_border_x:
			opposite_screen_sprite.visible = false
			other_side_back_particles.emitting = false

	if not alive:
		other_side_back_particles.emitting = false


func get_visible_sprite_dimensions(sprite2d: Sprite2D) -> Vector2:
	if sprite2d.texture == null:
		return Vector2(0, 0)

	var image: Image = sprite2d.texture.get_image()
	var used_rect: Rect2i = image.get_used_rect()
	var visible_pixels_v2: Vector2 = Vector2(used_rect.size.x, used_rect.size.y)

	return visible_pixels_v2


func _on_damage_taken() -> void:
	if not can_be_damaged:
		return

	hp -= 1

	match hp:
		2:
			SFXManager.player_hurt.set_parameter("HPRemaining", "Alive")
			SFXManager.player_hurt.play()
			main.get_node("UI/HealthUI/Health").play("lose_health_1")

		1:
			SFXManager.player_hurt.set_parameter("HPRemaining", "Alive")
			SFXManager.player_hurt.play()
			main.get_node("UI/HealthUI/Health").play("lose_health_2")

		0:
			SFXManager.player_hurt.set_parameter("HPRemaining", "Dead")
			SFXManager.player_hurt.play()
			main.get_node("UI/HealthUI/Health").play("lose_health_3")

	if hp > 0:
		can_be_damaged = false
		i_frame_timer.start()
		velocity += Vector2(0, -100).rotated(rotation - deg_to_rad(90))

	if hp <= 0:
		# you are die lmao
		if alive:
			velocity += Vector2(0, -200).rotated(rotation - deg_to_rad(90))
			main.game_over.emit()

		alive = false	


func _on_i_frame_timer_timeout() -> void:
	sprite.visible = true
	can_be_damaged = true
