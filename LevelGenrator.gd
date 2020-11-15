extends Node

var levels
var rng = RandomNumberGenerator.new()
var cur_scene
func _ready():
	levels = load("res://Enemies.tscn")
	rng.randomize()
	generate()
	

func _process(delta):
	if cur_scene != null:
		if cur_scene.get_child_count() == 0:
			cur_scene.queue_free()
			generate()

func generate():
	var count = levels.instance().get_child_count()
	var scene_id = rng.randi_range(0, count-1)
	var scene = levels.instance().get_child(scene_id).duplicate(15)
	scene.set_position(Vector2(0, 0))
	var score = get_node("/root/Main/score")
	for c in scene.get_children():
		c.connect("damage_taken", score, "damage_taken")
	add_child(scene)
	cur_scene = scene
