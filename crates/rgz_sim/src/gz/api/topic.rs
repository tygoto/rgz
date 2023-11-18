use rgz_msgs as msgs;

#[derive(Debug, Clone)]
pub(crate) enum GzTopic {
    Clock(Option<msgs::Clock>),
    GazeboResourcePaths(Option<msgs::StringMsgV>),
    GuiCameraPose(Option<msgs::Pose>),
    Stats(Option<msgs::WorldStatistics>),
    WorldClock(String, Option<msgs::Clock>),
    WorldDynamicPoseInfo(String, Option<msgs::PoseV>),
    WorldPoseInfo(String, Option<msgs::PoseV>),
    WorldSceneDeletion(String, Option<msgs::UInt32V>),
    WorldSceneInfo(String, Option<msgs::Scene>),
    WorldState(String, Option<msgs::SerializedStepMap>),
    WorldStats(String, Option<msgs::WorldStatistics>),
    Unknown,
}

impl GzTopic {
    pub fn name(&self) -> String {
        match self {
            Self::Clock (..) => "/clock".to_string(),
            Self::GazeboResourcePaths (..) => "/gazebo/resource_paths".to_string(),
            Self::GuiCameraPose (..) => "/gui/camera/pose".to_string(),
            Self::Stats (..) => "/stats".to_string(),
            Self::WorldClock (world_name, ..) => format!("/world/{}/clock", world_name),
            Self::WorldDynamicPoseInfo (world_name, ..) => format!("/world/{}/dynamic_pose/info", world_name),
            Self::WorldPoseInfo (world_name, ..) => format!("/world/{}/pose/info", world_name),
            Self::WorldSceneDeletion (world_name, ..) => format!("/world/{}/scene/deletion", world_name),
            Self::WorldSceneInfo (world_name, ..) => format!("/world/{}/scene/info", world_name),
            Self::WorldState (world_name, ..) => format!("/world/{}/state", world_name),
            Self::WorldStats (world_name, ..) => format!("/world/{}/stats", world_name),
            _ => "unknown".to_string(),
        }
    }

    // pub fn from_str_name(value: &str) -> Option<Self> {
    //     let re = Regex::new(r"^/world/([^/]+)/(.+)$").unwrap();
    //
    //     if let Some(caps) = re.captures(value) {
    //         let world_name = caps.get(1).unwrap().as_str().to_string();
    //         let topic = caps.get(2).unwrap().as_str().to_string();
    //
    //         match topic.as_str() {
    //             "clock" => Some(Self::Clock(Some(world_name))),
    //             "dynamic_pose/info" => Some(Self::DynamicPoseInfo(world_name)),
    //             "pose/info" => Some(Self::PoseInfo(world_name)),
    //             "scene/deletion" => Some(Self::SceneDeletion(world_name)),
    //             "scene/info" => Some(Self::SceneInfo(world_name)),
    //             "state" => Some(Self::State(world_name)),
    //             "stats" => Some(Self::Stats(Some(world_name))),
    //             "light_config" => Some(Self::LightConfig(world_name)),
    //             _ => None,
    //         }
    //     } else {
    //         match value {
    //             "/clock" => Some(Self::Clock(None)),
    //             "/gazebo/resource_paths" => Some(Self::GazeboResourcePaths),
    //             "/gui/camera/pose" => Some(Self::GuiCameraPose),
    //             "/stats" => Some(Self::Stats(None)),
    //             _ => None,
    //         }
    //     }
    // }
}
