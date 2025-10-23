extends Control

@export var scene_root_node: Control
@onready var main_menu_node: Control = get_node("/root").get_node_or_null("MainMenu")

func _process(_delta) -> void:
	if Input.is_action_just_pressed("esc"):
		dissapear_node(scene_root_node)

		if main_menu_node != null:
			appear_node(main_menu_node)
				
		queue_free()


func _on_return_pressed() -> void:
	SFXManager.click.play()

	dissapear_node(scene_root_node)

	if main_menu_node != null:
		appear_node(main_menu_node)
			
	queue_free()
	
	
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
