extends Label

var score = 0
var player: NodePath

func damage_taken(id, damage, health, progress):
	score += damage
	set_text(String(score))
