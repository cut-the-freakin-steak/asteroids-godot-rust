extends Node

var fmod2d = FmodEventEmitter2D.new()

func _ready() -> void:
	FmodServer.set_software_format(
	FmodSoftwareFormatSettings
