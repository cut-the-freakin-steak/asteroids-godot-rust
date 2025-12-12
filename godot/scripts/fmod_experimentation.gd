extends Node2D

var fmod_event_emitter = FmodEventEmitter2D.new()

func _ready() -> void:
	FmodServer.init(
