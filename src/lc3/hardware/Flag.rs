
use bitflags::bitflags;

bitflags! {
    #[derive(PartialEq, Debug)]
    pub struct ConditionFlags: u8 {
        const POS = 0b001; // Positive
        const ZRO = 0b010; // Zero
        const NEG = 0b100; // Negative
    }
}

impl ConditionFlags {
    pub fn update_from_value(value: i16) -> Self {
        if value == 0 {
            ConditionFlags::ZRO
        } else if value < 0 {
            ConditionFlags::NEG
        } else {
            ConditionFlags::POS
        }
    }
    
    pub fn is_condition_met(&self, condition: ConditionFlags) -> bool {
        self.contains(condition)
    }

    /// Custom debug string for the active flags.
    pub fn to_debug_string(&self) -> String {
        let mut flags = vec![];
        if self.contains(ConditionFlags::POS) {
            flags.push("POS");
        }
        if self.contains(ConditionFlags::ZRO) {
            flags.push("ZRO");
        }
        if self.contains(ConditionFlags::NEG) {
            flags.push("NEG");
        }
        flags.join(" | ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_from_value() {
        assert_eq!(ConditionFlags::update_from_value(0), ConditionFlags::ZRO);
        assert_eq!(ConditionFlags::update_from_value(10), ConditionFlags::POS);
        assert_eq!(ConditionFlags::update_from_value(-10), ConditionFlags::NEG);
    }

    #[test]
    fn test_is_condition_met() {
        let flags = ConditionFlags::POS | ConditionFlags::ZRO;
        assert!(flags.is_condition_met(ConditionFlags::POS));
        assert!(flags.is_condition_met(ConditionFlags::ZRO));
        assert!(!flags.is_condition_met(ConditionFlags::NEG));
    }

    #[test]
    fn test_to_debug_string() {
        let flags = ConditionFlags::POS | ConditionFlags::NEG;
        assert_eq!(flags.to_debug_string(), "POS | NEG");
    }
}