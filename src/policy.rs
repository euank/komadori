use types::{User, CreateGroupRequest, UpdateUserRequest};
use uuid::Uuid;
use permissions;

pub enum Action<'a> {
    CreateGroup(&'a CreateGroupRequest),
    ListGroups(()),
    UpdateUser(&'a UpdateUserRequest),
    GetUser(&'a Uuid),
}

fn user_in_group(user: &User, group: Uuid) -> bool {
    for g in &user.groups {
        if g.uuid == group.simple().to_string() {
            return true
        }
    }
    return false
}

pub fn is_allowed(user: User, action: Action) -> bool {
    match action {
        Action::CreateGroup(_) => {
            return user_in_group(&user, *permissions::ADMIN_GROUP)
        }
        Action::ListGroups(_) => {
            return user_in_group(&user, *permissions::ADMIN_GROUP)
        }
        Action::UpdateUser(req) => {
            return user_update_allowed(user, req)
        }
        Action::GetUser(target) => {
            let user_uuid = match Uuid::parse_str(&user.uuid) {
                Ok(u) => u,
                Err(e) => {
                    error!("malformed user: {}: {}", user.uuid, e);
                    return false;
                }
            };
            // admins can list anyone, a user can list themselves
            return user_in_group(&user, *permissions::ADMIN_GROUP) ||
                &user_uuid == target

        }
    }
}

fn user_update_allowed(user: User, req: &UpdateUserRequest) -> bool {
    if user_in_group(&user, *permissions::ADMIN_GROUP) {
        // admins can make any update to any user
        return true;
    }

    // A user may only update themselves
    if user.uuid != req.user_uuid {
        return false;
    }

    return req.changes_only_user_controlled_fields();
}
