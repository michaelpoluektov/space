extends Node

export(float) var speed = 1

func _ready():
	pass # Replace with function body.

func _process(delta):
	get_parent().position.y += speed
