extends Node2D

export var speed = 400 
export var border_offset = 30 # How fast the player will move (pixels/sec).
var screen_size  # Size of the game window.
var particles

# Called when the node enters the scene tree for the first time.
func _ready():
	screen_size = get_viewport_rect().size
	for child in get_parent().get_children():
		if child is AnimatedSprite:
			child.play()
	particles = get_node("../engine_part")
	print(particles)

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	var velocity = Vector2()  # The player's movement vector.
	if Input.is_action_pressed("ui_right"):
		velocity.x += 1
		particles.set_param(8, 2)
	if Input.is_action_pressed("ui_left"):
		velocity.x -= 1
		particles.set_param(8, 2)
	if Input.is_action_pressed("ui_down"):
		velocity.y += 1
		particles.set_param(8, 1)
	if Input.is_action_pressed("ui_up"):
		velocity.y -= 1
		particles.set_param(8, 4)
	if velocity.length() > 0:
		velocity = velocity.normalized() * speed
		
	if velocity.length() == 0:
		particles.set_param(8, 4)
#	else:
#		$AnimatedSprite.stop()
	var player = get_parent()
	player.position += velocity * delta
	player.position.x = clamp(player.position.x, border_offset, screen_size.x-border_offset)
	player.position.y = clamp(player.position.y, border_offset, screen_size.y-border_offset)

