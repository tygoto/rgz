use rgz_msgs::*;

#[derive(Debug)]
pub(crate) enum GzService {
    GazeboResourcePathsAdd {
        request: StringMsgV,
        response: Option<Empty>
    },
    GazeboResourcePathsGet {
        request: Empty,
        response: Option<StringMsgV>,
    },
    GazeboResourcePathsResolve {
        request: Empty,
        response: Option<StringMsg>,
    },
    GazeboWorlds {
        request: Empty,
        response: Option<StringMsgV>,
    },
    GuiCameraViewControl {
        request: StringMsg,
        response: Option<Boolean>,
    },
    GuiCameraViewControlReferenceVisual {
        request: Boolean,
        response: Option<Boolean>,
    },
    GuiCameraViewControlSensitivity {
        request: Double,
        response: Option<Boolean>,
    },
    GuiCopy {
        request: StringMsg,
        response: Option<Boolean>,
    },
    GuiFollow {
        request: StringMsg,
        response: Option<Boolean>,
    },
    GuiFollowOffset {
        request: Vector3d,
        response: Option<Boolean>,
    },
    GuiMoveTo {
        request: StringMsg,
        response: Option<Boolean>,
    },
    GuiMoveToPose {
        request: GuiCamera,
        response: Option<Boolean>,
    },
    GuiPaste {
        request: Empty,
        response: Option<Boolean>,
    },
    GuiScreenshot {
        request: StringMsg,
        response: Option<Boolean>,
    },
    GuiViewCollisions {
        request: StringMsg,
        response: Option<Boolean>,
    },
    GuiViewCom {
        request: StringMsg,
        response: Option<Boolean>,
    },
    GuiViewFrames {
        request: StringMsg,
        response: Option<Boolean>,
    },
    GuiViewInertia {
        request: StringMsg,
        response: Option<Boolean>,
    },
    GuiViewJoints {
        request: StringMsg,
        response: Option<Boolean>,
    },
    GuiViewTransparent {
        request: StringMsg,
        response: Option<Boolean>,
    },
    GuiViewWireframes {
        request: StringMsg,
        response: Option<Boolean>,
    },
    Marker {
        request: Marker,
        response: Option<Empty>,
    },
    MarkerList {
        request: Empty,
        response: Option<MarkerV>,
    },
    MarkerArray {
        request: MarkerV,
        response: Option<Boolean>,
    },
    ServerControl {
        request: ServerControl,
        response: Option<Boolean>,
    },
    WorldControl {
        world_name: String,
        request: WorldControl,
        response: Option<Boolean>,
    },
    WorldControlState {
        world_name: String,
        request: WorldControlState,
        response: Option<Boolean>,
    },
    WorldCreate {
        world_name: String,
        request: EntityFactory,
        response: Option<Boolean>,
    },
    WorldCreateMultiple {
        world_name: String,
        request: EntityFactoryV,
        response: Option<Boolean>,
    },
    WorldDeclareParameter {
        world_name: String,
        request: Parameter,
        response: Option<ParameterError>,
    },
    WorldDisableCollision {
        world_name: String,
        request: Entity,
        response: Option<Boolean>,
    },
    WorldEnableCollision {
        world_name: String,
        request: Entity,
        response: Option<Boolean>,
    },
    WorldEntitySystemAdd {
        world_name: String,
        request: EntityPluginV,
        response: Option<Boolean>,
    },
    WorldGenerateWorldSdf {
        world_name: String,
        request: SdfGeneratorConfig,
        response: Option<StringMsg>,
    },
    WorldGetParameter {
        world_name: String,
        request: ParameterName,
        response: Option<ParameterValue>,
    },
    WorldGuiInfo {
        world_name: String,
        request: Empty,
        response: Option<Gui>,
    },
    WorldLevelSetPerformer {
        world_name: String,
        request: StringMsg,
        response: Option<Boolean>,
    },
    WorldLightConfig {
        world_name: String,
        request: Light,
        response: Option<Boolean>,
    },
    WorldListParameters {
        world_name: String,
        request: Empty,
        response: Option<ParameterDeclarations>,
    },
    WorldPlaybackControl {
        world_name: String,
        request: LogPlaybackControl,
        response: Option<Boolean>,
    },
    WorldRemove {
        world_name: String,
        request: Entity,
        response: Option<Boolean>,
    },
    WorldSceneGraph {
        world_name: String,
        request: Empty,
        response: Option<StringMsg>,
    },
    WorldSceneInfo {
        world_name: String,
        request: Empty,
        response: Option<Scene>,
    },
    WorldSetParameter {
        world_name: String,
        request: Parameter,
        response: Option<ParameterError>,
    },
    WorldSetPhysics {
        world_name: String,
        request: Physics,
        response: Option<Boolean>,
    },
    WorldSetPose {
        world_name: String,
        request: Pose,
        response: Option<Boolean>,
    },
    WorldSetPoseVector {
        world_name: String,
        request: PoseV,
        response: Option<Boolean>,
    },
    WorldSetSphericalCoordinates {
        world_name: String,
        request: SphericalCoordinates,
        response: Option<Boolean>,
    },
    WorldState {
        world_name: String,
        request: Empty,
        response: Option<SerializedStepMap>,
    },
    WorldStateAsync {
        world_name: String,
        request: StringMsg,
        response: Option<Empty>,
    },
    WorldSystemInfo {
        world_name: String,
        request: Empty,
        response: Option<EntityPluginV>,
    },
    WorldVisualConfig {
        world_name: String,
        request: Visual,
        response: Option<Boolean>,
    },
    WorldWheelSlip {
        world_name: String,
        request: WheelSlipParametersCmd,
        response: Option<Boolean>,
    },
    Unknown,
}

