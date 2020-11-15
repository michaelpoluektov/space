extends Control


var time = 0
export(int) var speed = 0

func _ready():
	pass # Replace with function body.

func _process(delta):
	time += delta*speed
	material.set_shader_param("frame", time)
