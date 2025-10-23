extends Node
 
@onready var window_size = DisplayServer.window_get_size()
var window_w: float
var window_h: float
 
func _process(_delta: float):
	# check to see if the window size changes
	if DisplayServer.window_get_size() != window_size:
		# get window width
		window_w = DisplayServer.window_get_size().x
		# since the resolution is 1:1 divide the width by 1 and multiply by 1 to get height
		window_h = window_w / 1.0 * 1 # if the aspect ratio would be 1:1, this would be a whole lot more useful lmao
	# set the window size to the current width and the new height
		DisplayServer.window_set_size(Vector2(window_w, window_h))
	# change the window_size variable to match the new size
		window_size = DisplayServer.window_get_size()

#! DONT PAY ATTENTION TO ANYTHING BELOW THIS COMMENT, THIS IS JUST FOR FUNNY MODE

# var shake_intensity: float = 0.0
# var active_shake_time: float = 0.0
#
# var shake_decay: float = 5.0
#
# var shake_time: float = 0.0
# var shake_time_speed: float = 20.0
#
# var noise = FastNoiseLite.new()
#
# func _physics_process(_delta: float) -> void:
	# window_shake(1000, 0.2)
	# if active_shake_time > 0:
	# 	shake_time += shake_time_speed * delta
	# 	active_shake_time -= delta
	#
	# 	DisplayServer.window_set_position(Vector2(
	# 		noise.get_noise_2d(shake_time, 0) * shake_intensity + 100,
	# 		noise.get_noise_2d(0, shake_time) * shake_intensity + 100
	# 	))
	# 	
	# 	shake_intensity = max(shake_intensity - shake_decay * delta, 0)
	#
	# else:
	# 	DisplayServer.window_set_position(lerp(DisplayServer.window_get_position(), Vector2(100, 100), 10.5 * delta))
#
#
# func window_shake(intensity: float, time: float) -> void:
# 	if Settings.screen_shake_on:
# 		randomize()
# 		noise.seed = randi()
# 		noise.frequency = 2.0
#
# 		shake_intensity = intensity
# 		active_shake_time = time
# 		shake_time = 0.0
