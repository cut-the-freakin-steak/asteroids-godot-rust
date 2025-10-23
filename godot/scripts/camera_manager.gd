extends Node

@onready var camera: PhantomCamera2D = $PhantomCamera2D

var shake_intensity: float = 0.0
var active_shake_time: float = 0.0

var shake_decay: float = 5.0

var shake_time: float = 0.0
var shake_time_speed: float = 20.0

var noise = FastNoiseLite.new()

func _physics_process(delta: float) -> void:
	if active_shake_time > 0:
		shake_time += shake_time_speed * delta
		active_shake_time -= delta

		camera.position = Vector2(
			noise.get_noise_2d(shake_time, 0) * shake_intensity + 100,
			noise.get_noise_2d(0, shake_time) * shake_intensity + 100
		)
		
		shake_intensity = max(shake_intensity - shake_decay * delta, 0)

	else:
		camera.position = lerp(camera.position, Vector2(100, 100), 10.5 * delta)


func screen_shake(intensity: float, time: float) -> void:
	if Settings.screen_shake_on:
		randomize()
		noise.seed = randi()
		noise.frequency = 2.0

		shake_intensity = intensity
		active_shake_time = time
		shake_time = 0.0
