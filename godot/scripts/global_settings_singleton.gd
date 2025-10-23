extends Node

var settings_save_path = "user://settings.json"
var vsync_on: bool = true
var screen_shake_on: bool = true
var hurricane_mode: bool = false

var master_volume: float = 1.0
var music_volume: float = 1.0
var sfx_volume: float = 1.0

@onready var master_bus: FmodBus = FmodServer.get_bus("bus:/")
@onready var music_bus: FmodBus = FmodServer.get_bus("bus:/Music")
@onready var sfx_bus: FmodBus = FmodServer.get_bus("bus:/SFX")

func save_settings() -> void:
	var settings_dict = {
		"vsync_on": vsync_on,
		"screen_shake_on": screen_shake_on,
		"hurricane_mode": hurricane_mode,
		"master_volume": master_volume,
		"music_volume": music_volume,
		"sfx_volume": sfx_volume
	}

	var json_string = JSON.stringify(settings_dict, "\t")
	var file: FileAccess = FileAccess.open(settings_save_path, FileAccess.WRITE)
	
	if file == null:
		printerr("the goddamn file returned null goddamnit")
		return

	file.store_string(json_string)
	file.close()


func load_settings() -> void:
	if not FileAccess.file_exists(settings_save_path):
		return

	var file = FileAccess.open(settings_save_path, FileAccess.READ)
	
	if file == null:
		printerr("the goddamn file returned null goddamnit")
		return

	var json_string = file.get_as_text()
	
	file.close()
	
	var json = JSON.new()

	# parse json and check result to make sure settings arent corrupt
	var parsed_result = json.parse(json_string)
	
	if parsed_result != OK:
		printerr("failed to parse the goddamn settings goddamnit")
		return
	
	var settings_dict = json.data
	
	# load each setting and update variable values with a fallback value just in case
	vsync_on = settings_dict.get("vsync_on", vsync_on)
	screen_shake_on = settings_dict.get("screen_shake_on", screen_shake_on)
	hurricane_mode = settings_dict.get("hurricane_mode", hurricane_mode)

	master_volume = settings_dict.get("master_volume", master_volume)
	music_volume = settings_dict.get("music_volume", music_volume)
	sfx_volume = settings_dict.get("sfx_volume", sfx_volume)
	
	# any settings not seen here handle their behaviour based on a variable, and they aren't set with a specific command
	match vsync_on:
		true:
			DisplayServer.window_set_vsync_mode(DisplayServer.VSYNC_ENABLED)
		
		false:
			DisplayServer.window_set_vsync_mode(DisplayServer.VSYNC_DISABLED)

	master_bus.volume = master_volume
	music_bus.volume = music_volume
	sfx_bus.volume = sfx_volume
