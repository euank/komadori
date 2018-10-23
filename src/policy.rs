use types::{User, CreateGroupRequest};
use uuid::Uuid;
use permissions;

pub enum Action<'a> {
    CreateGroup(&'a CreateGroupRequest),
    ListGroups(()),
}

fn user_in_group(user: User, group: Uuid) -> bool {
    for g in user.groups {
        if g.uuid == group.simple().to_string() {
            return true
        }
    }
    return false
}

pub fn is_allowed(user: User, action: Action) -> bool {
    match action {
        Action::CreateGroup(_) => {
            return user_in_group(user, *permissions::ADMIN_GROUP)
        }
        Action::ListGroups(_) => {
            return user_in_group(user, *permissions::ADMIN_GROUP)
        }
    }
}
