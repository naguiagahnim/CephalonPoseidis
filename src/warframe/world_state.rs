use super::api::WarframeApi;

pub struct WorldState;

impl WorldState {
    pub async fn update() -> {
        let world_state = WarframeApi::get_world_state().await.unwrap();
        world_state
    }

    pub async fn 
}