extends Button

func _ready():
	connect("pressed", self, "pressed")

func pressed():
	var root = get_node("/root")
	var level = root.get_node("GameOver")
	root.remove_child(level)
	level.call_deferred("free")

	# Add the next level
	var next_level_resource = load("res://Main.tscn")
	var next_level = next_level_resource.instance()
	root.add_child(next_level)
