extends Node2D

export(float) var speed = 0
export(int) var dir = -1
export(String) var path = "res://PlayerProjectile.tscn"
var bulletObj
var time = 0
var count = 0

func _ready():
	bulletObj = load("res://PlayerProjectile.tscn")

func _process(delta):
	time += delta

	if time > speed:
		time = 0
		var bullet = bulletObj.instance()
	
		bullet.set_name(String(count))
		bullet.global_position = self.get_global_position()
		
		bullet.set_linear_velocity(Vector2(0, dir)*500)
		bullet.set_gravity_scale(0)
		get_node("/root/Main").add_child(bullet)
		count += 1
		



