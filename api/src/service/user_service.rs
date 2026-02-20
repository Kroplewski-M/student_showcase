use crate::{config::Config, db::user_repo::UserRepo};

#[derive(Clone)]
pub struct UserService {
    user_repo: UserRepo,
    config: Config,
}

impl UserService {
    pub fn new(user_repo: UserRepo, config: Config) -> Self {
        Self { user_repo, config }
    }
}
