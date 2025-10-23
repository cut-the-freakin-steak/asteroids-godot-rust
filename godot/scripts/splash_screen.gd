extends Control

@export var splash_screen: TextureRect

var main_menu_scene: PackedScene = preload("res://scenes/main_menu.tscn")
var in_time: float = 0.5
var fade_in_time: float = 1.5
var pause_time: float = 1.5
var fade_out_time: float = 1.5
var out_time: float = 1.25

func _ready() -> void:
	Settings.load_settings()
	fade()
	SFXManager.splash_screen.play()


func fade() -> void:
	splash_screen.modulate.a = 0.0
	
	var tween = self.create_tween()
	tween.tween_interval(in_time)
	tween.tween_property(splash_screen, "modulate:a", 1.0, fade_in_time)
	tween.tween_interval(pause_time)
	tween.tween_property(splash_screen, "modulate:a", 0.0, fade_out_time)
	tween.tween_interval(out_time)
	await tween.finished
	get_tree().change_scene_to_packed(main_menu_scene)
