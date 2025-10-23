extends Control

@onready var scene_root_node: Node = get_tree().current_scene

@export var ui_animation: AnimationPlayer
@export var title_idle_animation: AnimationPlayer
@export var button_animation: AnimationPlayer

@export var title: Label
@export var play_button: Button
@export var settings_button: Button
@export var quit_button: Button
@export var tutorial_button: Button
@export var credits_button: Button

@export var are_you_sure: Button
@export var no_quit: Button
@export var yes_quit: Button

var game_scene: PackedScene = preload("res://scenes/main.tscn")
var settings_scene: PackedScene = preload("res://scenes/settings.tscn")
var tutorial_scene: PackedScene = preload("res://scenes/tutorial.tscn")
var credits_scene: PackedScene = preload("res://scenes/credits.tscn")

var small_ast_scene: PackedScene = preload("res://scenes/asteroid-small.tscn")
var medium_ast_scene: PackedScene = preload("res://scenes/asteroid-medium.tscn")
var big_ast_scene: PackedScene = preload("res://scenes/asteroid-big.tscn")
var asteroids: Array[PackedScene] = [small_ast_scene, medium_ast_scene, big_ast_scene]

func _ready() -> void:
	MusicManager.title_theme.play()

func _process(_delta: float) -> void:
	if not button_animation.is_playing() and (Input.is_action_just_pressed("shoot") or Input.is_action_just_pressed("skip_animation")):
		ui_animation.stop()
		play_button.visible = true
		settings_button.visible = true
		quit_button.visible = true
		tutorial_button.visible = true
		credits_button.visible = true
			
		title.modulate.a = 1.0
		play_button.modulate.a = 1.0
		settings_button.modulate.a = 1.0
		quit_button.modulate.a = 1.0
		tutorial_button.modulate.a = 1.0
		credits_button.modulate.a = 1.0

		title_idle_animation.play("idle")
		button_animation.play("idle")


func start_animations() -> void:
	ui_animation.play("ascend_title")
	if not play_button.visible:
		ui_animation.queue("pop_in_buttons")
	

func start_title_idle() -> void:
	title_idle_animation.play("idle")


func start_button_idle() -> void:
	button_animation.play("idle")


func _on_asteroid_timer_timeout() -> void:
	$Asteroids.add_child(asteroids[randi_range(0, 2)].instantiate())


func _on_play_pressed() -> void:
	MusicManager.title_theme.stop()
	SFXManager.click.play()
	get_tree().change_scene_to_packed(game_scene)


func _on_settings_pressed() -> void:	
	SFXManager.click.play()

	dissapear_node(scene_root_node)

	get_tree().get_root().add_child(settings_scene.instantiate())


func _on_quit_pressed() -> void:
	SFXManager.click.play()
	title.visible = false
	play_button.visible = false
	play_button.disabled = true
	settings_button.visible = false
	settings_button.disabled = true
	quit_button.visible = false
	quit_button.disabled = true
	tutorial_button.visible = false
	tutorial_button.disabled = true
	credits_button.visible = false
	credits_button.disabled = true

	are_you_sure.visible = true
	are_you_sure.disabled = false
	yes_quit.visible = true
	yes_quit.disabled = false
	no_quit.visible = true
	no_quit.disabled = false


func _on_are_you_sure_pressed() -> void:
	SFXManager.click.play()
	OS.shell_open("https://i.ytimg.com/vi/YSWMYnuOImg/hqdefault.jpg")


func _on_no_quit_pressed() -> void:
	SFXManager.click.play()
	are_you_sure.visible = false
	are_you_sure.disabled = true
	yes_quit.visible = false
	yes_quit.disabled = true
	no_quit.visible = false
	no_quit.disabled = true
	
	title.visible = true
	play_button.visible = true
	play_button.disabled = false
	settings_button.visible = true
	settings_button.disabled = false
	quit_button.visible = true
	quit_button.disabled = false
	tutorial_button.visible = true
	tutorial_button.disabled = false
	credits_button.visible = true
	credits_button.disabled = false


func _on_yes_quit_pressed() -> void:
	SFXManager.click.play()
	get_tree().quit()


func _on_tutorial_pressed() -> void:
	SFXManager.click.play()

	dissapear_node(scene_root_node)

	get_tree().get_root().add_child(tutorial_scene.instantiate())
	

func _on_credits_pressed() -> void:
	SFXManager.click.play()
	
	dissapear_node(scene_root_node)

	get_tree().get_root().add_child(credits_scene.instantiate())
	
func dissapear_node(node: Node) -> void:
	node.visible = false

	var buttons = get_tree().get_nodes_in_group("button")
	for button in buttons:
		button.disabled = true


func appear_node(node: Node) -> void:
	node.visible = true

	var buttons = get_tree().get_nodes_in_group("button")
	for button in buttons:
		button.disabled = false
