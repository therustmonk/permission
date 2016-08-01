
pub trait HasPermission<T> {
    fn has_permission(&self, permission: &T) -> bool;
}

#[cfg(test)]
mod tests {
    use super::*;

    enum OnePermission {
        CanDoIt,
        CanDoOther,
    }

    #[allow(dead_code)]
    struct SecondPermission {
        can_do_second: bool,
        can_do_even_more: bool,
    }

    impl SecondPermission {
        fn new(second: bool, even_more: bool) -> Self {
            SecondPermission {
                can_do_second: second,
                can_do_even_more: even_more,
            }
        }
    }

    struct User { }

    impl User {
        fn new() -> Self {
            User { }
        }
    }

    impl HasPermission<OnePermission> for User {
        fn has_permission(&self, permission: &OnePermission) -> bool {
            match *permission {
                OnePermission::CanDoIt => true,
                OnePermission::CanDoOther => false,
            }
        }
    }

    impl HasPermission<SecondPermission> for User {
        fn has_permission(&self, permission: &SecondPermission) -> bool {
            match *permission {
                SecondPermission{ can_do_second: true, .. } => true,
                _ => false,
            }
        }
    }

    #[test]
    fn check_permissions() {
        let user = User::new();
        assert_eq!(user.has_permission(&OnePermission::CanDoIt), true);
        assert_eq!(user.has_permission(&OnePermission::CanDoOther), false);
        assert_eq!(user.has_permission(&SecondPermission::new(true, false)), true);
        assert_eq!(user.has_permission(&SecondPermission::new(true, true)), true);
        assert_eq!(user.has_permission(&SecondPermission::new(false, true)), false);
        assert_eq!(user.has_permission(&SecondPermission::new(false, false)), false);
    }

    #[test]
    #[should_panic]
    fn check_wrong_permissions() {
        let user = User::new();
        assert_eq!(user.has_permission(&OnePermission::CanDoOther), true);
    }
}
