extends Node2D

signal game_over
signal asteroid_hit(asteroid_size: String, position: Vector2)

@onready var player: CharacterBody2D = $PlayerStuff/Player
@onready var scene_root_node: Node = get_tree().current_scene
@onready var game_over_text: Label = $UI/GameOver
@onready var try_again_button: Button = $UI/TryAgain
@onready var main_menu_button: Button = $UI/MainMenu
@onready var ui_animation: AnimationPlayer = $UI/UIAnimation
@onready var game_over_label_animation: AnimationPlayer = $UI/GameOverAnimation
@onready var game_over_buttons_animation: AnimationPlayer = $UI/ButtonAnimation
@onready var game_over_animation_timer: Timer = $UI/GameOverAnimTimer
@onready var score_label: Label = $UI/ScoreText
@onready var camera_manager: Node = $CameraManager
@onready var pause_ui: Control = $UI/PauseUI
@onready var pause_label: Label = $UI/PauseUI/Pause
@onready var resume_button: Button = $UI/PauseUI/Resume
@onready var pause_settings_button: Button = $UI/PauseUI/Settings
@onready var pause_main_menu_button: Button = $UI/PauseUI/MainMenu
@onready var alive_to_dead_music_timer: Timer = $AliveToDeadMusicTimer

var main_menu_scene: PackedScene = load("res://scenes/main_menu.tscn")
var settings_scene: PackedScene = preload("res://scenes/settings.tscn")
var game_scene: PackedScene = preload("res://scenes/main.tscn")
var small_ast_scene: PackedScene = preload("res://scenes/asteroid-small.tscn")
var medium_ast_scene: PackedScene = preload("res://scenes/asteroid-medium.tscn")
var big_ast_scene: PackedScene = preload("res://scenes/asteroid-big.tscn")
var asteroids: Array[PackedScene] = [small_ast_scene, medium_ast_scene, big_ast_scene]

var laser: PackedScene = preload("res://scenes/bullet.tscn")
var score: int = 0
var is_game_over: bool = false
var is_paused: bool = false
var game_over_anim_skipped: bool = false

func _ready():
	if Settings.hurricane_mode:
		MusicManager.gameplay.set_parameter("WhichGameplaySong", "Hurricane")
		
	else:
		MusicManager.gameplay.set_parameter("WhichGameplaySong", "Normal")

	MusicManager.gameplay.set_parameter("MuffledOrNot", 1.0)
	MusicManager.gameplay.set_parameter("NormalGameplaySongPitch", 0.0)

	score = 0
	is_game_over = false
	game_over_anim_skipped = false
	game_over.connect(_on_game_over)
	asteroid_hit.connect(_spawn_asteroid)


func _process(_delta: float) -> void:	
	if Input.is_action_just_pressed("esc"):
		if not is_game_over:
			if not is_paused:
				is_paused = true
				pause_ui.visible = true
				resume_button.disabled = false
				pause_settings_button.disabled = false
				pause_main_menu_button.disabled = false
				MusicManager.gameplay.set_parameter("MuffledOrNot", 0.40)
		
			else:
				pause_ui.visible = false
				resume_button.disabled = true
				pause_settings_button.disabled = true
				pause_main_menu_button.disabled = true
				is_paused = false
				MusicManager.gameplay.set_parameter("MuffledOrNot", 1.0)

	if is_paused:
		if Settings.hurricane_mode:
			if MusicManager.gameplay.get_parameter("WhichGameplaySong") != "Hurricane":
				MusicManager.gameplay.stop()
				MusicManager.gameplay.set_parameter("WhichGameplaySong", "Hurricane")
				MusicManager.gameplay.play(false)
	
		else:
			if MusicManager.gameplay.get_parameter("WhichGameplaySong") != "Normal":
				MusicManager.gameplay.stop()
				MusicManager.gameplay.set_parameter("WhichGameplaySong", "Normal")
				MusicManager.gameplay.play(false)

		return

	if is_game_over != true:
		MusicManager.gameplay.play(false)

	if Input.is_action_just_pressed("shoot") and player.alive and player.shoot_timer.time_left == 0:
		var new_laser: CharacterBody2D = laser.instantiate()
		new_laser.global_position = player.get_node("ShootOrigin").global_position
		$Lasers.add_child(new_laser)
		player.shoot_timer.start()
		
	score_label.text = "Score: " + str(score)
	
	if is_game_over and not game_over_buttons_animation.is_playing() and Input.is_action_just_pressed("skip_animation"):
		game_over_anim_skipped = true
		ui_animation.stop()
		try_again_button.visible = true
		main_menu_button.visible = true
		game_over_text.modulate.a = 1.0
		try_again_button.modulate.a = 1.0
		main_menu_button.modulate.a = 1.0
		game_over_label_animation.play("idle")
		game_over_buttons_animation.play("idle")
		
	if Settings.hurricane_mode:
		camera_manager.screen_shake(3.5, 0.2)
		

