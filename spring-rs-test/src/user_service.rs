use spring::plugin::service::Service;

#[derive(Clone, Service)]
pub struct UserService;

impl UserService {
    pub async fn stuff_to_do(self) -> &'static str {
        return "We're doing stuff here.";
    }
}