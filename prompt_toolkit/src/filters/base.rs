#![allow(dead_code)]

pub enum Filter {
    Never,
    Always,
    Condition { func: Box<dyn Fn() -> bool> },
}

impl Filter {
    pub fn eval(&self) -> bool {
        match self {
            Filter::Never => false,
            Filter::Always => true,
            Filter::Condition { func } => func(),
        }
    }

    pub fn and(self, other: Filter) -> Filter {
        match self {
            Filter::Always => other,
            Filter::Never => Filter::Never,
            Filter::Condition { .. } => {
                if let Filter::Always = other {
                    self
                } else if let Filter::Never = other {
                    other
                } else {
                    Filter::Condition {
                        func: Box::new(move || self.eval() && other.eval()),
                    }
                }
            }
        }
    }

    pub fn or(self, other: Filter) -> Filter {
        match self {
            Filter::Always => Filter::Always,
            Filter::Never => other,
            Filter::Condition { .. } => {
                if let Filter::Always = other {
                    other
                } else if let Filter::Never = other {
                    self
                } else {
                    Filter::Condition {
                        func: Box::new(move || self.eval() || other.eval()),
                    }
                }
            }
        }
    }

    pub fn invert(self) -> Filter {
        match self {
            Filter::Always => Filter::Never,
            Filter::Never => Filter::Always,
            Filter::Condition { func: f } => Filter::Condition {
                func: Box::new(move || !f()),
            },
        }
    }

    pub fn condition(f: impl Fn() -> bool + 'static) -> Filter {
        Filter::Condition { func: Box::new(f) }
    }
}

mod test {
    #[cfg(test)]
    use super::*;

    #[test]
    fn test_basic_filters() {
        assert!(!Filter::Never.eval());
        assert!(Filter::Always.eval());
    }

    #[test]
    fn test_condition_filter() {
        let f = Filter::Condition {
            func: Box::new(|| true),
        };
        assert!(f.eval());

        let f = Filter::Condition {
            func: Box::new(|| false),
        };
        assert!(!f.eval());
    }

    #[test]
    fn test_and() {
        assert!(!Filter::Never.and(Filter::Never).eval());
        assert!(!Filter::Never.and(Filter::Always).eval());
        assert!(!Filter::Always.and(Filter::Never).eval());
        assert!(Filter::Always.and(Filter::Always).eval());

        let c = Filter::Condition {
            func: Box::new(|| true),
        };
        assert!(Filter::Always.and(c).eval());
        let c = Filter::Condition {
            func: Box::new(|| true),
        };
        assert!(!Filter::Never.and(c).eval());
    }

    #[test]
    fn test_or() {
        let f = Filter::Never;
        let f2 = Filter::Never;
        assert!(!f.or(f2).eval());

        let t = Filter::Always;
        let f = Filter::Never;
        assert!(f.or(t).eval());

        let t = Filter::Always;
        let f = Filter::Never;
        assert!(t.or(f).eval());

        let t = Filter::Always;
        let t2 = Filter::Always;
        assert!(t.or(t2).eval());

        let t = Filter::Always;
        let c = Filter::Condition {
            func: Box::new(|| true),
        };
        assert!(t.or(c).eval());
        let c = Filter::Condition {
            func: Box::new(|| true),
        };
        let f = Filter::Always;
        assert!(c.or(f).eval());
    }

    #[test]
    fn test_invert() {
        assert!(Filter::Never.invert().eval());
        assert!(!Filter::Always.invert().eval());

        let f = Filter::Condition {
            func: Box::new(|| true),
        };
        assert!(!f.invert().eval());
    }

    #[test]
    fn test_condition() {
        let f = Filter::condition(|| true);
        assert!(f.eval());

        let f = Filter::condition(|| false);
        assert!(!f.eval());
    }
}
