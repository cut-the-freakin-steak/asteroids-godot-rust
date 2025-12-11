extends Node

var fmod2d = FmodEventEmitter2D.new()

func _ready() -> void:
	fmod2d.paused = true
