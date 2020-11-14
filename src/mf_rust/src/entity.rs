use gdnative::prelude::*;
use uuid::Uuid;

#[derive(NativeClass)]
#[inherit(Node2D)]
#[register_with(Self::register_signals)]
pub struct Entity { 
    id: Uuid,
    health: i64
}

#[methods]
impl Entity {
    fn new(_owner: &Node2D) -> Self {
        Entity { health: 0, id: Uuid::new_v4() }
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
                    name: "delta",
                    default: Variant::from_i64(0),
                    export_info: ExportInfo::new(VariantType::I64),
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
    fn take_damage(&mut self, owner: &Node2D, damage: i64) {
        if self.health - damage <= 0 {
            owner.emit_signal("killed", &[Variant::from_str(self.id.to_hyphenated().to_string())]);
            self.health = 0;
        } else {
            self.health -= damage;
            owner.emit_signal("damage_taken", &[
                Variant::from_str(self.id.to_hyphenated().to_string()), 
                Variant::from_i64(damage), 
                Variant::from_i64(self.health)
            ]);
        }
    }
}