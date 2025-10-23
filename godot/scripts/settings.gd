extends Control

@export var scene_root_node: Control
@onready var main_menu_node: Control = get_node("/root").get_node_or_null("MainMenu")
@onready var game_scene_node: Node2D = get_node("/root").get_node_or_null("Main")

@export var vsync_toggle: CheckButton
@export var screen_shake_toggle: CheckButton 
@export var hurricane_mode_toggle: CheckButton

@export var master_volume_slider: HSlider
@export var music_volume_slider: HSlider
@export var sfx_volume_slider: HSlider

@onready var master_bus: FmodBus = FmodServer.get_bus("bus:/")
@onready var music_bus: FmodBus = FmodServer.get_bus("bus:/Music")
@onready var sfx_bus: FmodBus = FmodServer.get_bus("bus:/SFX")

# check if we gotta go
func _process(_delta: float) -> void:
	if Input.is_action_just_pressed("esc"):
		exit_settings()

# load settings to gui
func _ready() -> void:
	match Settings.vsync_on:
		true:
			DisplayServer.window_set_vsync_mode(DisplayServer.VSYNC_ENABLED)
			vsync_toggle.button_pressed = true
		
		false:
			DisplayServer.window_set_vsync_mode(DisplayServer.VSYNC_DISABLED)
			vsync_toggle.button_pressed = false

	match Settings.screen_shake_on:
		true:
			screen_shake_toggle.button_pressed = true
		
		false:
			screen_shake_toggle.button_pressed = false
			
	match Settings.hurricane_mode:
		true:
			hurricane_mode_toggle.button_pressed = true
			
		false:
			hurricane_mode_toggle.button_pressed = false

	master_bus.volume = Settings.master_volume
	master_volume_slider.value = Settings.master_volume

	music_bus.volume = Settings.music_volume
	music_volume_slider.value = Settings.music_volume

	sfx_bus.volume = Settings.sfx_volume
	sfx_volume_slider.value = Settings.sfx_volume
	
# save all settings and restore menus
func exit_settings() -> void:
	Settings.save_settings()

	dissapear_node(scene_root_node)

	if main_menu_node != null:
		appear_node(main_menu_node)

	if game_scene_node != null:
		# appear_root_node() wouldnt work here because it doesnt have the following 4 lines of code
		game_scene_node.pause_label.visible = true
		game_scene_node.resume_button.visible = true
		game_scene_node.pause_settings_button.visible = true
		game_scene_node.pause_main_menu_button.visible = true

		var game_scene_buttons = game_scene_node.get_tree().get_nodes_in_group("button")
		for button in game_scene_buttons:
			button.disabled = false
			
	queue_free()


func _on_return_pressed() -> void:
	SFXManager.click.play()
	exit_settings()

#! misc settings
func _on_v_sync_toggle_toggled(toggled_on: bool) -> void:
	SFXManager.click.play()

	if toggled_on:
		DisplayServer.window_set_vsync_mode(DisplayServer.VSYNC_ENABLED)
		Settings.vsync_on = true
		
	else:
		DisplayServer.window_set_vsync_mode(DisplayServer.VSYNC_DISABLED)
		Settings.vsync_on = false

	Settings.save_settings()


func _on_screen_shake_toggle_toggled(toggled_on: bool) -> void:
	SFXManager.click.play()

	if toggled_on:
		Settings.screen_shake_on = true
	
	else:
		Settings.screen_shake_on = false

	Settings.save_settings()


func _on_hurricane_mode_toggle_toggled(toggled_on: bool) -> void:
	SFXManager.click.play()

	if toggled_on:
		Settings.hurricane_mode = true
	
	else:
		Settings.hurricane_mode = false

	Settings.save_settings()

#! audio settings
func _on_master_slider_value_changed(value: float) -> void:
	Settings.master_volume = value
	master_bus.volume = Settings.master_volume


func _on_music_slider_value_changed(value: float) -> void:
	Settings.music_volume = value
	music_bus.volume = Settings.music_volume


func _on_sfx_slider_value_changed(value: float) -> void:
	Settings.sfx_volume = value
	sfx_bus.volume = Settings.sfx_volume
	
	
func dissapear_node(node: Node) -> void:
	node.visible = false

	var buttons = node.get_tree().get_nodes_in_group("button")
	for button in buttons:
		button.disabled = true
	
func appear_node(node: Node) -> void:
	node.visible = true

	var buttons = node.get_tree().get_nodes_in_group("button")
	for button in buttons:
		button.disabled = false