func pop_in_buttons() -> void:
	if game_over_anim_skipped:
		return
	
	ui_animation.play("pop_in_game_over_buttons")
	# also play animation for label moving around
	game_over_label_animation.play("idle")
		

func button_idle_animation() -> void:
	game_over_buttons_animation.play("idle")
	
	
func _spawn_asteroid(asteroid_size: String, ast_position: Vector2) -> void:
	if asteroid_size == "big":
		var new_ast1: Asteroid = medium_ast_scene.instantiate()
		var new_ast2: Asteroid = medium_ast_scene.instantiate()
		new_ast1.use_set_position = true
		new_ast2.use_set_position = true
		new_ast1.global_position = ast_position
		new_ast2.global_position = ast_position
		$Asteroids.call_deferred("add_child", new_ast1)
		$Asteroids.call_deferred("add_child", new_ast2)
	
	elif asteroid_size == "medium":
		var new_ast1: Asteroid = small_ast_scene.instantiate()
		var new_ast2: Asteroid = small_ast_scene.instantiate()
		new_ast1.use_set_position = true
		new_ast2.use_set_position = true
		new_ast1.global_position = ast_position
		new_ast2.global_position = ast_position
		$Asteroids.call_deferred("add_child", new_ast1)
		$Asteroids.call_deferred("add_child", new_ast2)
		
	else:
		pass


func _on_game_over() -> void:
	is_game_over = true
	game_over_text.visible = true
	ui_animation.play("appear_game_over_text")
	game_over_animation_timer.start()
	MusicManager.gameplay.set_parameter("NormalGameplaySongPitch", -1.0)
	alive_to_dead_music_timer.start()
		

func _on_asteroid_timer_timeout() -> void:
	if is_paused:
		return

	else:	
		$Asteroids.add_child(asteroids[randi_range(0, 2)].instantiate())


func _on_game_over_anim_timer_timeout() -> void:
	ui_animation.play("ascend_game_over_text")


func _on_resume_pressed() -> void:
	SFXManager.click.play()
	if Settings.hurricane_mode:
		MusicManager.gameplay.set_parameter("WhichGameplaySong", "Hurricane")
		
	else:
		MusicManager.gameplay.set_parameter("WhichGameplaySong", "Normal")
		
	MusicManager.gameplay.set_parameter("MuffledOrNot", 1.0)

	pause_ui.visible = false
	resume_button.disabled = true
	pause_settings_button.disabled = true
	pause_main_menu_button.disabled = true
	is_paused = false
	

func _on_settings_pressed() -> void:
	SFXManager.click.play()
	pause_label.visible = false
	resume_button.visible = false
	pause_settings_button.visible = false
	pause_main_menu_button.visible = false

	var buttons = get_tree().get_nodes_in_group("button")
	for button in buttons:
		button.disabled = true

	get_tree().get_root().add_child(settings_scene.instantiate())


func _on_try_again_pressed() -> void:
	SFXManager.click.play()
	get_tree().change_scene_to_packed(game_scene)


func _on_main_menu_pressed() -> void:
	MusicManager.gameplay.stop()
	SFXManager.click.play()
	get_tree().change_scene_to_packed(main_menu_scene)


func _on_alive_to_dead_music_timer_timeout() -> void:
	MusicManager.gameplay.set_parameter("WhichGameplaySong", "GameOver")
