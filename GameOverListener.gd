extends Node

func _ready():
	get_node("/root/Main/Player/Area2D").connect("area_entered", self, "game_over")

func game_over(area):
	if area.get_parent().get_script() != null:
		print(area.get_parent().get("group") )
		if area.get_parent().get_script().get_path() != "res://Projectile.gdns" || area.get_parent().get("group") != 0:
			var score = get_node("/root/Main/score").get("score")
			var root = get_node("/root")
			var level = root.get_node("Main")
			root.remove_child(level)
			level.call_deferred("free")

			# Add the next level
			var next_level_resource = load("res://GameOver.tscn")
			var next_level = next_level_resource.instance()
			
			root.add_child(next_level)
			next_level.get_node("/root/GameOver/score").set_text("Score: "+String(score))