impl GzService {
    pub fn name(&self) -> String {
        match self {
            Self::GazeboResourcePathsAdd {..} => "/gazebo/resource_paths/add".to_string(),
            Self::GazeboResourcePathsGet {..} => "/gazebo/resource_paths/get".to_string(),
            Self::GazeboResourcePathsResolve {..} => "/gazebo/resource_paths/resolve".to_string(),
            Self::GazeboWorlds {..} => "/gazebo/worlds".to_string(),
            Self::GuiCameraViewControl {..} => "/gui/camera/view_control".to_string(),
            Self::GuiCameraViewControlReferenceVisual {..} => "/gui/camera/view_control/reference_visual".to_string(),
            Self::GuiCameraViewControlSensitivity {..} => "/gui/camera/view_control/sensitivity".to_string(),
            Self::GuiCopy {..} => "/gui/copy".to_string(),
            Self::GuiFollow {..} => "/gui/follow".to_string(),
            Self::GuiFollowOffset {..} => "/gui/follow/offset".to_string(),
            Self::GuiMoveTo {..} => "/gui/move_to".to_string(),
            Self::GuiMoveToPose {..} => "/gui/move_to/pose".to_string(),
            Self::GuiPaste {..} => "/gui/paste".to_string(),
            Self::GuiScreenshot {..} => "/gui/screenshot".to_string(),
            Self::GuiViewCollisions {..} => "/gui/view/collisions".to_string(),
            Self::GuiViewCom {..} => "/gui/view/com".to_string(),
            Self::GuiViewFrames {..} => "/gui/view/frames".to_string(),
            Self::GuiViewInertia {..} => "/gui/view/inertia".to_string(),
            Self::GuiViewJoints {..} => "/gui/view/joints".to_string(),
            Self::GuiViewTransparent {..} => "/gui/view/transparent".to_string(),
            Self::GuiViewWireframes {..} => "/gui/view/wireframes".to_string(),
            Self::Marker {..} => "/marker".to_string(),
            Self::MarkerList {..} => "/marker/list".to_string(),
            Self::MarkerArray {..} => "/marker_array".to_string(),
            Self::ServerControl {..} => "/server_control".to_string(),
            Self::WorldControl { world_name, .. } => format!("/world/{}/control", world_name),
            Self::WorldControlState { world_name, .. } => format!("/world/{}/control/state", world_name),
            Self::WorldCreate { world_name, .. } => format!("/world/{}/create", world_name),
            Self::WorldCreateMultiple { world_name, .. } => format!("/world/{}/create_multiple", world_name),
            Self::WorldDeclareParameter { world_name, .. } => format!("/world/{}/declare_parameter", world_name),
            Self::WorldDisableCollision { world_name, .. } => format!("/world/{}/disable_collision", world_name),
            Self::WorldEnableCollision { world_name, .. } => format!("/world/{}/enable_collision", world_name),
            Self::WorldEntitySystemAdd { world_name, .. } => format!("/world/{}/entity_system/add", world_name),
            Self::WorldGenerateWorldSdf { world_name, .. } => format!("/world/{}/generate_world_sdf", world_name),
            Self::WorldGetParameter { world_name, .. } => format!("/world/{}/get_parameter", world_name),
            Self::WorldGuiInfo { world_name, .. } => format!("/world/{}/gui/info", world_name),
            Self::WorldLevelSetPerformer { world_name, .. } => format!("/world/{}/level/set_performer", world_name),
            Self::WorldLightConfig { world_name, .. } => format!("/world/{}/light_config", world_name),
            Self::WorldListParameters { world_name, .. } => format!("/world/{}/list_parameters", world_name),
            Self::WorldPlaybackControl { world_name, .. } => format!("/world/{}/playback/control", world_name),
            Self::WorldRemove { world_name, .. } => format!("/world/{}/remove", world_name),
            Self::WorldSceneGraph { world_name, .. } => format!("/world/{}/scene/graph", world_name),
            Self::WorldSceneInfo { world_name, .. } => format!("/world/{}/scene/info", world_name),
            Self::WorldSetParameter { world_name, .. } => format!("/world/{}/set_parameter", world_name),
            Self::WorldSetPhysics { world_name, .. } => format!("/world/{}/set_physics", world_name),
            Self::WorldSetPose { world_name, .. } => format!("/world/{}/set_pose", world_name),
            Self::WorldSetPoseVector { world_name, .. } => format!("/world/{}/set_pose_vector", world_name),
            Self::WorldSetSphericalCoordinates { world_name, .. } => format!("/world/{}/set_spherical_coordinates", world_name),
            Self::WorldState { world_name, .. } => format!("/world/{}/state", world_name),
            Self::WorldStateAsync { world_name, .. } => format!("/world/{}/state_async", world_name),
            Self::WorldSystemInfo { world_name, .. } => format!("/world/{}/system/info", world_name),
            Self::WorldVisualConfig { world_name, .. } => format!("/world/{}/visual_config", world_name),
            Self::WorldWheelSlip { world_name, .. } => format!("/world/{}/wheel_slip", world_name),
            _ => "unknown".to_string(),
        }
    }
}


