use gdnative::prelude::*;
use gdnative::api::Area2D;
use gdnative::api::RigidBody2D;
use uuid::Uuid;

use crate::projectile::Projectile;

#[derive(NativeClass)]
#[inherit(Node2D)]
#[register_with(Self::register_signals)]
pub struct Entity { 
    pub id: Uuid,
    #[property]
    pub health: i64,
    #[property]
    pub collider: NodePath,
    #[property]
    pub group: i64,
    pub max_health: i64
}

#[methods]
impl Entity {
    fn new(_owner: &Node2D) -> Self {
        Entity { health: 0, id: Uuid::new_v4(), collider: NodePath::from_str(""), group: 0, max_health: 0 }
    }

    fn register_signals(builder: &ClassBuilder<Self>) {
        builder.add_signal(Signal {
            name: "damage_taken",
            args: &[
                SignalArgument {
                    name: "id",
                    default: Variant::from_str(""),
                    export_info: ExportInfo::new(VariantType::GodotString),
                    usage: PropertyUsage::DEFAULT,
                },
                SignalArgument {
                    name: "damage",
                    default: Variant::from_i64(0),
                    export_info: ExportInfo::new(VariantType::I64),
                    usage: PropertyUsage::DEFAULT,
                },
                SignalArgument {
                    name: "health",
                    default: Variant::from_i64(0),
                    export_info: ExportInfo::new(VariantType::I64),
                    usage: PropertyUsage::DEFAULT,
                },
                SignalArgument {
                    name: "percents",
                    default: Variant::from_f64(0.0),
                    export_info: ExportInfo::new(VariantType::F64),
                    usage: PropertyUsage::DEFAULT,
                },
            ],
        });

        builder.add_signal(Signal {
            name: "killed",
            args: &[SignalArgument {
                name: "id",
                default: Variant::from_str(""),
                export_info: ExportInfo::new(VariantType::GodotString),
                usage: PropertyUsage::DEFAULT,
            }],
        });
    }

    #[export]
    fn _ready(&mut self, owner: TRef<Node2D>) {
        let collider = &mut owner.get_node(self.collider.to_godot_string()).unwrap();
        let collider = unsafe { collider.assume_safe() };

        collider
            .connect("area_entered", owner, "area_entered", VariantArray::new_shared(), 0)
            .unwrap();

        self.max_health = self.health;

        self.emit_damage_taken(&owner, 0);
    }

    #[export]
    fn area_entered(&mut self, owner: &Node2D, area: Variant) {
        if let Some(area) = area.try_to_object::<Area2D>() {
            if let Some(parent) = unsafe { area.assume_safe() }.get_parent() {
                if let Ok(parent) = unsafe { parent.assume_unique() }.try_cast::<RigidBody2D>() {
                    if let Ok(proj) = unsafe { parent.assume_unique() }.try_cast_instance::<Projectile>() {
                        proj.map(|p: &Projectile, o: TRef<RigidBody2D, Unique>| {
                            if self.id != p.sender && self.group != p.group {
                                self.take_damage(owner, p.damage);
                                (*o).queue_free();
                            }
                        })
                        .unwrap_or_else(|_| godot_print!("Unable to get projectile"));
                    }
                }
            }
        }

    }

    #[export]
    fn take_damage(&mut self, owner: &Node2D, damage: i64) {
        if self.health - damage <= 0 {
            owner.emit_signal("killed", &[Variant::from_str(self.id.to_hyphenated().to_string())]);
            self.health = 0;
            owner.queue_free();
        } else {
            self.health -= damage;
            self.emit_damage_taken(owner, damage);
        }
    }

    #[export]
    fn emit_damage_taken(&self, owner: &Node2D, damage: i64) {
        owner.emit_signal("damage_taken", &[
                Variant::from_str(self.id.to_hyphenated().to_string()), 
                Variant::from_i64(damage), 
                Variant::from_i64(self.health),
                Variant::from_f64(self.health as f64 / self.max_health as f64)
            ]);
    }
}