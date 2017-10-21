#[derive(PartialEq, Debug, Copy, Clone)]
pub enum BaseElement {
    Fire, Air, Water, Earth,
}

#[derive(PartialEq, PartialOrd, Debug, Copy, Clone)]
pub enum BaseMetal {
    Lead, Tin, Iron, Copper, Silver, Gold,
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Januae {
    Vitae, Mors,
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Atom {
    BaseElement(BaseElement),
    BaseMetal(BaseMetal),
    Januae(Januae),
    Quicksilver,
    Salt,
}

impl Atom {
    pub fn is_match(a1: &Atom, a2: &Atom, active_metal: &Option<BaseMetal>) -> bool {
        use self::Atom::*;
        use self::BaseMetal;
        match (a1, a2) {
            (&BaseElement(ref e1), &BaseElement(ref e2)) => e1 == e2,
            (&Salt, &Salt) => true,
            (&BaseElement(_), &Salt) => true,
            (&Salt, &BaseElement(_)) => true,
            (&Quicksilver, &BaseMetal(ref m)) if *m != BaseMetal::Gold => active_metal.into_iter().any(|am| m <= am),
            (&BaseMetal(ref m), &Quicksilver) if *m != BaseMetal::Gold => active_metal.into_iter().any(|am| m <= am),
            (&Januae(ref j1), &Januae(ref j2)) => j1 != j2,
            _ => false,
        }
    }
// Lead, Tin, Iron, Copper, Silver, Gold,
    pub fn parse(c: char) -> Option<Option<Self>> {
        use self::Atom::*;
        use self::BaseElement::*;
        use self::Januae::*;
        use self::BaseMetal::*;
        
        match c {
            'f' => Some(Some(BaseElement(Fire))),
            'a' => Some(Some(BaseElement(Air))),
            'w' => Some(Some(BaseElement(Water))),
            'e' => Some(Some(BaseElement(Earth))),
            's' => Some(Some(Salt)),
            'q' => Some(Some(Quicksilver)),
            'l' => Some(Some(BaseMetal(Lead))),
            't' => Some(Some(BaseMetal(Tin))),
            'i' => Some(Some(BaseMetal(Iron))),
            'c' => Some(Some(BaseMetal(Copper))),
            'r' => Some(Some(BaseMetal(Silver))),
            'g' => Some(Some(BaseMetal(Gold))),
            '^' => Some(Some(Januae(Vitae))), 
            'v' => Some(Some(Januae(Mors))),
            '.' => Some(None),
            _ => None,
        }

    }
    
}

#[cfg(test)]
mod tests {
    use super::BaseMetal;
    use super::BaseMetal::*;
    use super::BaseElement::*;
    use super::Januae::*;
    use super::Atom;
    use super::Atom::*;

    const ALL_AVAILABLE_METALS: [Option<BaseMetal>; 7] = [Some(Lead), Some(Tin), Some(Iron), Some(Copper), Some(Silver), Some(Gold), None,];
    const ALL_BASE_ELEMENTS: [Atom; 4] = [BaseElement(Fire), BaseElement(Air), BaseElement(Water), BaseElement(Earth),];
    const ALL_JANUAE: [Atom; 2] = [Januae(Vitae), Januae(Mors),];
    
    #[test]
    fn base_elem_match_correctly() {
        for m in &ALL_AVAILABLE_METALS {
            for e1 in &ALL_BASE_ELEMENTS {
                for e2 in &ALL_BASE_ELEMENTS {
                    assert_eq!(Atom::is_match(e1, e2, m), e1 == e2);
                    assert_eq!(Atom::is_match(e2, e1, m), e1 == e2);
                }
            }
        }
    }

    #[test]
    fn salt_match_element() {
        for m in &ALL_AVAILABLE_METALS {
            for e in &ALL_BASE_ELEMENTS {
                assert!(Atom::is_match(e, &Salt, m));
                assert!(Atom::is_match(&Salt, e, m));
            }
        }
    }

    #[test]
    fn januae_match() {
        for m in &ALL_AVAILABLE_METALS {
            for j1 in &ALL_JANUAE {
                for j2 in &ALL_JANUAE {
                    assert_eq!(Atom::is_match(j1, j2, m), j1 != j2);
                    assert_eq!(Atom::is_match(j2, j1, m), j1 != j2);
                }
            }
        }
    }

    #[test]
    fn metal_match() {
        unimplemented!(); // TODO
    }

    #[test]
    fn others_rejected() {
        for m in &ALL_AVAILABLE_METALS {
            assert!(!Atom::is_match(&Salt, &BaseMetal(Gold), m));
            assert!(!Atom::is_match(&Salt, &BaseMetal(Silver), m));
            assert!(!Atom::is_match(&Salt, &BaseMetal(Lead), m));
            assert!(!Atom::is_match(&BaseElement(Fire), &BaseMetal(Lead), m));
            assert!(!Atom::is_match(&BaseElement(Fire), &Quicksilver, m));
            assert!(!Atom::is_match(&BaseMetal(Gold), &Quicksilver, m));
            assert!(!Atom::is_match(&Quicksilver, &Quicksilver, m));
            assert!(!Atom::is_match(&Januae(Vitae), &Quicksilver, m));
            assert!(!Atom::is_match(&Januae(Vitae), &Salt, m));
            assert!(!Atom::is_match(&Januae(Vitae), &BaseMetal(Lead), m));
        }
    }
}
