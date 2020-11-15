extends Node2D

export var speed = 400  # How fast the player will move (pixels/sec).
var screen_size  # Size of the game window.


# Called when the node enters the scene tree for the first time.
func _ready():
	screen_size = get_viewport_rect().size
	for child in get_parent().get_children():
		if child is AnimatedSprite:
			child.play()

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	var velocity = Vector2()  # The player's movement vector.
	if Input.is_action_pressed("ui_right"):
		velocity.x += 1
	if Input.is_action_pressed("ui_left"):
		velocity.x -= 1
	if Input.is_action_pressed("ui_down"):
		velocity.y += 1
	if Input.is_action_pressed("ui_up"):
		velocity.y -= 1
	if velocity.length() > 0:
		velocity = velocity.normalized() * speed
#	else:
#		$AnimatedSprite.stop()
	var player = get_parent()
	player.position += velocity * delta
	player.position.x = clamp(player.position.x, 0, screen_size.x)
	player.position.y = clamp(player.position.y, 0, screen_size.y)

