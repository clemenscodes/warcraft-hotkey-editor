use warcraft_api::{
    ArmorFigure as Armor, AttackRange, AttackSpeed, AttributeStatistic, DamagePerSecond,
    DamageRange, EffectiveHitPoints, Evasion, Gain, HitPoints, HitPointsRegen, Mana, ManaRegen,
};
use warcraft_api::{AttackType, DefenseType};

pub trait StatFigure: Copy + PartialEq + 'static {
    fn display(&self) -> String;

    fn is_muted(&self) -> bool;
}

impl StatFigure for HitPoints {
    fn display(&self) -> String {
        let value = self.value();
        value.to_string()
    }

    fn is_muted(&self) -> bool {
        false
    }
}

impl StatFigure for Mana {
    fn display(&self) -> String {
        let value = self.value();
        value.to_string()
    }

    fn is_muted(&self) -> bool {
        let mana = *self;
        mana.is_zero()
    }
}

impl StatFigure for Armor {
    fn display(&self) -> String {
        let value = self.value();
        format!("{value:.0}")
    }

    fn is_muted(&self) -> bool {
        false
    }
}

impl StatFigure for EffectiveHitPoints {
    fn display(&self) -> String {
        let value = self.value();
        format!("{value:.0}")
    }

    fn is_muted(&self) -> bool {
        false
    }
}

impl StatFigure for Evasion {
    fn display(&self) -> String {
        let percent = self.percent();
        format!("{percent:.0}%")
    }

    fn is_muted(&self) -> bool {
        false
    }
}

impl StatFigure for Gain {
    fn display(&self) -> String {
        let value = self.value();
        format!("+{value:.1}")
    }

    fn is_muted(&self) -> bool {
        let gain = *self;
        gain.is_zero()
    }
}

impl StatFigure for HitPointsRegen {
    fn display(&self) -> String {
        let value = self.value();
        format!("+{value:.2}")
    }

    fn is_muted(&self) -> bool {
        let regen = *self;
        regen.is_zero()
    }
}

impl StatFigure for ManaRegen {
    fn display(&self) -> String {
        let value = self.value();
        format!("+{value:.2}")
    }

    fn is_muted(&self) -> bool {
        let regen = *self;
        regen.is_zero()
    }
}

impl StatFigure for AttributeStatistic {
    fn display(&self) -> String {
        let value = self.value();
        value.to_string()
    }

    fn is_muted(&self) -> bool {
        false
    }
}

impl StatFigure for DamageRange {
    fn display(&self) -> String {
        let minimum = self.minimum();
        let maximum = self.maximum();
        format!("{minimum}\u{2013}{maximum}")
    }

    fn is_muted(&self) -> bool {
        false
    }
}

impl StatFigure for AttackRange {
    fn display(&self) -> String {
        let value = self.value();
        value.to_string()
    }

    fn is_muted(&self) -> bool {
        false
    }
}

impl StatFigure for AttackSpeed {
    fn display(&self) -> String {
        let cooldown_seconds = self.cooldown_seconds();
        format!("{cooldown_seconds:.2}s")
    }

    fn is_muted(&self) -> bool {
        false
    }
}

impl StatFigure for DamagePerSecond {
    fn display(&self) -> String {
        let value = self.value();
        format!("{value:.1}")
    }

    fn is_muted(&self) -> bool {
        false
    }
}

impl StatFigure for DefenseType {
    fn display(&self) -> String {
        self.to_string()
    }

    fn is_muted(&self) -> bool {
        false
    }
}

impl StatFigure for AttackType {
    fn display(&self) -> String {
        self.to_string()
    }

    fn is_muted(&self) -> bool {
        false
    }
}
