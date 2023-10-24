#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Time {
    /// / \brief Seconds
    #[prost(int64, tag = "1")]
    pub sec: i64,
    /// / \brief Nanoseconds
    #[prost(int32, tag = "2")]
    pub nsec: i32,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Header {
    #[prost(message, optional, tag = "1")]
    pub stamp: ::core::option::Option<Time>,
    #[prost(message, repeated, tag = "2")]
    pub data: ::prost::alloc::vec::Vec<header::Map>,
}
/// Nested message and enum types in `Header`.
pub mod header {
    #[derive(::rgz_derive::GzMessage)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Map {
        #[prost(string, tag = "1")]
        pub key: ::prost::alloc::string::String,
        #[prost(string, repeated, tag = "2")]
        pub value: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Vector3d {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    #[prost(double, tag = "2")]
    pub x: f64,
    #[prost(double, tag = "3")]
    pub y: f64,
    #[prost(double, tag = "4")]
    pub z: f64,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BoxGeom {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    #[prost(message, optional, tag = "2")]
    pub size: ::core::option::Option<Vector3d>,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CapsuleGeom {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief Radius of the capsule
    #[prost(double, tag = "2")]
    pub radius: f64,
    /// / \brief Height of the cylinder
    #[prost(double, tag = "3")]
    pub length: f64,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConeGeom {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief The base radius of cone in meters
    #[prost(double, tag = "2")]
    pub radius: f64,
    /// / \brief The distance in meters from the base to the apex of the cone
    #[prost(double, tag = "3")]
    pub length: f64,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CylinderGeom {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    #[prost(double, tag = "2")]
    pub radius: f64,
    #[prost(double, tag = "3")]
    pub length: f64,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EllipsoidGeom {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief 3D Vector with the three radius that define a ellipsoid
    #[prost(message, optional, tag = "2")]
    pub radii: ::core::option::Option<Vector3d>,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Image {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief Image width (number of columns)
    #[prost(uint32, tag = "2")]
    pub width: u32,
    /// / \brief Image height (number of rows)
    #[prost(uint32, tag = "3")]
    pub height: u32,
    /// / \brief Full row length in bytes
    #[prost(uint32, tag = "4")]
    pub step: u32,
    /// / \brief Actual data, size if (step * rows)
    #[prost(bytes = "vec", tag = "5")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    /// / \brief Pixel format type.
    #[prost(enumeration = "PixelFormatType", tag = "6")]
    pub pixel_format_type: i32,
}
/// / \brief Possible pixel formats.
/// / This list should match gz::common::Image::PixelFormatType
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PixelFormatType {
    UnknownPixelFormat = 0,
    LInt8 = 1,
    LInt16 = 2,
    RgbInt8 = 3,
    RgbaInt8 = 4,
    BgraInt8 = 5,
    RgbInt16 = 6,
    RgbInt32 = 7,
    BgrInt8 = 8,
    BgrInt16 = 9,
    BgrInt32 = 10,
    RFloat16 = 11,
    RgbFloat16 = 12,
    RFloat32 = 13,
    RgbFloat32 = 14,
    BayerRggb8 = 15,
    BayerBggr8 = 16,
    BayerGbrg8 = 17,
    BayerGrbg8 = 18,
}
impl PixelFormatType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PixelFormatType::UnknownPixelFormat => "UNKNOWN_PIXEL_FORMAT",
            PixelFormatType::LInt8 => "L_INT8",
            PixelFormatType::LInt16 => "L_INT16",
            PixelFormatType::RgbInt8 => "RGB_INT8",
            PixelFormatType::RgbaInt8 => "RGBA_INT8",
            PixelFormatType::BgraInt8 => "BGRA_INT8",
            PixelFormatType::RgbInt16 => "RGB_INT16",
            PixelFormatType::RgbInt32 => "RGB_INT32",
            PixelFormatType::BgrInt8 => "BGR_INT8",
            PixelFormatType::BgrInt16 => "BGR_INT16",
            PixelFormatType::BgrInt32 => "BGR_INT32",
            PixelFormatType::RFloat16 => "R_FLOAT16",
            PixelFormatType::RgbFloat16 => "RGB_FLOAT16",
            PixelFormatType::RFloat32 => "R_FLOAT32",
            PixelFormatType::RgbFloat32 => "RGB_FLOAT32",
            PixelFormatType::BayerRggb8 => "BAYER_RGGB8",
            PixelFormatType::BayerBggr8 => "BAYER_BGGR8",
            PixelFormatType::BayerGbrg8 => "BAYER_GBRG8",
            PixelFormatType::BayerGrbg8 => "BAYER_GRBG8",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNKNOWN_PIXEL_FORMAT" => Some(Self::UnknownPixelFormat),
            "L_INT8" => Some(Self::LInt8),
            "L_INT16" => Some(Self::LInt16),
            "RGB_INT8" => Some(Self::RgbInt8),
            "RGBA_INT8" => Some(Self::RgbaInt8),
            "BGRA_INT8" => Some(Self::BgraInt8),
            "RGB_INT16" => Some(Self::RgbInt16),
            "RGB_INT32" => Some(Self::RgbInt32),
            "BGR_INT8" => Some(Self::BgrInt8),
            "BGR_INT16" => Some(Self::BgrInt16),
            "BGR_INT32" => Some(Self::BgrInt32),
            "R_FLOAT16" => Some(Self::RFloat16),
            "RGB_FLOAT16" => Some(Self::RgbFloat16),
            "R_FLOAT32" => Some(Self::RFloat32),
            "RGB_FLOAT32" => Some(Self::RgbFloat32),
            "BAYER_RGGB8" => Some(Self::BayerRggb8),
            "BAYER_BGGR8" => Some(Self::BayerBggr8),
            "BAYER_GBRG8" => Some(Self::BayerGbrg8),
            "BAYER_GRBG8" => Some(Self::BayerGrbg8),
            _ => None,
        }
    }
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeightmapGeom {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// The height data
    #[prost(message, optional, tag = "2")]
    pub image: ::core::option::Option<Image>,
    /// Size in meters
    #[prost(message, optional, tag = "3")]
    pub size: ::core::option::Option<Vector3d>,
    /// Origin in world coordinate frame
    #[prost(message, optional, tag = "4")]
    pub origin: ::core::option::Option<Vector3d>,
    #[prost(float, repeated, tag = "5")]
    pub heights: ::prost::alloc::vec::Vec<f32>,
    #[prost(int32, tag = "6")]
    pub width: i32,
    #[prost(int32, tag = "7")]
    pub height: i32,
    /// List of textures
    #[prost(message, repeated, tag = "8")]
    pub texture: ::prost::alloc::vec::Vec<heightmap_geom::Texture>,
    /// How to blend the textures
    #[prost(message, repeated, tag = "9")]
    pub blend: ::prost::alloc::vec::Vec<heightmap_geom::Blend>,
    /// Enable terrain paging in rendering
    #[prost(bool, tag = "10")]
    pub use_terrain_paging: bool,
    /// The image filename
    #[prost(string, tag = "11")]
    pub filename: ::prost::alloc::string::String,
    /// Sample level
    #[prost(uint32, tag = "12")]
    pub sampling: u32,
}
/// Nested message and enum types in `HeightmapGeom`.
pub mod heightmap_geom {
    #[derive(::rgz_derive::GzMessage)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Texture {
        #[prost(string, tag = "1")]
        pub diffuse: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub normal: ::prost::alloc::string::String,
        #[prost(double, tag = "3")]
        pub size: f64,
    }
    #[derive(::rgz_derive::GzMessage)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Blend {
        #[prost(double, tag = "1")]
        pub min_height: f64,
        #[prost(double, tag = "2")]
        pub fade_dist: f64,
    }
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageGeom {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
    #[prost(double, tag = "3")]
    pub scale: f64,
    #[prost(int32, tag = "4")]
    pub threshold: i32,
    #[prost(double, tag = "5")]
    pub height: f64,
    #[prost(int32, tag = "6")]
    pub granularity: i32,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MeshGeom {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    #[prost(string, tag = "2")]
    pub filename: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub scale: ::core::option::Option<Vector3d>,
    #[prost(string, tag = "4")]
    pub submesh: ::prost::alloc::string::String,
    #[prost(bool, tag = "5")]
    pub center_submesh: bool,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Vector2d {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    #[prost(double, tag = "2")]
    pub x: f64,
    #[prost(double, tag = "3")]
    pub y: f64,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlaneGeom {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    #[prost(message, optional, tag = "2")]
    pub normal: ::core::option::Option<Vector3d>,
    #[prost(message, optional, tag = "3")]
    pub size: ::core::option::Option<Vector2d>,
    #[prost(double, tag = "4")]
    pub d: f64,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Polyline {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    #[prost(double, tag = "2")]
    pub height: f64,
    #[prost(message, repeated, tag = "3")]
    pub point: ::prost::alloc::vec::Vec<Vector2d>,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SphereGeom {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief Radius of the sphere.
    #[prost(double, tag = "2")]
    pub radius: f64,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Geometry {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    #[prost(enumeration = "geometry::Type", tag = "2")]
    pub r#type: i32,
    #[prost(message, optional, tag = "3")]
    pub r#box: ::core::option::Option<BoxGeom>,
    #[prost(message, optional, tag = "4")]
    pub cylinder: ::core::option::Option<CylinderGeom>,
    #[prost(message, optional, tag = "5")]
    pub plane: ::core::option::Option<PlaneGeom>,
    #[prost(message, optional, tag = "6")]
    pub sphere: ::core::option::Option<SphereGeom>,
    #[prost(message, optional, tag = "7")]
    pub image: ::core::option::Option<ImageGeom>,
    #[prost(message, optional, tag = "8")]
    pub heightmap: ::core::option::Option<HeightmapGeom>,
    #[prost(message, optional, tag = "9")]
    pub mesh: ::core::option::Option<MeshGeom>,
    #[prost(message, optional, tag = "10")]
    pub cone: ::core::option::Option<ConeGeom>,
    #[prost(message, optional, tag = "13")]
    pub capsule: ::core::option::Option<CapsuleGeom>,
    #[prost(message, optional, tag = "14")]
    pub ellipsoid: ::core::option::Option<EllipsoidGeom>,
    #[prost(message, repeated, tag = "11")]
    pub points: ::prost::alloc::vec::Vec<Vector3d>,
    #[prost(message, repeated, tag = "12")]
    pub polyline: ::prost::alloc::vec::Vec<Polyline>,
}
/// Nested message and enum types in `Geometry`.
pub mod geometry {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Type {
        Box = 0,
        Cylinder = 1,
        Sphere = 2,
        Plane = 3,
        Image = 4,
        Heightmap = 5,
        Mesh = 6,
        TriangleFan = 7,
        LineStrip = 8,
        Polyline = 9,
        Cone = 10,
        Empty = 11,
        Arrow = 12,
        Axis = 13,
        Capsule = 14,
        Ellipsoid = 15,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Box => "BOX",
                Type::Cylinder => "CYLINDER",
                Type::Sphere => "SPHERE",
                Type::Plane => "PLANE",
                Type::Image => "IMAGE",
                Type::Heightmap => "HEIGHTMAP",
                Type::Mesh => "MESH",
                Type::TriangleFan => "TRIANGLE_FAN",
                Type::LineStrip => "LINE_STRIP",
                Type::Polyline => "POLYLINE",
                Type::Cone => "CONE",
                Type::Empty => "EMPTY",
                Type::Arrow => "ARROW",
                Type::Axis => "AXIS",
                Type::Capsule => "CAPSULE",
                Type::Ellipsoid => "ELLIPSOID",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "BOX" => Some(Self::Box),
                "CYLINDER" => Some(Self::Cylinder),
                "SPHERE" => Some(Self::Sphere),
                "PLANE" => Some(Self::Plane),
                "IMAGE" => Some(Self::Image),
                "HEIGHTMAP" => Some(Self::Heightmap),
                "MESH" => Some(Self::Mesh),
                "TRIANGLE_FAN" => Some(Self::TriangleFan),
                "LINE_STRIP" => Some(Self::LineStrip),
                "POLYLINE" => Some(Self::Polyline),
                "CONE" => Some(Self::Cone),
                "EMPTY" => Some(Self::Empty),
                "ARROW" => Some(Self::Arrow),
                "AXIS" => Some(Self::Axis),
                "CAPSULE" => Some(Self::Capsule),
                "ELLIPSOID" => Some(Self::Ellipsoid),
                _ => None,
            }
        }
    }
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WebRequest {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    #[prost(string, tag = "2")]
    pub operation: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub topic: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub msg_type: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub compression: ::prost::alloc::string::String,
    #[prost(double, tag = "6")]
    pub hz: f64,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Quaternion {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    #[prost(double, tag = "2")]
    pub x: f64,
    #[prost(double, tag = "3")]
    pub y: f64,
    #[prost(double, tag = "4")]
    pub z: f64,
    #[prost(double, tag = "5")]
    pub w: f64,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pose {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(uint32, tag = "3")]
    pub id: u32,
    #[prost(message, optional, tag = "4")]
    pub position: ::core::option::Option<Vector3d>,
    #[prost(message, optional, tag = "5")]
    pub orientation: ::core::option::Option<Quaternion>,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Hydra {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// Info for the right paddle
    #[prost(message, optional, tag = "2")]
    pub right: ::core::option::Option<hydra::Paddle>,
    /// Info for the left paddle
    #[prost(message, optional, tag = "3")]
    pub left: ::core::option::Option<hydra::Paddle>,
}
/// Nested message and enum types in `Hydra`.
pub mod hydra {
    #[derive(::rgz_derive::GzMessage)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Paddle {
        /// Pose of the paddle
        #[prost(message, optional, tag = "1")]
        pub pose: ::core::option::Option<super::Pose>,
        /// The button labeled LB
        #[prost(bool, tag = "2")]
        pub button_bumper: bool,
        /// Button 1
        #[prost(bool, tag = "3")]
        pub button_1: bool,
        /// Button 2
        #[prost(bool, tag = "4")]
        pub button_2: bool,
        /// Button 3
        #[prost(bool, tag = "5")]
        pub button_3: bool,
        /// Button 4
        #[prost(bool, tag = "6")]
        pub button_4: bool,
        /// Button that is activated by pressing down on the joystick.
        #[prost(bool, tag = "7")]
        pub button_joy: bool,
        /// The button located between button 1 and 2.
        #[prost(bool, tag = "8")]
        pub button_center: bool,
        /// Range(-1, 1) where -1 == back, and +1 == forward.
        #[prost(double, tag = "9")]
        pub joy_x: f64,
        /// Range(-1, 1) where -1 == left, and +1 == right.
        #[prost(double, tag = "10")]
        pub joy_y: f64,
        /// Range(0, 1) where 0 is no press, and 1 is full press.
        #[prost(double, tag = "11")]
        pub trigger: f64,
    }
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Friction {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief Coefficient of friction in the range of \[0..1\].
    #[prost(double, tag = "2")]
    pub mu: f64,
    /// / \brief Second coefficient of friction in the range of \[0..1\].
    #[prost(double, tag = "3")]
    pub mu2: f64,
    /// / \brief Direction of mu1 in the collision local reference frame.
    #[prost(message, optional, tag = "4")]
    pub fdir1: ::core::option::Option<Vector3d>,
    /// / \brief Force dependent slip direction 1 in collision local frame, between
    /// / the range of \[0..1\].
    #[prost(double, tag = "5")]
    pub slip1: f64,
    /// / \brief Force dependent slip direction 2 in collision local frame, between
    /// / the range of \[0..1\].
    #[prost(double, tag = "6")]
    pub slip2: f64,
    /// / \brief Torsional friction.
    #[prost(message, optional, tag = "7")]
    pub torsional: ::core::option::Option<friction::Torsional>,
}
/// Nested message and enum types in `Friction`.
pub mod friction {
    #[derive(::rgz_derive::GzMessage)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Torsional {
        /// / \brief Torsional coefficient of friction in the range of \[0..1\].
        #[prost(double, tag = "1")]
        pub coefficient: f64,
        /// / \brief By default, torsional friction is calculated using the
        /// / "patch_radius", which is sqrt(R*d), where "R" is the radius of the
        /// / collision at the contact point (surface_radius) and "d" is the contact
        /// / depth. If this flag is set to false, surface_radius and contact depth
        /// / will be used instead of patch radius.
        #[prost(bool, tag = "2")]
        pub use_patch_radius: bool,
        /// / \brief Radius of contact patch surface, used for torsional friction.
        #[prost(double, tag = "3")]
        pub patch_radius: f64,
        /// / \brief Surface radius on the point of contact, used for torsional
        /// / friction.
        #[prost(double, tag = "4")]
        pub surface_radius: f64,
        /// / \brief Torsional friction information exclusive to ODE physics engine.
        #[prost(message, optional, tag = "5")]
        pub ode: ::core::option::Option<torsional::Ode>,
    }
    /// Nested message and enum types in `Torsional`.
    pub mod torsional {
        #[derive(::rgz_derive::GzMessage)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Ode {
            /// / \brief Force dependent slip for torsional friction, between the range
            /// / of \[0..1\].
            #[prost(double, tag = "1")]
            pub slip: f64,
        }
    }
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Surface {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    #[prost(message, optional, tag = "2")]
    pub friction: ::core::option::Option<Friction>,
    #[prost(double, tag = "3")]
    pub restitution_coefficient: f64,
    #[prost(double, tag = "4")]
    pub bounce_threshold: f64,
    #[prost(double, tag = "5")]
    pub soft_cfm: f64,
    #[prost(double, tag = "6")]
    pub soft_erp: f64,
    #[prost(double, tag = "7")]
    pub kp: f64,
    #[prost(double, tag = "8")]
    pub kd: f64,
    #[prost(double, tag = "9")]
    pub max_vel: f64,
    #[prost(double, tag = "10")]
    pub min_depth: f64,
    #[prost(bool, tag = "11")]
    pub collide_without_contact: bool,
    #[prost(uint32, tag = "12")]
    pub collide_without_contact_bitmask: u32,
    #[prost(uint32, tag = "13")]
    pub collide_bitmask: u32,
    #[prost(double, tag = "14")]
    pub elastic_modulus: f64,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Color {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    #[prost(float, tag = "2")]
    pub r: f32,
    #[prost(float, tag = "3")]
    pub g: f32,
    #[prost(float, tag = "4")]
    pub b: f32,
    #[prost(float, tag = "5")]
    pub a: f32,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Material {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    #[prost(message, optional, tag = "2")]
    pub script: ::core::option::Option<material::Script>,
    #[prost(enumeration = "material::ShaderType", tag = "3")]
    pub shader_type: i32,
    #[prost(string, tag = "4")]
    pub normal_map: ::prost::alloc::string::String,
    /// / \brief Ambient color
    #[prost(message, optional, tag = "5")]
    pub ambient: ::core::option::Option<Color>,
    /// / \brief Diffuse color
    #[prost(message, optional, tag = "6")]
    pub diffuse: ::core::option::Option<Color>,
    /// / \brief Specular color
    #[prost(message, optional, tag = "7")]
    pub specular: ::core::option::Option<Color>,
    /// / \brief Emissive color
    #[prost(message, optional, tag = "8")]
    pub emissive: ::core::option::Option<Color>,
    #[prost(bool, tag = "9")]
    pub lighting: bool,
    /// / \brief Physically Based Rendering (PBR) material properties
    #[prost(message, optional, tag = "10")]
    pub pbr: ::core::option::Option<material::Pbr>,
    /// / \brief Render order. The higher value will be rendered on top of the
    /// / other coplanar polygons
    #[prost(double, tag = "11")]
    pub render_order: f64,
    /// / \brief If true, the mesh that this material is applied to will be
    /// / rendered as double sided
    #[prost(bool, tag = "12")]
    pub double_sided: bool,
    /// / \brief Specular exponent
    #[prost(double, tag = "13")]
    pub shininess: f64,
}
/// Nested message and enum types in `Material`.
pub mod material {
    #[derive(::rgz_derive::GzMessage)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Script {
        #[prost(string, repeated, tag = "1")]
        pub uri: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        #[prost(string, tag = "2")]
        pub name: ::prost::alloc::string::String,
    }
    /// / \brief Physically Based Rendering (PBR) material properties.
    #[derive(::rgz_derive::GzMessage)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Pbr {
        /// / \brief Type of PBR workflow
        #[prost(enumeration = "pbr::WorkflowType", tag = "1")]
        pub r#type: i32,
        /// / \brief Filename of the albedo map
        #[prost(string, tag = "2")]
        pub albedo_map: ::prost::alloc::string::String,
        /// / \brief Filename of the normal map
        #[prost(string, tag = "3")]
        pub normal_map: ::prost::alloc::string::String,
        /// / \brief Metalness value (metal workflow)
        #[prost(double, tag = "4")]
        pub metalness: f64,
        /// / \brief Filename of the metalness map (metal workflow)
        #[prost(string, tag = "5")]
        pub metalness_map: ::prost::alloc::string::String,
        /// / \brief Roughness value (metal workflow)
        #[prost(double, tag = "6")]
        pub roughness: f64,
        /// / \brief Filename of the roughness map (metal workflow)
        #[prost(string, tag = "7")]
        pub roughness_map: ::prost::alloc::string::String,
        /// / \brief Glossiness value (specular workflow)
        #[prost(double, tag = "8")]
        pub glossiness: f64,
        /// / \brief Filename of the glossiness map (specular workflow)
        #[prost(string, tag = "9")]
        pub glossiness_map: ::prost::alloc::string::String,
        /// / \brief Filename of the specular map (specular workflow)
        #[prost(string, tag = "10")]
        pub specular_map: ::prost::alloc::string::String,
        /// / \brief Filename of the environment map
        #[prost(string, tag = "11")]
        pub environment_map: ::prost::alloc::string::String,
        /// / \brief Filename of the ambient occlusion map
        #[prost(string, tag = "12")]
        pub ambient_occlusion_map: ::prost::alloc::string::String,
        /// / \brief Filename of the emissive map
        #[prost(string, tag = "13")]
        pub emissive_map: ::prost::alloc::string::String,
        /// / \brief Filename of the light map.
        #[prost(string, tag = "14")]
        pub light_map: ::prost::alloc::string::String,
        /// / \brief Texture coordinate set for the light map
        #[prost(uint32, tag = "15")]
        pub light_map_texcoord_set: u32,
    }
    /// Nested message and enum types in `PBR`.
    pub mod pbr {
        /// / \brief Type of PBR workflow
        #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            ::prost::Enumeration
        )]
        #[repr(i32)]
        pub enum WorkflowType {
            /// / \brief No workflow
            None = 0,
            /// / \brief Metallic/Roughness workflow
            Metal = 1,
            /// / \brief Specular/Glossiness workflow
            Specular = 2,
        }
        impl WorkflowType {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    WorkflowType::None => "NONE",
                    WorkflowType::Metal => "METAL",
                    WorkflowType::Specular => "SPECULAR",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "NONE" => Some(Self::None),
                    "METAL" => Some(Self::Metal),
                    "SPECULAR" => Some(Self::Specular),
                    _ => None,
                }
            }
        }
    }
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum ShaderType {
        Vertex = 0,
        Pixel = 1,
        NormalMapObjectSpace = 2,
        NormalMapTangentSpace = 3,
    }
    impl ShaderType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ShaderType::Vertex => "VERTEX",
                ShaderType::Pixel => "PIXEL",
                ShaderType::NormalMapObjectSpace => "NORMAL_MAP_OBJECT_SPACE",
                ShaderType::NormalMapTangentSpace => "NORMAL_MAP_TANGENT_SPACE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "VERTEX" => Some(Self::Vertex),
                "PIXEL" => Some(Self::Pixel),
                "NORMAL_MAP_OBJECT_SPACE" => Some(Self::NormalMapObjectSpace),
                "NORMAL_MAP_TANGENT_SPACE" => Some(Self::NormalMapTangentSpace),
                _ => None,
            }
        }
    }
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Plugin {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub filename: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub innerxml: ::prost::alloc::string::String,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Visual {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(uint32, tag = "3")]
    pub id: u32,
    #[prost(string, tag = "4")]
    pub parent_name: ::prost::alloc::string::String,
    #[prost(uint32, tag = "5")]
    pub parent_id: u32,
    #[prost(bool, tag = "6")]
    pub cast_shadows: bool,
    #[prost(double, tag = "7")]
    pub transparency: f64,
    #[prost(double, tag = "8")]
    pub laser_retro: f64,
    #[prost(message, optional, tag = "9")]
    pub pose: ::core::option::Option<Pose>,
    #[prost(message, optional, tag = "10")]
    pub geometry: ::core::option::Option<Geometry>,
    #[prost(message, optional, tag = "11")]
    pub material: ::core::option::Option<Material>,
    #[prost(bool, tag = "12")]
    pub visible: bool,
    #[prost(bool, tag = "13")]
    pub delete_me: bool,
    #[prost(bool, tag = "14")]
    pub is_static: bool,
    #[prost(message, repeated, tag = "15")]
    pub plugin: ::prost::alloc::vec::Vec<Plugin>,
    #[prost(message, optional, tag = "16")]
    pub scale: ::core::option::Option<Vector3d>,
    /// / \brief Option meta information associated with this visual.
    #[prost(message, optional, tag = "17")]
    pub meta: ::core::option::Option<visual::Meta>,
    /// / \brief Type of visual.
    #[prost(enumeration = "visual::Type", tag = "18")]
    pub r#type: i32,
}
/// Nested message and enum types in `Visual`.
pub mod visual {
    /// / \brief Optional meta information for the visual. The information
    /// / contained within this element should be used to provide additional
    /// / feedback to an end user.
    #[derive(::rgz_derive::GzMessage)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Meta {
        /// / \brief The layer in which this visual is displayed. The layer number
        /// / is useful for programs, such as Gazebo, that put visuals in different
        /// / layers for enhanced visualization.
        #[prost(int32, tag = "1")]
        pub layer: i32,
    }
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Type {
        /// / \brief Entity visual
        Entity = 0,
        /// / \brief Model visual
        Model = 1,
        /// / \brief Link visual
        Link = 2,
        /// / \brief Visual visual
        Visual = 3,
        /// / \brief Collision visual
        Collision = 4,
        /// / \brief Sensor visual
        Sensor = 5,
        /// / \brief GUI visual
        Gui = 6,
        /// / \brief Physics data visual
        Physics = 7,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Entity => "ENTITY",
                Type::Model => "MODEL",
                Type::Link => "LINK",
                Type::Visual => "VISUAL",
                Type::Collision => "COLLISION",
                Type::Sensor => "SENSOR",
                Type::Gui => "GUI",
                Type::Physics => "PHYSICS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ENTITY" => Some(Self::Entity),
                "MODEL" => Some(Self::Model),
                "LINK" => Some(Self::Link),
                "VISUAL" => Some(Self::Visual),
                "COLLISION" => Some(Self::Collision),
                "SENSOR" => Some(Self::Sensor),
                "GUI" => Some(Self::Gui),
                "PHYSICS" => Some(Self::Physics),
                _ => None,
            }
        }
    }
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Collision {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    #[prost(uint32, tag = "2")]
    pub id: u32,
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    #[prost(double, tag = "4")]
    pub laser_retro: f64,
    #[prost(double, tag = "5")]
    pub max_contacts: f64,
    #[prost(message, optional, tag = "6")]
    pub pose: ::core::option::Option<Pose>,
    #[prost(message, optional, tag = "7")]
    pub geometry: ::core::option::Option<Geometry>,
    #[prost(message, optional, tag = "8")]
    pub surface: ::core::option::Option<Surface>,
    #[prost(message, repeated, tag = "9")]
    pub visual: ::prost::alloc::vec::Vec<Visual>,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Light {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(enumeration = "light::LightType", tag = "3")]
    pub r#type: i32,
    #[prost(message, optional, tag = "4")]
    pub pose: ::core::option::Option<Pose>,
    #[prost(message, optional, tag = "5")]
    pub diffuse: ::core::option::Option<Color>,
    #[prost(message, optional, tag = "6")]
    pub specular: ::core::option::Option<Color>,
    #[prost(float, tag = "7")]
    pub attenuation_constant: f32,
    #[prost(float, tag = "8")]
    pub attenuation_linear: f32,
    #[prost(float, tag = "9")]
    pub attenuation_quadratic: f32,
    #[prost(message, optional, tag = "10")]
    pub direction: ::core::option::Option<Vector3d>,
    #[prost(float, tag = "11")]
    pub range: f32,
    #[prost(bool, tag = "12")]
    pub cast_shadows: bool,
    #[prost(float, tag = "13")]
    pub spot_inner_angle: f32,
    #[prost(float, tag = "14")]
    pub spot_outer_angle: f32,
    #[prost(float, tag = "15")]
    pub spot_falloff: f32,
    /// / \brief Unique id of the light
    #[prost(uint32, tag = "16")]
    pub id: u32,
    /// / \brief Unique id of light's parent
    #[prost(uint32, tag = "17")]
    pub parent_id: u32,
    /// / \brief light intensity
    #[prost(float, tag = "18")]
    pub intensity: f32,
    /// / \brief is the light on or off
    /// / true is off, otherwise is on
    #[prost(bool, tag = "19")]
    pub is_light_off: bool,
    /// / \brief Set if the visual of the light
    /// / is visible, true is visible,
    /// / otherwise is invisible
    #[prost(bool, tag = "20")]
    pub visualize_visual: bool,
}
/// Nested message and enum types in `Light`.
pub mod light {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum LightType {
        Point = 0,
        Spot = 1,
        Directional = 2,
    }
    impl LightType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                LightType::Point => "POINT",
                LightType::Spot => "SPOT",
                LightType::Directional => "DIRECTIONAL",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "POINT" => Some(Self::Point),
                "SPOT" => Some(Self::Spot),
                "DIRECTIONAL" => Some(Self::Directional),
                _ => None,
            }
        }
    }
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AxisAlignedBox {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief Minimum corner of the axis aligned bound box in the global frame.
    #[prost(message, optional, tag = "2")]
    pub min_corner: ::core::option::Option<Vector3d>,
    /// / \brief Maximum corner of the axis aligned bound box in the global frame.
    #[prost(message, optional, tag = "3")]
    pub max_corner: ::core::option::Option<Vector3d>,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Axis {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief The x,y,z components of the axis unit vector.
    #[prost(message, optional, tag = "2")]
    pub xyz: ::core::option::Option<Vector3d>,
    /// / \brief The lower joint axis limit (radians for revolute joints,
    /// / meters for prismatic joints). Not valid if the joint that uses this
    /// / axis is continuous.
    #[prost(double, tag = "3")]
    pub limit_lower: f64,
    /// / \brief The upper joint axis limit (radians for revolute joints,
    /// / meters for prismatic joints). Not valid if the joint that uses this
    /// / axis is continuous.
    #[prost(double, tag = "4")]
    pub limit_upper: f64,
    /// / \brief Value for enforcing the maximum joint effort applied.
    /// / Limit is not enforced if value is negative.
    #[prost(double, tag = "5")]
    pub limit_effort: f64,
    /// / \brief Value for enforcing the maximum joint velocity.
    #[prost(double, tag = "6")]
    pub limit_velocity: f64,
    /// / \brief The physical velocity dependent viscous damping coefficient
    /// / of the joint axis.
    #[prost(double, tag = "7")]
    pub damping: f64,
    /// / \brief The physical static friction value of the joint.
    #[prost(double, tag = "8")]
    pub friction: f64,
    /// / \brief Position of the joint. For angular joints, such as revolute
    /// / joints, the units are radians. For linear joints, such as prismatic
    /// / joints, the units are meters.
    #[prost(double, tag = "10")]
    pub position: f64,
    /// / \brief Velocity of the joint in SI units (meter/second).
    #[prost(double, tag = "11")]
    pub velocity: f64,
    /// / \brief Force applied to the joint in SI units (Newton).
    #[prost(double, tag = "12")]
    pub force: f64,
    /// / \brief Acceleration of the joint is SI units (meter/second^2).
    #[prost(double, tag = "13")]
    pub acceleration: f64,
    /// / \brief Set the name of the coordinate frame in which this joint axis's
    /// / unit vector is expressed. An empty value implies the parent (joint)
    /// / frame.
    #[prost(string, tag = "14")]
    pub xyz_expressed_in: ::prost::alloc::string::String,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SensorNoise {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief The type of noise
    #[prost(enumeration = "sensor_noise::Type", tag = "2")]
    pub r#type: i32,
    /// / \brief Noise mean
    /// / Used by GAUSSIAN, and GAUSSIAN_QUANTIZED
    #[prost(double, tag = "3")]
    pub mean: f64,
    /// / \brief Noise standard deviation
    /// / Used by GAUSSIAN, and GAUSSIAN_QUANTIZED
    #[prost(double, tag = "4")]
    pub stddev: f64,
    /// / \brief Noise mean bias
    /// / Used by GAUSSIAN, and GAUSSIAN_QUANTIZED
    #[prost(double, tag = "5")]
    pub bias_mean: f64,
    /// / \brief Noise standard deviation bias
    /// / Used by GAUSSIAN, and GAUSSIAN_QUANTIZED
    #[prost(double, tag = "6")]
    pub bias_stddev: f64,
    /// / \brief Noise  precision.
    /// / Used by GAUSSIAN_QUANTIZED
    #[prost(double, tag = "7")]
    pub precision: f64,
    /// / \brief For type "gaussian*", the standard deviation of the noise used to
    /// / drive a process to model slow variations in a sensor bias.
    #[prost(double, tag = "8")]
    pub dynamic_bias_stddev: f64,
    /// / \brief For type "gaussian*", the correlation time in seconds of the
    /// / noise used to drive a process to model slow variations in a sensor bias.
    /// / A typical value, when used, would be on the order of
    /// / 3600 seconds (1 hour).
    #[prost(double, tag = "9")]
    pub dynamic_bias_correlation_time: f64,
}
/// Nested message and enum types in `SensorNoise`.
pub mod sensor_noise {
    /// / \brief Noise types
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Type {
        /// / \brief No noise
        None = 0,
        /// / \brief Gaussian noise
        Gaussian = 2,
        /// / \brief Gaussian noise plus quantization of outputs (rounding)
        GaussianQuantized = 3,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::None => "NONE",
                Type::Gaussian => "GAUSSIAN",
                Type::GaussianQuantized => "GAUSSIAN_QUANTIZED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "NONE" => Some(Self::None),
                "GAUSSIAN" => Some(Self::Gaussian),
                "GAUSSIAN_QUANTIZED" => Some(Self::GaussianQuantized),
                _ => None,
            }
        }
    }
}
/// / \brief Message that describes an altimeter sensor.
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AltimeterSensor {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief Noise parameters for the vertical position.
    #[prost(message, optional, tag = "2")]
    pub vertical_position_noise: ::core::option::Option<SensorNoise>,
    /// / \brief Noise parameters for the vertical velocity.
    #[prost(message, optional, tag = "3")]
    pub vertical_velocity_noise: ::core::option::Option<SensorNoise>,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Distortion {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    #[prost(message, optional, tag = "2")]
    pub center: ::core::option::Option<Vector2d>,
    #[prost(double, tag = "3")]
    pub k1: f64,
    #[prost(double, tag = "4")]
    pub k2: f64,
    #[prost(double, tag = "5")]
    pub k3: f64,
    #[prost(double, tag = "6")]
    pub p1: f64,
    #[prost(double, tag = "7")]
    pub p2: f64,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Double {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief Double data
    #[prost(double, tag = "2")]
    pub data: f64,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Lens {
    /// / \brief Lens type
    #[prost(enumeration = "lens::Type", tag = "1")]
    pub r#type: i32,
    /// / \brief Lens scale to horizontal field of view.
    #[prost(bool, tag = "2")]
    pub scale_to_hfov: bool,
    /// / \brief Lens custom function linear scaling constant
    #[prost(double, tag = "3")]
    pub c1: f64,
    /// / \brief Lens custom function angular scaling constant.
    #[prost(double, tag = "4")]
    pub c2: f64,
    /// / \brief Lens custom function angle offset constant.
    #[prost(double, tag = "5")]
    pub c3: f64,
    /// / \brief Lens custom function focal length.
    #[prost(double, tag = "6")]
    pub focal_length: f64,
    /// / \brief Lens custom function type.
    #[prost(enumeration = "lens::FunctionType", tag = "7")]
    pub function_type: i32,
    /// / \brief Lens cutoff angle in radians. Everything outside of the specified
    /// / angle will be hidden.
    #[prost(double, tag = "8")]
    pub cutoff_angle: f64,
    /// / \brief The resolution of the environment cube map used to draw the world.
    #[prost(int32, tag = "9")]
    pub environment_texture_size: i32,
    /// / \brief Lens X focal length in pixels.
    #[prost(double, tag = "10")]
    pub intrinsics_fx: f64,
    /// / \brief Lens Y focal length in pixels.
    #[prost(double, tag = "11")]
    pub intrinsics_fy: f64,
    /// / \brief Lens X principal point in pixels.
    #[prost(double, tag = "12")]
    pub intrinsics_cx: f64,
    /// / \brief Lens Y principal point in pixels.
    #[prost(double, tag = "13")]
    pub intrinsics_cy: f64,
    /// / \brief Lens XY axis skew.
    #[prost(double, tag = "14")]
    pub intrinsics_skew: f64,
}
/// Nested message and enum types in `Lens`.
pub mod lens {
    /// / \brief Types of lens models.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Type {
        NotSpecified = 0,
        Gnomonical = 1,
        Stereographic = 2,
        Equidistant = 3,
        EquisolidAngle = 4,
        Orthographic = 5,
        Custom = 6,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::NotSpecified => "TYPE_NOT_SPECIFIED",
                Type::Gnomonical => "GNOMONICAL",
                Type::Stereographic => "STEREOGRAPHIC",
                Type::Equidistant => "EQUIDISTANT",
                Type::EquisolidAngle => "EQUISOLID_ANGLE",
                Type::Orthographic => "ORTHOGRAPHIC",
                Type::Custom => "CUSTOM",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TYPE_NOT_SPECIFIED" => Some(Self::NotSpecified),
                "GNOMONICAL" => Some(Self::Gnomonical),
                "STEREOGRAPHIC" => Some(Self::Stereographic),
                "EQUIDISTANT" => Some(Self::Equidistant),
                "EQUISOLID_ANGLE" => Some(Self::EquisolidAngle),
                "ORTHOGRAPHIC" => Some(Self::Orthographic),
                "CUSTOM" => Some(Self::Custom),
                _ => None,
            }
        }
    }
    /// / \brief Lens custom function type.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum FunctionType {
        FunctionNotSpecified = 0,
        Sin = 1,
        Tan = 2,
        Id = 3,
    }
    impl FunctionType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                FunctionType::FunctionNotSpecified => "FUNCTION_NOT_SPECIFIED",
                FunctionType::Sin => "SIN",
                FunctionType::Tan => "TAN",
                FunctionType::Id => "ID",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "FUNCTION_NOT_SPECIFIED" => Some(Self::FunctionNotSpecified),
                "SIN" => Some(Self::Sin),
                "TAN" => Some(Self::Tan),
                "ID" => Some(Self::Id),
                _ => None,
            }
        }
    }
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CameraSensor {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief Horizontal field of view in radians
    #[prost(double, tag = "2")]
    pub horizontal_fov: f64,
    /// / \brief Image size in pixels.
    #[prost(message, optional, tag = "3")]
    pub image_size: ::core::option::Option<Vector2d>,
    /// / \brief Image format. This field is deprecated, please use pixel_format.
    #[prost(string, tag = "4")]
    pub image_format: ::prost::alloc::string::String,
    /// / \brief Near clip distance in meters.
    #[prost(double, tag = "5")]
    pub near_clip: f64,
    /// / \brief Far clip distance in meters.
    #[prost(double, tag = "6")]
    pub far_clip: f64,
    /// / \brief True if frames should be saved.
    #[prost(bool, tag = "7")]
    pub save_enabled: bool,
    /// / \brief Path in which to save frames.
    #[prost(string, tag = "8")]
    pub save_path: ::prost::alloc::string::String,
    /// / \brief Optional distortion information.
    #[prost(message, optional, tag = "9")]
    pub distortion: ::core::option::Option<Distortion>,
    /// / \brief Optional noise parameters for the image.
    #[prost(message, optional, tag = "10")]
    pub image_noise: ::core::option::Option<SensorNoise>,
    /// / \brief Optional depth near clip in meters.
    #[prost(message, optional, tag = "11")]
    pub depth_near_clip: ::core::option::Option<Double>,
    /// / \brief Optional depth far clip in meters.
    #[prost(message, optional, tag = "12")]
    pub depth_far_clip: ::core::option::Option<Double>,
    /// / \brief Optional bounding box camera type.
    #[prost(enumeration = "camera_sensor::BoundingBoxType", tag = "13")]
    pub bounding_box_type: i32,
    /// / \brief Optional segmentation camera type.
    #[prost(enumeration = "camera_sensor::SegmentationType", tag = "14")]
    pub segmentation_type: i32,
    /// / \brief Optional lens information
    #[prost(message, optional, tag = "15")]
    pub lens: ::core::option::Option<Lens>,
    /// / \brief True if the camera will be triggered by a topic
    #[prost(bool, tag = "16")]
    pub triggered: bool,
    /// / \brief Name of the topic that will trigger the camera if enabled
    #[prost(string, tag = "17")]
    pub triggered_topic: ::prost::alloc::string::String,
    /// / \brief Value used for anti-aliasing
    #[prost(int32, tag = "18")]
    pub anti_aliasing: i32,
    /// / \brief Visibility mask of a camera. When the camera's visibility_mask and
    /// / a visual's visibility_flags evaluates to non-zero, then the visual will
    /// / be visible to the camera.
    #[prost(uint32, tag = "19")]
    pub visibility_mask: u32,
    /// / \brief True if the camera is a depth camera.
    #[prost(bool, tag = "20")]
    pub is_depth_camera: bool,
    /// / \brief Pixel format used by the camera. This replaces image_format.
    #[prost(enumeration = "PixelFormatType", tag = "21")]
    pub pixel_format: i32,
}
/// Nested message and enum types in `CameraSensor`.
pub mod camera_sensor {
    /// / \brief Bounding box types.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum BoundingBoxType {
        /// / \brief No bounding box.
        NoBoundingBox = 0,
        /// / \brief 2D box that shows the full box of occluded objects
        FullBox2d = 1,
        /// / \brief 2D box that shows the visible part of the occluded object
        VisibleBox2d = 2,
        /// / \brief 3D oriented box
        Box3d = 3,
    }
    impl BoundingBoxType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                BoundingBoxType::NoBoundingBox => "NO_BOUNDING_BOX",
                BoundingBoxType::FullBox2d => "FULL_BOX_2D",
                BoundingBoxType::VisibleBox2d => "VISIBLE_BOX_2D",
                BoundingBoxType::Box3d => "BOX_3D",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "NO_BOUNDING_BOX" => Some(Self::NoBoundingBox),
                "FULL_BOX_2D" => Some(Self::FullBox2d),
                "VISIBLE_BOX_2D" => Some(Self::VisibleBox2d),
                "BOX_3D" => Some(Self::Box3d),
                _ => None,
            }
        }
    }
    /// / \brief Segmentation types.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum SegmentationType {
        /// / \brief No segmentation.
        NoSegmentation = 0,
        /// / \brief Pixels of same label from different items
        /// / have the same color & id.
        Semantic = 1,
        /// / \brief Pixels of same label from different items, have different
        /// / color & id. 1 channel for label id & 2 channels for instance id
        Panoptic = 2,
    }
    impl SegmentationType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SegmentationType::NoSegmentation => "NO_SEGMENTATION",
                SegmentationType::Semantic => "SEMANTIC",
                SegmentationType::Panoptic => "PANOPTIC",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "NO_SEGMENTATION" => Some(Self::NoSegmentation),
                "SEMANTIC" => Some(Self::Semantic),
                "PANOPTIC" => Some(Self::Panoptic),
                _ => None,
            }
        }
    }
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContactSensor {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    #[prost(string, tag = "2")]
    pub collision_name: ::prost::alloc::string::String,
}
/// / \brief Message that describes an air pressure sensor.
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AirPressureSensor {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief Reference altitude in meters. This value can be used by a sensor
    /// / implementation to augment the altitude of the sensor. For example, if
    /// / you are using simulation, instead of creating a 1000 m mountain model on
    /// / which to place your sensor, you could instead set this value to 1000 and
    /// / place your model on a ground plane with a Z height of zero.
    #[prost(double, tag = "2")]
    pub reference_altitude: f64,
    /// / \brief Sensor pressure noise.
    #[prost(message, optional, tag = "3")]
    pub pressure_noise: ::core::option::Option<SensorNoise>,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GpsSensor {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief Position sensing. Consists of horizontal and vertical noise
    /// / properties
    #[prost(message, optional, tag = "2")]
    pub position: ::core::option::Option<gps_sensor::Sensing>,
    /// / \brief Velocity sensing. Consists of horizontal and vertical noise
    /// / properties
    #[prost(message, optional, tag = "3")]
    pub velocity: ::core::option::Option<gps_sensor::Sensing>,
}
/// Nested message and enum types in `GPSSensor`.
pub mod gps_sensor {
    /// / \brief Sensing information
    #[derive(::rgz_derive::GzMessage)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Sensing {
        /// / \brief Horizontal noise
        #[prost(message, optional, tag = "1")]
        pub horizontal_noise: ::core::option::Option<super::SensorNoise>,
        /// / \brief Vertical noise
        #[prost(message, optional, tag = "2")]
        pub vertical_noise: ::core::option::Option<super::SensorNoise>,
    }
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImuSensor {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief Angular velocity information
    #[prost(message, optional, tag = "2")]
    pub angular_velocity: ::core::option::Option<imu_sensor::AngularVelocity>,
    /// / \brief Linear acceleration information
    #[prost(message, optional, tag = "3")]
    pub linear_acceleration: ::core::option::Option<imu_sensor::LinearAcceleration>,
    /// / \brief Orientation reference frame information.
    #[prost(message, optional, tag = "4")]
    pub orientation_ref_frame: ::core::option::Option<
        imu_sensor::OrientationReferenceFrame,
    >,
}
/// Nested message and enum types in `IMUSensor`.
pub mod imu_sensor {
    /// / \brief Angular velocity information
    #[derive(::rgz_derive::GzMessage)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AngularVelocity {
        /// / \brief Noise about the x-axis
        #[prost(message, optional, tag = "1")]
        pub x_noise: ::core::option::Option<super::SensorNoise>,
        /// / \brief Noise about the y-axis
        #[prost(message, optional, tag = "2")]
        pub y_noise: ::core::option::Option<super::SensorNoise>,
        /// / \brief Noise about the z-axis
        #[prost(message, optional, tag = "3")]
        pub z_noise: ::core::option::Option<super::SensorNoise>,
    }
    /// / \brief Linear acceleration information
    #[derive(::rgz_derive::GzMessage)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LinearAcceleration {
        /// / \brief Noise about the x-axis
        #[prost(message, optional, tag = "1")]
        pub x_noise: ::core::option::Option<super::SensorNoise>,
        /// / \brief Noise about the y-axis
        #[prost(message, optional, tag = "2")]
        pub y_noise: ::core::option::Option<super::SensorNoise>,
        /// / \brief Noise about the z-axis
        #[prost(message, optional, tag = "3")]
        pub z_noise: ::core::option::Option<super::SensorNoise>,
    }
    /// / \brief Orientation reference frame information
    #[derive(::rgz_derive::GzMessage)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct OrientationReferenceFrame {
        /// / \brief This string represents special hardcoded use cases that are
        /// / commonly seen with typical robot IMU's:
        /// /   - CUSTOM: use Euler angle custom_rpy orientation specification.
        /// /             The orientation of the IMU's reference frame is defined
        /// /             by adding the custom_rpy rotation
        /// /             to the parent_frame.
        /// /   - NED: The IMU XYZ aligns with NED, where NED orientation relative
        /// /          to the world
        /// /             is defined by the SphericalCoordinates class.
        /// /   - ENU: The IMU XYZ aligns with ENU, where ENU orientation relative
        /// /          to the world is defined by the SphericalCoordinates class.
        /// /   - NWU: The IMU XYZ aligns with NWU, where NWU orientation relative
        /// /          to the world is defined by the SphericalCoordinates class.
        /// /   - GRAV_UP: where direction of gravity maps to IMU reference frame
        /// /              Z-axis with Z-axis pointing in the opposite direction of
        /// /              gravity. IMU reference frame X-axis direction is defined
        /// /              by GravityDirX(). Note if GravityDirX() is parallel to
        /// /              gravity direction, this configuration fails. Otherwise,
        /// /              IMU reference frame X-axis is defined by projection of
        /// /              GravtyDirX onto a plane normal to the gravity vector.
        /// /              IMU reference frame Y-axis is a vector orthogonal to
        /// /              both X and Z axis following the right hand rule.
        /// /  - GRAV_DOWN: where direction of gravity maps to IMU reference frame
        /// /               Z-axis with Z-axis pointing in the direction of gravity.
        /// /               IMU reference frame X-axis direction is defined by
        /// /               GravityDirX(). Note if GravityDirX() is parallel to
        /// /               gravity direction, this configuration fails. Otherwise,
        /// /               IMU reference frame X-axis is defined by projection of
        /// /               GravityDirX() onto a plane normal to the gravity vector.
        /// /               IMU reference frame Y-axis is a vector orthogonal to both
        /// /               X and Z axis following the right hand rule.
        #[prost(string, tag = "1")]
        pub localization: ::prost::alloc::string::String,
        /// / \brief This field and custom_rpy_parent_frame are used when
        /// / Localization is set to CUSTOM. Orientation
        /// / (fixed axis roll, pitch yaw) transform from ParentFrame to this IMU's
        /// / reference frame.
        /// /
        /// / Some common examples are:
        /// /  - IMU reports in its local frame on boot. IMU sensor frame is the
        /// /    reference frame. Example: parent_frame="", custom_rpy="0 0 0"
        /// /  - IMU reports in Gazebo world frame.
        /// /    Example sdf: parent_frame="world", custom_rpy="0 0 0"
        /// /  - IMU reports in NWU frame. Uses SphericalCoordinates class to
        /// /    determine world frame in relation to magnetic north and gravity;
        /// /    i.e. rotation between North-West-Up and world (+X,+Y,+Z) frame is
        /// /    defined by SphericalCoordinates class.
        /// /    Example sdf given world is NWU: parent_frame="world",
        /// /    custom_rpy="0 0 0"
        /// /  - IMU reports in NED frame. Uses SphericalCoordinates class to
        /// /    determine world frame in relation to magnetic north and gravity;
        /// /    i.e. rotation between North-East-Down and world (+X,+Y,+Z) frame is
        /// /    defined by SphericalCoordinates class.
        /// /    Example sdf given world is NWU: parent_frame="world",
        /// /    custom_rpy="M_PI 0 0"
        /// /  - IMU reports in ENU frame. Uses SphericalCoordinates class to
        /// /    determine world frame in relation to magnetic north and gravity;
        /// /    i.e. rotation between East-North-Up and world (+X,+Y,+Z) frame is
        /// /    defined by SphericalCoordinates class.
        /// /    Example sdf given world is NWU: parent_frame="world",
        /// /    custom_rpy="0 0 -0.5*M_PI"
        /// /  - IMU reports in ROS optical frame as described in
        /// /    <http://www.ros.org/reps/rep-0103.html#suffix-frames,> which is
        /// /    (z-forward, x-left to right when facing +z, y-top to bottom when
        /// /    facing +z). (default gazebo camera is +x:view direction, +y:left,
        /// /    +z:up).
        /// /    Example sdf: parent_frame="local", custom_rpy="-0.5*M_PI 0 -0.5*M_PI"
        #[prost(message, optional, tag = "2")]
        pub custom_rpy: ::core::option::Option<super::Vector3d>,
        /// / \brief The name of parent frame which the custom_rpy transform is
        /// / defined relative to. It can be any valid fully scoped link name or the
        /// / special reserved "world" frame. If left empty, use the sensor's own
        /// / local frame.
        #[prost(string, tag = "3")]
        pub custom_rpy_parent_frame: ::prost::alloc::string::String,
        /// / \brief Used when localization is set to GRAV_UP or GRAV_DOWN, a
        /// / projection of this vector into a plane that is orthogonal to the
        /// / gravity vector defines the direction of the IMU reference frame's
        /// / X-axis.  grav_dir_x is  defined in the coordinate frame as defined by
        /// / the parent_frame element.
        #[prost(message, optional, tag = "4")]
        pub gravity_dir_x: ::core::option::Option<super::Vector3d>,
        /// / \brief The name of parent frame which the GravityDirX vector is
        /// / defined relative to. It can be any valid fully scoped link name or the
        /// / special reserved "world" frame. If left empty, use the sensor's own
        /// / local frame.
        #[prost(string, tag = "5")]
        pub gravity_dir_x_parent_frame: ::prost::alloc::string::String,
    }
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LidarSensor {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    #[prost(bool, tag = "2")]
    pub display_scan: bool,
    #[prost(int32, tag = "3")]
    pub horizontal_samples: i32,
    #[prost(double, tag = "4")]
    pub horizontal_resolution: f64,
    #[prost(double, tag = "5")]
    pub horizontal_min_angle: f64,
    #[prost(double, tag = "6")]
    pub horizontal_max_angle: f64,
    #[prost(int32, tag = "7")]
    pub vertical_samples: i32,
    #[prost(double, tag = "8")]
    pub vertical_resolution: f64,
    #[prost(double, tag = "9")]
    pub vertical_min_angle: f64,
    #[prost(double, tag = "10")]
    pub vertical_max_angle: f64,
    #[prost(double, tag = "11")]
    pub range_min: f64,
    #[prost(double, tag = "12")]
    pub range_max: f64,
    #[prost(double, tag = "13")]
    pub range_resolution: f64,
    #[prost(message, optional, tag = "14")]
    pub noise: ::core::option::Option<SensorNoise>,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogicalCameraSensor {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief Near clipping plane of the view frustum in meters.
    #[prost(double, tag = "2")]
    pub near_clip: f64,
    /// / \brief Far clipping plane of the view frustum in meters.
    #[prost(double, tag = "3")]
    pub far_clip: f64,
    /// / \brief Horizontal field of view in radians.
    #[prost(double, tag = "4")]
    pub horizontal_fov: f64,
    /// / \brief Near and far clipping plane aspect ratio (width/height).
    #[prost(double, tag = "5")]
    pub aspect_ratio: f64,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MagnetometerSensor {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief Noise about the x-axis
    #[prost(message, optional, tag = "2")]
    pub x_noise: ::core::option::Option<SensorNoise>,
    /// / \brief Noise about the y-axis
    #[prost(message, optional, tag = "3")]
    pub y_noise: ::core::option::Option<SensorNoise>,
    /// / \brief Noise about the z-axis
    #[prost(message, optional, tag = "4")]
    pub z_noise: ::core::option::Option<SensorNoise>,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sensor {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief Name of the sensor
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// / \brief Id of the sensor
    #[prost(uint32, tag = "3")]
    pub id: u32,
    /// / \brief Name of the parent, usually a link or joint.
    #[prost(string, tag = "4")]
    pub parent: ::prost::alloc::string::String,
    /// / \brief Id of the parent, usually a link or joint.
    #[prost(uint32, tag = "5")]
    pub parent_id: u32,
    /// / \brief Sensor type
    #[prost(string, tag = "6")]
    pub r#type: ::prost::alloc::string::String,
    /// / \brief True indicates that the sensor should always
    /// / produce data, instead of producing data only when
    /// / a consumer is connected to the data topic
    #[prost(bool, tag = "7")]
    pub always_on: bool,
    /// / \brief Refresh rate
    #[prost(double, tag = "8")]
    pub update_rate: f64,
    /// / \brief Sensor pose
    #[prost(message, optional, tag = "9")]
    pub pose: ::core::option::Option<Pose>,
    /// / \brief Description of a camera sensor
    #[prost(message, optional, tag = "10")]
    pub camera: ::core::option::Option<CameraSensor>,
    /// / \brief Description of a contact sensor
    #[prost(message, optional, tag = "11")]
    pub contact: ::core::option::Option<ContactSensor>,
    /// / \brief True value indicates that sensor data should be
    /// / visualized in the GUI
    #[prost(bool, tag = "12")]
    pub visualize: bool,
    /// / \brief Topic on which sensor data is published
    #[prost(string, tag = "13")]
    pub topic: ::prost::alloc::string::String,
    /// / \brief Description of a logical camera sensor
    #[prost(message, optional, tag = "14")]
    pub logical_camera: ::core::option::Option<LogicalCameraSensor>,
    /// / \brief Description of a gps sensor
    /// / TODO(chapulina) Migrate to NavSat
    #[prost(message, optional, tag = "15")]
    pub gps: ::core::option::Option<GpsSensor>,
    /// / \brief Description of an IMU sensor
    #[prost(message, optional, tag = "16")]
    pub imu: ::core::option::Option<ImuSensor>,
    /// / \brief Description of a Magnetometer sensor
    #[prost(message, optional, tag = "17")]
    pub magnetometer: ::core::option::Option<MagnetometerSensor>,
    /// / \brief Description of an Altimeter sensor.
    #[prost(message, optional, tag = "18")]
    pub altimeter: ::core::option::Option<AltimeterSensor>,
    /// / \brief Description of an Air Pressure sensor.
    #[prost(message, optional, tag = "19")]
    pub air_pressure: ::core::option::Option<AirPressureSensor>,
    /// / \brief Description of a lidar sensor
    #[prost(message, optional, tag = "20")]
    pub lidar: ::core::option::Option<LidarSensor>,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Joint {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(uint32, tag = "3")]
    pub id: u32,
    #[prost(enumeration = "joint::Type", tag = "4")]
    pub r#type: i32,
    #[prost(string, tag = "5")]
    pub parent: ::prost::alloc::string::String,
    #[prost(uint32, tag = "6")]
    pub parent_id: u32,
    #[prost(string, tag = "7")]
    pub child: ::prost::alloc::string::String,
    #[prost(uint32, tag = "8")]
    pub child_id: u32,
    #[prost(message, optional, tag = "9")]
    pub pose: ::core::option::Option<Pose>,
    #[prost(message, optional, tag = "10")]
    pub axis1: ::core::option::Option<Axis>,
    #[prost(message, optional, tag = "11")]
    pub axis2: ::core::option::Option<Axis>,
    #[prost(double, tag = "12")]
    pub cfm: f64,
    #[prost(double, tag = "13")]
    pub bounce: f64,
    #[prost(double, tag = "14")]
    pub fudge_factor: f64,
    #[prost(double, tag = "15")]
    pub limit_cfm: f64,
    #[prost(double, tag = "16")]
    pub limit_erp: f64,
    #[prost(double, tag = "17")]
    pub suspension_cfm: f64,
    #[prost(double, tag = "18")]
    pub suspension_erp: f64,
    #[prost(message, optional, tag = "19")]
    pub gearbox: ::core::option::Option<joint::Gearbox>,
    #[prost(message, optional, tag = "20")]
    pub screw: ::core::option::Option<joint::Screw>,
    #[prost(message, repeated, tag = "21")]
    pub sensor: ::prost::alloc::vec::Vec<Sensor>,
}
/// Nested message and enum types in `Joint`.
pub mod joint {
    #[derive(::rgz_derive::GzMessage)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Gearbox {
        /// / \brief Gearbox joint reference body link
        #[prost(string, tag = "1")]
        pub gearbox_reference_body: ::prost::alloc::string::String,
        /// / \brief Gearbox ratio.
        #[prost(double, tag = "2")]
        pub gearbox_ratio: f64,
    }
    #[derive(::rgz_derive::GzMessage)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Screw {
        /// / \brief Screw joint thread pitch.
        #[prost(double, tag = "1")]
        pub thread_pitch: f64,
    }
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Type {
        Revolute = 0,
        Revolute2 = 1,
        Prismatic = 2,
        Universal = 3,
        Ball = 4,
        Screw = 5,
        Gearbox = 6,
        Fixed = 7,
        Continuous = 8,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Revolute => "REVOLUTE",
                Type::Revolute2 => "REVOLUTE2",
                Type::Prismatic => "PRISMATIC",
                Type::Universal => "UNIVERSAL",
                Type::Ball => "BALL",
                Type::Screw => "SCREW",
                Type::Gearbox => "GEARBOX",
                Type::Fixed => "FIXED",
                Type::Continuous => "CONTINUOUS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "REVOLUTE" => Some(Self::Revolute),
                "REVOLUTE2" => Some(Self::Revolute2),
                "PRISMATIC" => Some(Self::Prismatic),
                "UNIVERSAL" => Some(Self::Universal),
                "BALL" => Some(Self::Ball),
                "SCREW" => Some(Self::Screw),
                "GEARBOX" => Some(Self::Gearbox),
                "FIXED" => Some(Self::Fixed),
                "CONTINUOUS" => Some(Self::Continuous),
                _ => None,
            }
        }
    }
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Inertial {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief Mass in kg
    #[prost(double, tag = "2")]
    pub mass: f64,
    /// / \brief CoM pose with respect to the link origin. In meters.
    #[prost(message, optional, tag = "3")]
    pub pose: ::core::option::Option<Pose>,
    /// / \brief Inertia matrix's XX element, in kg * m^2.
    #[prost(double, tag = "4")]
    pub ixx: f64,
    /// / \brief Inertia matrix's XY element, in kg * m^2.
    #[prost(double, tag = "5")]
    pub ixy: f64,
    /// / \brief Inertia matrix's XZ element, in kg * m^2.
    #[prost(double, tag = "6")]
    pub ixz: f64,
    /// / \brief Inertia matrix's YY element, in kg * m^2.
    #[prost(double, tag = "7")]
    pub iyy: f64,
    /// / \brief Inertia matrix's YZ element, in kg * m^2.
    #[prost(double, tag = "8")]
    pub iyz: f64,
    /// / \brief Inertia matrix's ZZ element, in kg * m^2.
    #[prost(double, tag = "9")]
    pub izz: f64,
    /// / \brief Fluid added mass matrix. The matrix is symmetric, so only the 21
    /// / elements of the top-half need to be set, organized as follows:
    /// /
    /// / 00, 01, 02, 03, 04, 05,
    /// /     11, 12, 13, 14, 15,
    /// /         22, 23, 24, 25,
    /// /             33, 34, 35,
    /// /                 44, 45,
    /// /                     55,
    /// /
    /// / Elements on the top-left 3x3 corner are in kg, the bottom-right ones are
    /// / in kg * m^2, and the rest are in kg * m.
    #[prost(double, repeated, tag = "10")]
    pub fluid_added_mass: ::prost::alloc::vec::Vec<f64>,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Boolean {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief Boolean data
    #[prost(bool, tag = "2")]
    pub data: bool,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Float {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief Float data
    #[prost(float, tag = "2")]
    pub data: f32,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StringMsg {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    #[prost(string, tag = "2")]
    pub data: ::prost::alloc::string::String,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParticleEmitter {
    /// / \brief Optional header data.
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief The emitter name.
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// / \brief Unique Id.
    #[prost(uint32, tag = "3")]
    pub id: u32,
    /// / \brief The emitter type.
    #[prost(enumeration = "particle_emitter::EmitterType", tag = "4")]
    pub r#type: i32,
    /// / \brief The position of the emitter.
    #[prost(message, optional, tag = "5")]
    pub pose: ::core::option::Option<Pose>,
    /// / The size of the emitter where the particles are sampled.
    #[prost(message, optional, tag = "6")]
    pub size: ::core::option::Option<Vector3d>,
    /// / \brief How many particles per second should be emitted.
    #[prost(message, optional, tag = "7")]
    pub rate: ::core::option::Option<Float>,
    /// / \brief The number of seconds the emitter is active.
    #[prost(message, optional, tag = "8")]
    pub duration: ::core::option::Option<Float>,
    /// / \brief Whether particle emitter is enabled or not.
    #[prost(message, optional, tag = "9")]
    pub emitting: ::core::option::Option<Boolean>,
    /// / \brief The particle dimensions (width, height, depth).
    #[prost(message, optional, tag = "10")]
    pub particle_size: ::core::option::Option<Vector3d>,
    /// / \brief The number of seconds each particle will ’live’ for before
    /// / being destroyed.
    #[prost(message, optional, tag = "11")]
    pub lifetime: ::core::option::Option<Float>,
    /// / \brief The material which all particles in the emitter will use.
    #[prost(message, optional, tag = "12")]
    pub material: ::core::option::Option<Material>,
    /// / \brief The minimum velocity each particle is emitted (m/s).
    #[prost(message, optional, tag = "13")]
    pub min_velocity: ::core::option::Option<Float>,
    /// / \brief The maximum velocity each particle is emitted (m/s).
    #[prost(message, optional, tag = "14")]
    pub max_velocity: ::core::option::Option<Float>,
    /// / \brief The starting color of the particles.
    #[prost(message, optional, tag = "15")]
    pub color_start: ::core::option::Option<Color>,
    /// / \brief The end color of the particles.
    #[prost(message, optional, tag = "16")]
    pub color_end: ::core::option::Option<Color>,
    /// / \brief The amount by which to scale the particles in both x and y
    /// / direction per second (screen coordinates).
    #[prost(message, optional, tag = "17")]
    pub scale_rate: ::core::option::Option<Float>,
    /// / \brief The path to the color image used as an affector.
    #[prost(message, optional, tag = "18")]
    pub color_range_image: ::core::option::Option<StringMsg>,
    /// / \brief The topic name used by the particle emitter for control and
    /// / modification.
    #[prost(message, optional, tag = "19")]
    pub topic: ::core::option::Option<StringMsg>,
    /// / \brief The ratio of particles that will be detected by sensors.
    /// / Increasing the ratio means there is a higher chance of particles
    /// / reflecting and interfering with depth sensing, making the emitter
    /// / appear more dense. Decreasing the ratio decreases the chance of
    /// / particles reflecting and interfering with depth sensing, making it
    /// / appear less dense. Value is in the range of \[0, 1\].
    #[prost(message, optional, tag = "20")]
    pub particle_scatter_ratio: ::core::option::Option<Float>,
}
/// Nested message and enum types in `ParticleEmitter`.
pub mod particle_emitter {
    /// / \brief All possible emitter types.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum EmitterType {
        /// / \brief Point emitter.
        Point = 0,
        /// / \brief Box emitter.
        Box = 1,
        /// / \brief Cylinder emitter.
        Cylinder = 2,
        /// / \brief Ellipsoid emitter.
        Ellipsoid = 3,
    }
    impl EmitterType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                EmitterType::Point => "POINT",
                EmitterType::Box => "BOX",
                EmitterType::Cylinder => "CYLINDER",
                EmitterType::Ellipsoid => "ELLIPSOID",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "POINT" => Some(Self::Point),
                "BOX" => Some(Self::Box),
                "CYLINDER" => Some(Self::Cylinder),
                "ELLIPSOID" => Some(Self::Ellipsoid),
                _ => None,
            }
        }
    }
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Projector {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub texture: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub pose: ::core::option::Option<Pose>,
    #[prost(double, tag = "5")]
    pub fov: f64,
    #[prost(double, tag = "6")]
    pub near_clip: f64,
    #[prost(double, tag = "7")]
    pub far_clip: f64,
    #[prost(bool, tag = "8")]
    pub enabled: bool,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Battery {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief Name of the battery
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// / \brief Real voltage in volts.
    #[prost(double, tag = "3")]
    pub voltage: f64,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Density {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    #[prost(double, tag = "2")]
    pub density: f64,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Link {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    #[prost(uint32, tag = "2")]
    pub id: u32,
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    #[prost(bool, tag = "4")]
    pub self_collide: bool,
    #[prost(bool, tag = "5")]
    pub gravity: bool,
    #[prost(bool, tag = "6")]
    pub kinematic: bool,
    #[prost(bool, tag = "7")]
    pub enabled: bool,
    #[prost(message, optional, tag = "8")]
    pub density: ::core::option::Option<Density>,
    #[prost(message, optional, tag = "9")]
    pub inertial: ::core::option::Option<Inertial>,
    #[prost(message, optional, tag = "10")]
    pub pose: ::core::option::Option<Pose>,
    #[prost(message, repeated, tag = "11")]
    pub visual: ::prost::alloc::vec::Vec<Visual>,
    #[prost(message, repeated, tag = "12")]
    pub collision: ::prost::alloc::vec::Vec<Collision>,
    #[prost(message, repeated, tag = "13")]
    pub sensor: ::prost::alloc::vec::Vec<Sensor>,
    #[prost(message, repeated, tag = "14")]
    pub projector: ::prost::alloc::vec::Vec<Projector>,
    #[prost(bool, tag = "15")]
    pub canonical: bool,
    /// / \brief A vector of batteries attached to this link.
    #[prost(message, repeated, tag = "16")]
    pub battery: ::prost::alloc::vec::Vec<Battery>,
    /// / \brief A vector of lights attached to this link
    #[prost(message, repeated, tag = "17")]
    pub light: ::prost::alloc::vec::Vec<Light>,
    /// / \brief A vector of particle emitters attached to this link.
    #[prost(message, repeated, tag = "18")]
    pub particle_emitter: ::prost::alloc::vec::Vec<ParticleEmitter>,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Model {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief Name of the model.
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// / \brief Unique ID associated with the model
    #[prost(uint32, tag = "3")]
    pub id: u32,
    /// / \brief True if the model is statc.
    #[prost(bool, tag = "4")]
    pub is_static: bool,
    /// / \brief Pose of the model.
    #[prost(message, optional, tag = "5")]
    pub pose: ::core::option::Option<Pose>,
    /// / \brief Information about the joints in this model.
    #[prost(message, repeated, tag = "6")]
    pub joint: ::prost::alloc::vec::Vec<Joint>,
    /// / \brief Information about the links in this model.
    #[prost(message, repeated, tag = "7")]
    pub link: ::prost::alloc::vec::Vec<Link>,
    /// / \brief True if the model was deleted.
    #[prost(bool, tag = "8")]
    pub deleted: bool,
    /// / \brief Information about the visuals in this model.
    #[prost(message, repeated, tag = "9")]
    pub visual: ::prost::alloc::vec::Vec<Visual>,
    /// / \brief Scaling factor applied to the model
    #[prost(message, optional, tag = "10")]
    pub scale: ::core::option::Option<Vector3d>,
    /// / \brief True if self collide is enabled.
    #[prost(bool, tag = "11")]
    pub self_collide: bool,
    /// / \brief An array of nested models.
    #[prost(message, repeated, tag = "12")]
    pub model: ::prost::alloc::vec::Vec<Model>,
    /// / \brief Axis aligned bounding box for the model. The center of the
    /// / bounding box should coincide with the model's pose.
    #[prost(message, optional, tag = "13")]
    pub bounding_box: ::core::option::Option<AxisAlignedBox>,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Entity {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief Entity unique identifier across all types. Defaults to null
    /// / entity (0).
    #[prost(uint64, tag = "2")]
    pub id: u64,
    /// / \brief Entity name, which is not guaranteed to be unique.
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    /// / \brief Entity type.
    #[prost(enumeration = "entity::Type", tag = "4")]
    pub r#type: i32,
}
/// Nested message and enum types in `Entity`.
pub mod entity {
    /// / \brief Entity type
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Type {
        /// / \brief No type specified
        None = 0,
        /// / \brief Light
        Light = 1,
        /// / \brief Model
        Model = 2,
        /// / \brief Link
        Link = 3,
        /// / \brief Visual
        Visual = 4,
        /// / \brief Collision
        Collision = 5,
        /// / \brief Sensor
        Sensor = 6,
        /// / \brief Joint
        Joint = 7,
        /// / \brief Actor
        Actor = 8,
        /// / \brief World
        World = 9,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::None => "NONE",
                Type::Light => "LIGHT",
                Type::Model => "MODEL",
                Type::Link => "LINK",
                Type::Visual => "VISUAL",
                Type::Collision => "COLLISION",
                Type::Sensor => "SENSOR",
                Type::Joint => "JOINT",
                Type::Actor => "ACTOR",
                Type::World => "WORLD",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "NONE" => Some(Self::None),
                "LIGHT" => Some(Self::Light),
                "MODEL" => Some(Self::Model),
                "LINK" => Some(Self::Link),
                "VISUAL" => Some(Self::Visual),
                "COLLISION" => Some(Self::Collision),
                "SENSOR" => Some(Self::Sensor),
                "JOINT" => Some(Self::Joint),
                "ACTOR" => Some(Self::Actor),
                "WORLD" => Some(Self::World),
                _ => None,
            }
        }
    }
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SphericalCoordinates {
    /// / \brief Optional header data.
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief Planetary surface model.
    #[prost(enumeration = "spherical_coordinates::SurfaceModel", tag = "2")]
    pub surface_model: i32,
    /// / \brief Latitude in degrees.
    #[prost(double, tag = "3")]
    pub latitude_deg: f64,
    /// / \brief Longitude in degrees.
    #[prost(double, tag = "4")]
    pub longitude_deg: f64,
    /// / \brief Elevation in meters.
    #[prost(double, tag = "5")]
    pub elevation: f64,
    /// / \brief Heading in degrees.
    #[prost(double, tag = "6")]
    pub heading_deg: f64,
    /// / \brief Entity that the coordinates apply to.
    /// / If not set, defaults to the world origin.
    #[prost(message, optional, tag = "7")]
    pub entity: ::core::option::Option<Entity>,
    /// / \brief Equatorial axis in meters.
    #[prost(double, tag = "8")]
    pub surface_axis_equatorial: f64,
    /// / \brief Polar axis in meters.
    #[prost(double, tag = "9")]
    pub surface_axis_polar: f64,
}
/// Nested message and enum types in `SphericalCoordinates`.
pub mod spherical_coordinates {
    /// / \brief Planetary surface models.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum SurfaceModel {
        /// / \brief World Geodetic System 1984
        EarthWgs84 = 0,
        /// / \brief Model of the moon, based on the Selenographic
        /// / coordinate system, see wikipedia: Selenographic
        /// / Coordinate System.
        MoonScs = 1,
        /// / \brief Custom surface type
        CustomSurface = 2,
    }
    impl SurfaceModel {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SurfaceModel::EarthWgs84 => "EARTH_WGS84",
                SurfaceModel::MoonScs => "MOON_SCS",
                SurfaceModel::CustomSurface => "CUSTOM_SURFACE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "EARTH_WGS84" => Some(Self::EarthWgs84),
                "MOON_SCS" => Some(Self::MoonScs),
                "CUSTOM_SURFACE" => Some(Self::CustomSurface),
                _ => None,
            }
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SphericalCoordinatesType {
    /// / \brief Latitude, Longitude and Altitude by SurfaceType
    Spherical = 0,
    /// / \brief Earth centered, earth fixed Cartesian
    Ecef = 1,
    /// / \brief Local tangent plane (East, North, Up)
    Global = 2,
    /// / \brief Heading-adjusted tangent plane (X, Y, Z)
    /// / This has kept a bug for backwards compatibility, use
    /// / LOCAL2 for the correct behaviour.
    Local = 3,
    /// / \brief Heading-adjusted tangent plane (X, Y, Z)
    Local2 = 4,
}
impl SphericalCoordinatesType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SphericalCoordinatesType::Spherical => "SPHERICAL",
            SphericalCoordinatesType::Ecef => "ECEF",
            SphericalCoordinatesType::Global => "GLOBAL",
            SphericalCoordinatesType::Local => "LOCAL",
            SphericalCoordinatesType::Local2 => "LOCAL2",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SPHERICAL" => Some(Self::Spherical),
            "ECEF" => Some(Self::Ecef),
            "GLOBAL" => Some(Self::Global),
            "LOCAL" => Some(Self::Local),
            "LOCAL2" => Some(Self::Local2),
            _ => None,
        }
    }
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EntityFactory {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief Pose where the entity will be spawned in the world.
    /// / If set, `spherical_coordinates` will be ignored.
    #[prost(message, optional, tag = "7")]
    pub pose: ::core::option::Option<Pose>,
    /// / \brief New name for the entity, overrides the name on the SDF.
    #[prost(string, tag = "8")]
    pub name: ::prost::alloc::string::String,
    /// / \brief Whether the server is allowed to rename the entity in case of
    /// / overlap with existing entities.
    #[prost(bool, tag = "9")]
    pub allow_renaming: bool,
    /// / \brief The pose will be defined relative to this frame. If left empty,
    /// / the "world" frame will be used.
    #[prost(string, tag = "10")]
    pub relative_to: ::prost::alloc::string::String,
    /// / \brief Spherical coordinates where the entity will be spawned in the
    /// / world.
    /// / If `pose` is also set:
    /// / * `pose.position` is ignored in favor of latitude, longitude and
    /// /   elevation.
    /// / * `pose.orientation` is used in conjunction with heading:
    /// /       Quaternion::fromEuler(0, 0, heading) * pose.orientation
    #[prost(message, optional, tag = "11")]
    pub spherical_coordinates: ::core::option::Option<SphericalCoordinates>,
    /// / \brief Only one method is supported at a time
    #[prost(oneof = "entity_factory::From", tags = "2, 3, 4, 5, 6")]
    pub from: ::core::option::Option<entity_factory::From>,
}
/// Nested message and enum types in `EntityFactory`.
pub mod entity_factory {
    /// / \brief Only one method is supported at a time
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum From {
        /// / \brief SDF description in string format.
        #[prost(string, tag = "2")]
        Sdf(::prost::alloc::string::String),
        /// / \brief Full path to SDF file.
        #[prost(string, tag = "3")]
        SdfFilename(::prost::alloc::string::String),
        /// / \brief Description of model to be inserted.
        #[prost(message, tag = "4")]
        Model(super::Model),
        /// / \brief Description of light to be inserted.
        #[prost(message, tag = "5")]
        Light(super::Light),
        /// / \brief Name of entity to clone.
        #[prost(string, tag = "6")]
        CloneName(::prost::alloc::string::String),
    }
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EntityFactoryV {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief The set of entity factory messages.
    #[prost(message, repeated, tag = "2")]
    pub data: ::prost::alloc::vec::Vec<EntityFactory>,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PluginV {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief Plugin messages.
    #[prost(message, repeated, tag = "2")]
    pub plugins: ::prost::alloc::vec::Vec<Plugin>,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Request {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    #[prost(int32, tag = "2")]
    pub id: i32,
    #[prost(string, tag = "3")]
    pub request: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub data: ::prost::alloc::string::String,
    #[prost(double, tag = "5")]
    pub dbl_data: f64,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RaySensor {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    #[prost(bool, tag = "2")]
    pub display_scan: bool,
    #[prost(int32, tag = "3")]
    pub horizontal_samples: i32,
    #[prost(double, tag = "4")]
    pub horizontal_resolution: f64,
    #[prost(double, tag = "5")]
    pub horizontal_min_angle: f64,
    #[prost(double, tag = "6")]
    pub horizontal_max_angle: f64,
    #[prost(int32, tag = "7")]
    pub vertical_samples: i32,
    #[prost(double, tag = "8")]
    pub vertical_resolution: f64,
    #[prost(double, tag = "9")]
    pub vertical_min_angle: f64,
    #[prost(double, tag = "10")]
    pub vertical_max_angle: f64,
    #[prost(double, tag = "11")]
    pub range_min: f64,
    #[prost(double, tag = "12")]
    pub range_max: f64,
    #[prost(double, tag = "13")]
    pub range_resolution: f64,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Oriented3DBox {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief Center of the bounding box in 3d camera coordinates
    #[prost(message, optional, tag = "2")]
    pub center: ::core::option::Option<Vector3d>,
    /// / \brief Orientation of the bounding box in 3d camera coordinates
    #[prost(message, optional, tag = "3")]
    pub orientation: ::core::option::Option<Quaternion>,
    /// / \brief The size of the bounding box on XYZ (width/height/depth)
    #[prost(message, optional, tag = "4")]
    pub box_size: ::core::option::Option<Vector3d>,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnnotatedOriented3DBox {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief oreinted 3d box
    #[prost(message, optional, tag = "2")]
    pub r#box: ::core::option::Option<Oriented3DBox>,
    /// / \brief Label (class) of the box's object
    #[prost(uint32, tag = "3")]
    pub label: u32,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogStatus {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    #[prost(message, optional, tag = "2")]
    pub sim_time: ::core::option::Option<Time>,
    #[prost(message, optional, tag = "3")]
    pub log_file: ::core::option::Option<log_status::LogFile>,
}
/// Nested message and enum types in `LogStatus`.
pub mod log_status {
    #[derive(::rgz_derive::GzMessage)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LogFile {
        #[prost(string, tag = "1")]
        pub uri: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub base_path: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub full_path: ::prost::alloc::string::String,
        #[prost(float, tag = "4")]
        pub size: f32,
        #[prost(enumeration = "log_file::Units", tag = "5")]
        pub size_units: i32,
        #[prost(bool, tag = "6")]
        pub record_resources: bool,
    }
    /// Nested message and enum types in `LogFile`.
    pub mod log_file {
        #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            ::prost::Enumeration
        )]
        #[repr(i32)]
        pub enum Units {
            Bytes = 0,
            KBytes = 1,
            MBytes = 2,
            GBytes = 3,
        }
        impl Units {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Units::Bytes => "BYTES",
                    Units::KBytes => "K_BYTES",
                    Units::MBytes => "M_BYTES",
                    Units::GBytes => "G_BYTES",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "BYTES" => Some(Self::Bytes),
                    "K_BYTES" => Some(Self::KBytes),
                    "M_BYTES" => Some(Self::MBytes),
                    "G_BYTES" => Some(Self::GBytes),
                    _ => None,
                }
            }
        }
    }
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AxisAligned2DBox {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief Minimum corner of the axis aligned bound box in image coord.
    #[prost(message, optional, tag = "2")]
    pub min_corner: ::core::option::Option<Vector2d>,
    /// / \brief Maximum corner of the axis aligned bound box in the image coord.
    #[prost(message, optional, tag = "3")]
    pub max_corner: ::core::option::Option<Vector2d>,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnnotatedAxisAligned2DBox {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief Axis aligned 2d bounding box
    #[prost(message, optional, tag = "2")]
    pub r#box: ::core::option::Option<AxisAligned2DBox>,
    /// / \brief Label (class) of the box's object
    #[prost(uint32, tag = "3")]
    pub label: u32,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnnotatedAxisAligned2DBoxV {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief vector of 2d annotated boxes
    #[prost(message, repeated, tag = "2")]
    pub annotated_box: ::prost::alloc::vec::Vec<AnnotatedAxisAligned2DBox>,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JointAnimation {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    #[prost(string, tag = "2")]
    pub model_name: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub joint: ::prost::alloc::vec::Vec<joint_animation::Joint>,
    #[prost(message, repeated, tag = "4")]
    pub time: ::prost::alloc::vec::Vec<Time>,
}
/// Nested message and enum types in `JointAnimation`.
pub mod joint_animation {
    #[derive(::rgz_derive::GzMessage)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Joint {
        #[prost(string, repeated, tag = "1")]
        pub name: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        #[prost(double, repeated, tag = "2")]
        pub angle: ::prost::alloc::vec::Vec<f64>,
    }
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParameterDeclaration {
    /// / \brief Parameter name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// / \brief Parameter type, i.e. the associated protobuf type.
    #[prost(string, tag = "2")]
    pub r#type: ::prost::alloc::string::String,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParameterDeclarations {
    /// / \brief Parameter declarations.
    #[prost(message, repeated, tag = "1")]
    pub parameter_declarations: ::prost::alloc::vec::Vec<ParameterDeclaration>,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Duration {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief Seconds
    #[prost(int64, tag = "2")]
    pub sec: i64,
    /// / \brief Nanoseconds
    #[prost(int32, tag = "3")]
    pub nsec: i32,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JointTrajectoryPoint {
    /// / \brief Position of each joint relative to their "0" position. Units are
    /// / dependent on the joint type, where radians are used for revolute or
    /// / continuous joints, and meters for prismatic joints
    #[prost(double, repeated, tag = "1")]
    pub positions: ::prost::alloc::vec::Vec<f64>,
    /// / \brief Rate of change in position of each joint. Units are dependent on
    /// / the joint type, where radians/second are used for revolute or continuous
    /// / joints, and meters/second for prismatic joints
    #[prost(double, repeated, tag = "2")]
    pub velocities: ::prost::alloc::vec::Vec<f64>,
    /// / \brief Rate of change in velocity of each joint. Units are dependent on
    /// / the joint type, where radians/second^2 are used for revolute or
    /// / continuous joints, and meters/second^2 for prismatic joints
    #[prost(double, repeated, tag = "3")]
    pub accelerations: ::prost::alloc::vec::Vec<f64>,
    /// / \brief Torque or force applied at each joint. Units are dependent on the
    /// / joint type, where newton-meters are used for revolute or continuous
    /// / joints (torque), and newtons for prismatic joints (force)
    #[prost(double, repeated, tag = "4")]
    pub effort: ::prost::alloc::vec::Vec<f64>,
    /// / \brief Desired time from the beginning of trajectory execution until
    /// / this trajectory point should be reached. This value must be strictly
    /// / increasing for consecutive points
    #[prost(message, optional, tag = "5")]
    pub time_from_start: ::core::option::Option<Duration>,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JointTrajectory {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief Ordered list of joint names that must be active during execution
    /// / of this trajectory. The order shall correspond to the values in each
    /// / trajectory point
    #[prost(string, repeated, tag = "2")]
    pub joint_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// / \brief Ordered list of time-parameterised trajectory points, which can
    /// / describe positions, velocities, accelerations and/or effort for all
    /// / active joints at each point in time. All points must be ordered
    /// / according to their time from start, which must be strictly increasing
    #[prost(message, repeated, tag = "3")]
    pub points: ::prost::alloc::vec::Vec<JointTrajectoryPoint>,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SensorV {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief Sensor messages.
    #[prost(message, repeated, tag = "2")]
    pub sensors: ::prost::alloc::vec::Vec<Sensor>,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatteryState {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief Voltage in Volts
    #[prost(double, tag = "2")]
    pub voltage: f64,
    /// / \brief Current draw in Ampere
    #[prost(double, tag = "3")]
    pub current: f64,
    /// / \brief Amount of charge in the battery in Ah
    #[prost(double, tag = "4")]
    pub charge: f64,
    /// / \brief Capacity in Ah
    #[prost(double, tag = "5")]
    pub capacity: f64,
    /// / \brief Percentage of charge left
    #[prost(double, tag = "6")]
    pub percentage: f64,
    /// / \brief The charging status
    #[prost(enumeration = "battery_state::PowerSupplyStatus", tag = "7")]
    pub power_supply_status: i32,
}
/// Nested message and enum types in `BatteryState`.
pub mod battery_state {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum PowerSupplyStatus {
        Unknown = 0,
        Charging = 1,
        Discharging = 2,
        NotCharging = 3,
        Full = 4,
    }
    impl PowerSupplyStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                PowerSupplyStatus::Unknown => "UNKNOWN",
                PowerSupplyStatus::Charging => "CHARGING",
                PowerSupplyStatus::Discharging => "DISCHARGING",
                PowerSupplyStatus::NotCharging => "NOT_CHARGING",
                PowerSupplyStatus::Full => "FULL",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNKNOWN" => Some(Self::Unknown),
                "CHARGING" => Some(Self::Charging),
                "DISCHARGING" => Some(Self::Discharging),
                "NOT_CHARGING" => Some(Self::NotCharging),
                "FULL" => Some(Self::Full),
                _ => None,
            }
        }
    }
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Version {
    /// / \brief Major version.
    #[prost(int32, tag = "1")]
    pub major: i32,
    /// / \brief Minor version.
    #[prost(int32, tag = "2")]
    pub minor: i32,
    /// / \brief Patch version.
    #[prost(int32, tag = "3")]
    pub patch: i32,
    /// / \brief Pre-release version.
    #[prost(string, tag = "4")]
    pub prerelease: ::prost::alloc::string::String,
    /// / \brief Build version.
    #[prost(string, tag = "5")]
    pub build: ::prost::alloc::string::String,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VersionedName {
    /// / \brief Version information.
    #[prost(message, optional, tag = "1")]
    pub version: ::core::option::Option<Version>,
    /// / \brief Name associated with the version.
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WirelessNode {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    #[prost(string, tag = "2")]
    pub essid: ::prost::alloc::string::String,
    #[prost(double, tag = "3")]
    pub frequency: f64,
    #[prost(double, tag = "4")]
    pub signal_level: f64,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrackVisual {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief Name of the visual to track
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// / \brief Id of the visual to track
    #[prost(uint32, tag = "3")]
    pub id: u32,
    /// / \brief True to have the tracking camera inherit the orientation of
    /// / the tracked visual.
    #[prost(bool, tag = "4")]
    pub inherit_orientation: bool,
    /// / \brief Minimum follow distance
    #[prost(double, tag = "5")]
    pub min_dist: f64,
    /// / \brief Maximum follow distance
    #[prost(double, tag = "6")]
    pub max_dist: f64,
    /// / \brief If set to true, the position of the camera is fixed.
    #[prost(bool, tag = "7")]
    pub r#static: bool,
    /// / \brief If set to true, the position of the camera is relative to the
    /// / model reference frame.
    #[prost(bool, tag = "8")]
    pub use_model_frame: bool,
    /// / \brief Position of the camera.
    #[prost(message, optional, tag = "9")]
    pub xyz: ::core::option::Option<Vector3d>,
    /// / \brief If set to true, the camera inherits the yaw rotation of the model.
    #[prost(bool, tag = "10")]
    pub inherit_yaw: bool,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GuiCamera {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub view_controller: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub pose: ::core::option::Option<Pose>,
    #[prost(message, optional, tag = "5")]
    pub track: ::core::option::Option<TrackVisual>,
    /// / \brief Type of projection: "perspective" or "orthographic".
    #[prost(string, tag = "6")]
    pub projection_type: ::prost::alloc::string::String,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogPlaybackStatistics {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief Log start time
    #[prost(message, optional, tag = "2")]
    pub start_time: ::core::option::Option<Time>,
    /// / \brief Log end time
    #[prost(message, optional, tag = "3")]
    pub end_time: ::core::option::Option<Time>,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorldStatistics {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief Current simulation time
    #[prost(message, optional, tag = "2")]
    pub sim_time: ::core::option::Option<Time>,
    /// / \brief Total time spent paused
    #[prost(message, optional, tag = "3")]
    pub pause_time: ::core::option::Option<Time>,
    /// / \brief Current real time
    #[prost(message, optional, tag = "4")]
    pub real_time: ::core::option::Option<Time>,
    /// / \brief Whether currently paused
    #[prost(bool, tag = "5")]
    pub paused: bool,
    /// / \brief Current iteration count
    #[prost(uint64, tag = "6")]
    pub iterations: u64,
    /// / \brief Total number of models in the world
    #[prost(int32, tag = "7")]
    pub model_count: i32,
    /// / \brief Statistics for log playback
    #[prost(message, optional, tag = "8")]
    pub log_playback_stats: ::core::option::Option<LogPlaybackStatistics>,
    /// / \brief This factor expresses how much real time elapses with each step
    /// / of simulation time.
    /// / E.g.: 0.5 means that 1 second in real time takes 2 seconds in simulation.
    #[prost(double, tag = "9")]
    pub real_time_factor: f64,
    /// / \brief Iteration step size. It's zero when paused.
    #[prost(message, optional, tag = "10")]
    pub step_size: ::core::option::Option<Time>,
    /// / \brief True if the simulator is stepping, as opposed to running. This
    /// / field can be ignored if the paused field is true.
    /// /
    /// / When `paused == true`, then simulation is paused and not updating at all
    /// / and the `stepping` field can be ignored.
    /// /
    /// / When `paused == false` and
    /// /   * `stepping == true` then Simulation is updating for a fixed number of
    /// /     iterations, or
    /// /   * `stepping == false` then Simulation is running for an unbounded
    /// /     number of iterations.
    #[prost(bool, tag = "11")]
    pub stepping: bool,
}
/// / \brief Holds all the information needed to reconstruct a component.
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SerializedComponent {
    /// / \brief Unique ID that represents a component's type.
    #[prost(uint64, tag = "1")]
    pub r#type: u64,
    /// / \brief Component's serialized data.
    #[prost(bytes = "vec", tag = "2")]
    pub component: ::prost::alloc::vec::Vec<u8>,
    /// / \brief Whether the component should be removed at the current state.
    #[prost(bool, tag = "3")]
    pub remove: bool,
}
/// / \brief Holds all the information needed to reconstruct an entity and its
/// / components.
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SerializedEntity {
    /// / \brief The entity is uniquely identified by its ID.
    #[prost(uint64, tag = "1")]
    pub id: u64,
    /// / \brief All the components belonging to the entity.
    #[prost(message, repeated, tag = "2")]
    pub components: ::prost::alloc::vec::Vec<SerializedComponent>,
    /// / \brief Whether the entity and all its components should be removed at the
    /// / current state.
    #[prost(bool, tag = "3")]
    pub remove: bool,
}
/// / \brief Holds all the information needed to reconstruct the state of an
/// / entity-component-system (ECS) architecture at a given time.
/// / An ECS's state consists of several entities, each with an arbitrary number
/// / of components tied to them.
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SerializedState {
    /// / \brief Header data, which contains the simulation time.
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief All the entities currently in the simulation.
    #[prost(message, repeated, tag = "2")]
    pub entities: ::prost::alloc::vec::Vec<SerializedEntity>,
}
/// / \brief All the data needed to step an ECS system, such as current
/// / simulation time and entity states.
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SerializedStep {
    /// / \brief Iteration information, such as sim time and paused state.
    #[prost(message, optional, tag = "1")]
    pub stats: ::core::option::Option<WorldStatistics>,
    /// / \brief State of entities and components.
    #[prost(message, optional, tag = "2")]
    pub state: ::core::option::Option<SerializedState>,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SdfGeneratorConfig {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief Global setting for SDFormat generation of entities
    #[prost(message, optional, tag = "2")]
    pub global_entity_gen_config: ::core::option::Option<
        sdf_generator_config::EntityGeneratorConfig,
    >,
    /// / \brief Per-entity override of global settings for SDFormat generation.
    /// / The key is the scoped name of an entity.
    #[prost(map = "string, message", tag = "3")]
    pub override_entity_gen_configs: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        sdf_generator_config::EntityGeneratorConfig,
    >,
}
/// Nested message and enum types in `SdfGeneratorConfig`.
pub mod sdf_generator_config {
    /// / \brief Configuration for SDFormat generation of entities (eg. models, actors, lights)
    #[derive(::rgz_derive::GzMessage)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EntityGeneratorConfig {
        /// / \brief Expand and inline included entities
        #[prost(message, optional, tag = "1")]
        pub expand_include_tags: ::core::option::Option<super::Boolean>,
        /// / \brief Use the Fuel version in generated URIs of Fuel resources
        #[prost(message, optional, tag = "2")]
        pub save_fuel_version: ::core::option::Option<super::Boolean>,
        /// / \brief Use absolute paths for resources such as meshes
        #[prost(message, optional, tag = "3")]
        pub resources_use_absolute_paths: ::core::option::Option<super::Boolean>,
        /// / \brief Copy model resources, such as meshes, and create a self contained
        /// / model.
        #[prost(message, optional, tag = "4")]
        pub copy_model_resources: ::core::option::Option<super::Boolean>,
    }
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SimEvent {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief ID of this event message
    #[prost(uint32, tag = "2")]
    pub id: u32,
    /// / \brief Type of sim event
    #[prost(string, tag = "3")]
    pub r#type: ::prost::alloc::string::String,
    /// / \brief Name of sim event
    #[prost(string, tag = "4")]
    pub name: ::prost::alloc::string::String,
    /// / \brief Statistics of the world
    #[prost(message, optional, tag = "5")]
    pub world_statistics: ::core::option::Option<WorldStatistics>,
    /// / \brief Data describing the sim event
    #[prost(string, tag = "6")]
    pub data: ::prost::alloc::string::String,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Int64 {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief Integer data
    #[prost(int64, tag = "2")]
    pub data: i64,
}
/// / \brief Altimeter sensor data
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Altimeter {
    /// Other Optional header data
    ///
    /// / Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief Vertical position data, in meters.
    #[prost(double, tag = "2")]
    pub vertical_position: f64,
    /// / \brief Vertical velocity data, in meters/second.
    #[prost(double, tag = "3")]
    pub vertical_velocity: f64,
    /// / \brief Vertical reference.
    #[prost(double, tag = "4")]
    pub vertical_reference: f64,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Road {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(double, tag = "3")]
    pub width: f64,
    #[prost(message, repeated, tag = "4")]
    pub point: ::prost::alloc::vec::Vec<Vector3d>,
    #[prost(message, optional, tag = "5")]
    pub material: ::core::option::Option<Material>,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NavSatSensor {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief Position sensing. Consists of horizontal and vertical noise
    /// / properties
    #[prost(message, optional, tag = "2")]
    pub position: ::core::option::Option<nav_sat_sensor::Sensing>,
    /// / \brief Velocity sensing. Consists of horizontal and vertical noise
    /// / properties
    #[prost(message, optional, tag = "3")]
    pub velocity: ::core::option::Option<nav_sat_sensor::Sensing>,
}
/// Nested message and enum types in `NavSatSensor`.
pub mod nav_sat_sensor {
    /// / \brief Sensing information
    #[derive(::rgz_derive::GzMessage)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Sensing {
        /// / \brief Horizontal noise
        #[prost(message, optional, tag = "1")]
        pub horizontal_noise: ::core::option::Option<super::SensorNoise>,
        /// / \brief Vertical noise
        #[prost(message, optional, tag = "2")]
        pub vertical_noise: ::core::option::Option<super::SensorNoise>,
    }
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorldReset {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    #[prost(bool, tag = "2")]
    pub all: bool,
    #[prost(bool, tag = "3")]
    pub time_only: bool,
    #[prost(bool, tag = "4")]
    pub model_only: bool,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorldControl {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    #[prost(bool, tag = "2")]
    pub pause: bool,
    #[prost(bool, tag = "3")]
    pub step: bool,
    #[prost(uint32, tag = "4")]
    pub multi_step: u32,
    #[prost(message, optional, tag = "5")]
    pub reset: ::core::option::Option<WorldReset>,
    #[prost(uint32, tag = "6")]
    pub seed: u32,
    /// \brief A simulation time in the future to run to and then pause.
    #[prost(message, optional, tag = "7")]
    pub run_to_sim_time: ::core::option::Option<Time>,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Wrench {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    #[prost(message, optional, tag = "2")]
    pub force: ::core::option::Option<Vector3d>,
    #[prost(message, optional, tag = "3")]
    pub torque: ::core::option::Option<Vector3d>,
    #[prost(message, optional, tag = "4")]
    pub force_offset: ::core::option::Option<Vector3d>,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserCmd {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief Unique id for user command.
    #[prost(uint32, tag = "2")]
    pub id: u32,
    /// / \brief Description for the command.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// / \brief Type of command.
    #[prost(enumeration = "user_cmd::Type", tag = "4")]
    pub r#type: i32,
    /// / \brief For model modify commands.
    #[prost(message, repeated, tag = "5")]
    pub model: ::prost::alloc::vec::Vec<Model>,
    /// / \brief For light modify commands.
    #[prost(message, repeated, tag = "6")]
    pub light: ::prost::alloc::vec::Vec<Light>,
    /// / \brief Name of entity targeted by command
    #[prost(string, tag = "7")]
    pub entity_name: ::prost::alloc::string::String,
    /// / \brief For World Control commands.
    #[prost(message, optional, tag = "8")]
    pub world_control: ::core::option::Option<WorldControl>,
    /// / \brief Wrench for apply wrench commands.
    #[prost(message, optional, tag = "9")]
    pub wrench: ::core::option::Option<Wrench>,
}
/// Nested message and enum types in `UserCmd`.
pub mod user_cmd {
    /// / \brief Types of user commands
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Type {
        /// / \brief Moving an entity.
        Moving = 0,
        /// / \brief Controlling the world.
        WorldControl = 1,
        /// / \brief Applying wrench.
        Wrench = 2,
        /// / \brief Scaling an entity.
        Scaling = 3,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Moving => "MOVING",
                Type::WorldControl => "WORLD_CONTROL",
                Type::Wrench => "WRENCH",
                Type::Scaling => "SCALING",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "MOVING" => Some(Self::Moving),
                "WORLD_CONTROL" => Some(Self::WorldControl),
                "WRENCH" => Some(Self::Wrench),
                "SCALING" => Some(Self::Scaling),
                _ => None,
            }
        }
    }
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserCmdStats {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief User commands in the undo list.
    #[prost(message, repeated, tag = "2")]
    pub undo_cmd: ::prost::alloc::vec::Vec<UserCmd>,
    /// / \brief User commands in the redo list.
    #[prost(message, repeated, tag = "3")]
    pub redo_cmd: ::prost::alloc::vec::Vec<UserCmd>,
}
/// / \brief Message that describes an air speed sensor.
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AirSpeedSensor {
    /// / \brief header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief Differential pressure (hPa).
    #[prost(double, tag = "2")]
    pub diff_pressure: f64,
    /// / \brief Temperature (kelvin).
    #[prost(double, tag = "3")]
    pub temperature: f64,
    /// / \brief Sensor speed noise.
    #[prost(message, optional, tag = "4")]
    pub pressure_noise: ::core::option::Option<SensorNoise>,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParameterName {
    /// / \brief Parameter name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnnotatedOriented3DBoxV {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief vector of 3d annotated oreinted boxes
    #[prost(message, repeated, tag = "2")]
    pub annotated_box: ::prost::alloc::vec::Vec<AnnotatedOriented3DBox>,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerControl {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    #[prost(string, tag = "2")]
    pub save_world_name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub save_filename: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub open_filename: ::prost::alloc::string::String,
    #[prost(bool, tag = "5")]
    pub new_world: bool,
    #[prost(bool, tag = "6")]
    pub stop: bool,
    #[prost(bool, tag = "7")]
    pub clone: bool,
    #[prost(uint32, tag = "8")]
    pub new_port: u32,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Gui {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    #[prost(bool, tag = "2")]
    pub fullscreen: bool,
    #[prost(message, optional, tag = "3")]
    pub camera: ::core::option::Option<GuiCamera>,
    #[prost(message, repeated, tag = "4")]
    pub plugin: ::prost::alloc::vec::Vec<Plugin>,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Test {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Dataframe {
    /// / \brief Header data.
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief Address of the sender.
    #[prost(string, tag = "2")]
    pub src_address: ::prost::alloc::string::String,
    /// / \brief Address of the destination.
    #[prost(string, tag = "3")]
    pub dst_address: ::prost::alloc::string::String,
    /// / \brief Payload.
    #[prost(bytes = "vec", tag = "4")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FloatV {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief Vector of float data
    #[prost(float, repeated, tag = "2")]
    pub data: ::prost::alloc::vec::Vec<f32>,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoseWithCovariance {
    /// / \brief Pose message.
    #[prost(message, optional, tag = "1")]
    pub pose: ::core::option::Option<Pose>,
    /// / \brief Row-major representation of the 6x6 covariance matrix
    /// / The orientation parameters use a fixed-axis representation.
    /// / In order, the parameters are:
    /// / (x, y, z, rotation about X axis, rotation about Y axis, rotation about Z axis)
    #[prost(message, optional, tag = "2")]
    pub covariance: ::core::option::Option<FloatV>,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NavSat {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief Latitude in degrees
    #[prost(double, tag = "2")]
    pub latitude_deg: f64,
    /// / \brief Longitude in degrees
    #[prost(double, tag = "3")]
    pub longitude_deg: f64,
    /// / \brief Altitude in meters
    #[prost(double, tag = "4")]
    pub altitude: f64,
    /// / \brief East velocity in the ENU frame, in m / s
    #[prost(double, tag = "5")]
    pub velocity_east: f64,
    /// / \brief North velocity in the ENU frame, in m / s
    #[prost(double, tag = "6")]
    pub velocity_north: f64,
    /// / \brief Up velocity in the ENU frame, in m / s
    #[prost(double, tag = "7")]
    pub velocity_up: f64,
    /// / \brief ID of reference frame
    #[prost(string, tag = "8")]
    pub frame_id: ::prost::alloc::string::String,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pid {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    #[prost(message, optional, tag = "2")]
    pub target_optional: ::core::option::Option<Double>,
    #[prost(message, optional, tag = "3")]
    pub p_gain_optional: ::core::option::Option<Double>,
    #[prost(message, optional, tag = "4")]
    pub i_gain_optional: ::core::option::Option<Double>,
    #[prost(message, optional, tag = "5")]
    pub d_gain_optional: ::core::option::Option<Double>,
    #[prost(message, optional, tag = "6")]
    pub i_max_optional: ::core::option::Option<Double>,
    #[prost(message, optional, tag = "7")]
    pub i_min_optional: ::core::option::Option<Double>,
    #[prost(message, optional, tag = "8")]
    pub limit_optional: ::core::option::Option<Double>,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WheelSlipParametersCmd {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief Entity which wheel slip parameters are going to be modified.
    /// /
    /// / The entity might be a model with at least one link or a link.
    /// / If the entity is a model, the wheel slip parameters of all its
    /// / links will be updated.
    /// /
    /// / The entity name (entity.name) will be used as an scoped name.
    /// / For example, in this
    /// / hierarchy:
    /// /
    /// / world_name
    /// /  model_name
    /// /    link_name
    /// /
    /// / All these names will return the link entity:
    /// /
    /// / * world_name::model_name::link_name
    /// / * model_name::link_name
    /// / * link_name
    #[prost(message, optional, tag = "2")]
    pub entity: ::core::option::Option<Entity>,
    /// / \brief Unitless lateral slip ratio.
    /// /
    /// / See <https://en.wikipedia.org/wiki/Slip_(vehicle_dynamics>).
    /// / to tangential force ratio (tangential / normal force).
    /// / At each time step, these compliances are multiplied by
    /// / the linear wheel spin velocity and divided by the wheel normal force
    /// / parameter specified in the sdf.
    #[prost(double, tag = "4")]
    pub slip_compliance_lateral: f64,
    /// / \brief Unitless longitudinal slip ratio.
    /// /
    /// / See <https://en.wikipedia.org/wiki/Slip_(vehicle_dynamics>).
    /// / to tangential force ratio (tangential / normal force).
    /// / At each time step, these compliances are multiplied by
    /// / the linear wheel spin velocity and divided by the wheel normal force
    /// / parameter specified in the sdf.
    #[prost(double, tag = "5")]
    pub slip_compliance_longitudinal: f64,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UInt32 {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief Integer data
    #[prost(uint32, tag = "2")]
    pub data: u32,
}
/// / \brief Message that encapsulates sensor data from a magnetometer.
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Magnetometer {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief Magnetic field strength (in Tesla) along body-frame axis
    #[prost(message, optional, tag = "2")]
    pub field_tesla: ::core::option::Option<Vector3d>,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Selection {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    #[prost(uint32, tag = "2")]
    pub id: u32,
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    #[prost(bool, tag = "4")]
    pub selected: bool,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Any {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief Type of value that is contained in this message.
    #[prost(enumeration = "any::ValueType", tag = "2")]
    pub r#type: i32,
    #[prost(oneof = "any::Value", tags = "3, 4, 5, 6, 7, 8, 9, 10, 11")]
    pub value: ::core::option::Option<any::Value>,
}
/// Nested message and enum types in `Any`.
pub mod any {
    /// / \brief The type of data the message contains.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum ValueType {
        /// / \brief Indicates that the message is empty
        None = 0,
        /// / \brief Indicates that the message contains a double
        Double = 1,
        /// / \brief Indicates that the message contains an int32
        Int32 = 2,
        /// / \brief Indicates that the message contains a string
        String = 3,
        /// / \brief Indicates that the message contains a Boolean
        Boolean = 4,
        /// / \brief Indicates that the message contains a Vector3d
        Vector3d = 5,
        /// / \brief Indicates that the message contains a Color
        Color = 6,
        /// / \brief Indicates that the message contains a Pose
        Pose3d = 7,
        /// / \brief Indicates that the message contains a Quaternion
        Quaterniond = 8,
        /// / \brief Indicates that the message contains a Time
        Time = 9,
    }
    impl ValueType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ValueType::None => "NONE",
                ValueType::Double => "DOUBLE",
                ValueType::Int32 => "INT32",
                ValueType::String => "STRING",
                ValueType::Boolean => "BOOLEAN",
                ValueType::Vector3d => "VECTOR3D",
                ValueType::Color => "COLOR",
                ValueType::Pose3d => "POSE3D",
                ValueType::Quaterniond => "QUATERNIOND",
                ValueType::Time => "TIME",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "NONE" => Some(Self::None),
                "DOUBLE" => Some(Self::Double),
                "INT32" => Some(Self::Int32),
                "STRING" => Some(Self::String),
                "BOOLEAN" => Some(Self::Boolean),
                "VECTOR3D" => Some(Self::Vector3d),
                "COLOR" => Some(Self::Color),
                "POSE3D" => Some(Self::Pose3d),
                "QUATERNIOND" => Some(Self::Quaterniond),
                "TIME" => Some(Self::Time),
                _ => None,
            }
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        /// / \brief A double value
        #[prost(double, tag = "3")]
        DoubleValue(f64),
        /// / \brief An int32 value
        #[prost(int32, tag = "4")]
        IntValue(i32),
        /// / \brief A string value
        #[prost(string, tag = "5")]
        StringValue(::prost::alloc::string::String),
        /// / \brief A boolean value
        #[prost(bool, tag = "6")]
        BoolValue(bool),
        /// / \brief A Vector3d value
        #[prost(message, tag = "7")]
        Vector3dValue(super::Vector3d),
        /// / \brief A Color value
        #[prost(message, tag = "8")]
        ColorValue(super::Color),
        /// / \brief A Pose value
        #[prost(message, tag = "9")]
        Pose3dValue(super::Pose),
        /// / \brief A Quaternion value
        #[prost(message, tag = "10")]
        QuaternionValue(super::Quaternion),
        /// / \brief A Time value
        #[prost(message, tag = "11")]
        TimeValue(super::Time),
    }
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Param {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief A set of name, value pairs.
    #[prost(map = "string, message", tag = "2")]
    pub params: ::std::collections::HashMap<::prost::alloc::string::String, Any>,
    /// / \brief Params nested within this one.
    #[prost(message, repeated, tag = "3")]
    pub children: ::prost::alloc::vec::Vec<Param>,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParamV {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief Repeated params.
    #[prost(message, repeated, tag = "2")]
    pub param: ::prost::alloc::vec::Vec<Param>,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CameraLens {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief Type of projection of the lens
    /// /        possible values are "gnomonical", "stereographic", "equidistant",
    /// /        "equisolid_angle", "stereographic", "custom".
    /// /        If you set this value to "custom" you need to specify at least one
    /// /        of the `c1`, `c2`, `c3`, `f` or `fun`.
    #[prost(string, tag = "2")]
    pub r#type: ::prost::alloc::string::String,
    /// / \brief Linear image scaling factor
    #[prost(double, tag = "3")]
    pub c1: f64,
    /// / \brief Angle scaling factor
    #[prost(double, tag = "4")]
    pub c2: f64,
    /// / \brief Angle offset factor
    #[prost(double, tag = "5")]
    pub c3: f64,
    /// / \brief Linear scaling factor, unlike `c1`, will be adjusted to match hfov
    /// /        if scale_to_fov is set to `true`.
    #[prost(double, tag = "6")]
    pub f: f64,
    /// / \brief Angle modification function
    ///          possible values are "tan", "sin" and "id".
    #[prost(string, tag = "7")]
    pub fun: ::prost::alloc::string::String,
    /// / \brief Scale image to fit horizontal FOV
    #[prost(bool, tag = "8")]
    pub scale_to_hfov: bool,
    /// / \brief Everything outside of this angle will be hidden,
    /// /        the angle is counted from camera's X (forward) axis.
    #[prost(double, tag = "9")]
    pub cutoff_angle: f64,
    /// / \brief Horizontal field of view in radians.
    #[prost(double, tag = "10")]
    pub hfov: f64,
    /// / \brief Size of cube map texture,
    /// /        used to store intermediate rendering result.
    #[prost(int32, tag = "11")]
    pub env_texture_size: i32,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Twist {
    /// / \brief Optional header data.
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief Linear velocity in 3d space.
    #[prost(message, optional, tag = "2")]
    pub linear: ::core::option::Option<Vector3d>,
    /// / \brief Angular velocity in 3d space.
    #[prost(message, optional, tag = "3")]
    pub angular: ::core::option::Option<Vector3d>,
}
/// / \brief Used for specifying how to load environmental data
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataLoadPathOptions {
    /// / \brief File path to load
    #[prost(string, tag = "1")]
    pub path: ::prost::alloc::string::String,
    /// / \brief Name of time axis
    #[prost(string, tag = "2")]
    pub time: ::prost::alloc::string::String,
    /// / \brief Is the data static in time
    #[prost(bool, tag = "3")]
    pub static_time: bool,
    /// / \brief Name of x axis
    #[prost(string, tag = "4")]
    pub x: ::prost::alloc::string::String,
    /// / \brief Name of y axis
    #[prost(string, tag = "5")]
    pub y: ::prost::alloc::string::String,
    /// / \brief Name of z axis
    #[prost(string, tag = "6")]
    pub z: ::prost::alloc::string::String,
    /// / Units
    #[prost(enumeration = "data_load_path_options::DataAngularUnits", tag = "7")]
    pub units: i32,
    /// / Spherical Coodinate type
    #[prost(enumeration = "SphericalCoordinatesType", tag = "8")]
    pub coordinate_type: i32,
}
/// Nested message and enum types in `DataLoadPathOptions`.
pub mod data_load_path_options {
    /// / \brief Units used by spherical coordinates
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum DataAngularUnits {
        Radians = 0,
        Degrees = 1,
    }
    impl DataAngularUnits {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                DataAngularUnits::Radians => "RADIANS",
                DataAngularUnits::Degrees => "DEGREES",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "RADIANS" => Some(Self::Radians),
                "DEGREES" => Some(Self::Degrees),
                _ => None,
            }
        }
    }
}
/// / \brief This message contains information about the performance of
/// / a sensor in the world.
/// / If the sensor is a camera then it will publish the frame per second (fps).
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PerformanceSensorMetrics {
    /// / \brief Sensor name
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// / \brief The update rate of the sensor in real time.
    #[prost(double, tag = "2")]
    pub real_update_rate: f64,
    /// / \brief The update rate of the sensor in simulation time.
    #[prost(double, tag = "3")]
    pub sim_update_rate: f64,
    /// / \brief The nominal update rate defined to the sensor.
    #[prost(double, tag = "4")]
    pub nominal_update_rate: f64,
    /// / \brief If the sensor is a camera then this field should be filled
    /// / with average fps in real time.
    #[prost(message, optional, tag = "5")]
    pub fps_optional: ::core::option::Option<Double>,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Fluid {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// Name of the fluid
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// Position of each particle in the fluid.
    #[prost(message, repeated, tag = "3")]
    pub position: ::prost::alloc::vec::Vec<Vector3d>,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EntityWrench {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief Entity to apply the wrench to.
    #[prost(message, optional, tag = "2")]
    pub entity: ::core::option::Option<Entity>,
    /// / \brief Wrench to apply.
    #[prost(message, optional, tag = "3")]
    pub wrench: ::core::option::Option<Wrench>,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Int32V {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief Vector of int data
    #[prost(int32, repeated, tag = "2")]
    pub data: ::prost::alloc::vec::Vec<i32>,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParameterError {
    #[prost(enumeration = "parameter_error::Type", tag = "1")]
    pub data: i32,
}
/// Nested message and enum types in `ParameterError`.
pub mod parameter_error {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Type {
        Success = 0,
        AlreadyDeclared = 1,
        InvalidType = 2,
        NotDeclared = 3,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Success => "SUCCESS",
                Type::AlreadyDeclared => "ALREADY_DECLARED",
                Type::InvalidType => "INVALID_TYPE",
                Type::NotDeclared => "NOT_DECLARED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SUCCESS" => Some(Self::Success),
                "ALREADY_DECLARED" => Some(Self::AlreadyDeclared),
                "INVALID_TYPE" => Some(Self::InvalidType),
                "NOT_DECLARED" => Some(Self::NotDeclared),
                _ => None,
            }
        }
    }
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoseAnimation {
    #[prost(string, tag = "1")]
    pub model_name: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub model_id: u32,
    #[prost(message, repeated, tag = "3")]
    pub pose: ::prost::alloc::vec::Vec<Pose>,
    #[prost(message, repeated, tag = "4")]
    pub time: ::prost::alloc::vec::Vec<Time>,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DvlKinematicEstimate {
    /// / \brief Estimate frame of reference (incl. conventions).
    #[prost(enumeration = "dvl_kinematic_estimate::ReferenceType", tag = "1")]
    pub reference: i32,
    /// / \brief Estimate mean.
    #[prost(message, optional, tag = "2")]
    pub mean: ::core::option::Option<Vector3d>,
    /// / \brief Estimate covariance matrix.
    /// / A 3 x 3 row-major matrix using a flat contiguous layout.
    #[prost(double, repeated, tag = "3")]
    pub covariance: ::prost::alloc::vec::Vec<f64>,
}
/// Nested message and enum types in `DVLKinematicEstimate`.
pub mod dvl_kinematic_estimate {
    /// / \brief Frames of reference (incl. conventions)
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum ReferenceType {
        /// / \brief Unspecific frame of reference.
        DvlReferenceUnspecified = 0,
        /// / \brief Earth bound frame of reference (typically ENU).
        DvlReferenceEarth = 1,
        /// / \brief Ship bound frame of reference (typically FSK).
        DvlReferenceShip = 2,
    }
    impl ReferenceType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ReferenceType::DvlReferenceUnspecified => "DVL_REFERENCE_UNSPECIFIED",
                ReferenceType::DvlReferenceEarth => "DVL_REFERENCE_EARTH",
                ReferenceType::DvlReferenceShip => "DVL_REFERENCE_SHIP",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DVL_REFERENCE_UNSPECIFIED" => Some(Self::DvlReferenceUnspecified),
                "DVL_REFERENCE_EARTH" => Some(Self::DvlReferenceEarth),
                "DVL_REFERENCE_SHIP" => Some(Self::DvlReferenceShip),
                _ => None,
            }
        }
    }
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DvlRangeEstimate {
    /// / \brief Estimate mean.
    #[prost(double, tag = "1")]
    pub mean: f64,
    /// / \brief Estimate variance.
    #[prost(double, tag = "2")]
    pub variance: f64,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DvlBeamState {
    /// / \brief Beam ID.
    #[prost(int32, tag = "1")]
    pub id: i32,
    /// / \brief Beam velocity estimate, measured along
    /// / its axis, in meters per second.
    #[prost(message, optional, tag = "2")]
    pub velocity: ::core::option::Option<DvlKinematicEstimate>,
    /// / \brief Beam range estimate, in meters.
    #[prost(message, optional, tag = "3")]
    pub range: ::core::option::Option<DvlRangeEstimate>,
    /// / \brief Beam signal strength indicator.
    #[prost(double, tag = "4")]
    pub rssi: f64,
    /// / \brief Measured background noise spectral density,
    /// / in watts per hertz.
    #[prost(double, tag = "5")]
    pub nsd: f64,
    /// / \brief Whether beam is locked to its target or not.
    /// / A beam is said to be locked when it can reliably
    /// / measure signal reflections.
    #[prost(bool, tag = "6")]
    pub locked: bool,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PropagationParticle {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    #[prost(double, tag = "2")]
    pub x: f64,
    #[prost(double, tag = "3")]
    pub y: f64,
    #[prost(double, tag = "4")]
    pub signal_level: f64,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UndoRedo {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief True to undo, false to redo.
    #[prost(bool, tag = "2")]
    pub undo: bool,
    /// / \brief Unique id of the user command. If this is provided, all commands
    /// / leading to that will be undone / redone.
    #[prost(uint32, tag = "3")]
    pub id: u32,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StringMsgV {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    #[prost(string, repeated, tag = "2")]
    pub data: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// / \brief (Copied from the ROS message): This message holds a collection of
/// / N-dimensional points, which may contain additional information such as
/// / normals, intensity, etc. The point data is stored as a binary blob, its
/// / layout described by the contents of the "fields" array.
/// /
/// / The point cloud data may be organized 2d (image-like) or 1d
/// / (unordered). Point clouds organized as 2d images may be produced by
/// / camera depth sensors such as stereo or time-of-flight.
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PointCloudPacked {
    /// / \brief Optional header data. This should contain time of sensor data
    /// / acquisition, and the coordinate frame ID (for 3D points).
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief Information that describes the data contained in the `data` field.
    #[prost(message, repeated, tag = "2")]
    pub field: ::prost::alloc::vec::Vec<point_cloud_packed::Field>,
    /// / \brief Height of a 2D structured point cloud, or 1 if the point cloud is
    /// / unordered.
    #[prost(uint32, tag = "3")]
    pub height: u32,
    /// / \brief Width of a 2D structured point cloud, or length of the point cloud
    /// / if the point cloud is unordered.
    #[prost(uint32, tag = "4")]
    pub width: u32,
    /// / \brief Is this data big endian?
    #[prost(bool, tag = "5")]
    pub is_bigendian: bool,
    /// / \brief Length of a point in bytes.
    #[prost(uint32, tag = "6")]
    pub point_step: u32,
    /// / \brief Length of row in bytes.
    #[prost(uint32, tag = "7")]
    pub row_step: u32,
    /// / \brief The point data, size is (row_step * height);
    #[prost(bytes = "vec", tag = "8")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    /// / \brief True if there are not invalid points.
    #[prost(bool, tag = "9")]
    pub is_dense: bool,
}
/// Nested message and enum types in `PointCloudPacked`.
pub mod point_cloud_packed {
    /// / \brief A field that describes the format of the data field.
    #[derive(::rgz_derive::GzMessage)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Field {
        /// / \brief Name of the field.
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        /// / \brief Offset from start of point struct
        #[prost(uint32, tag = "2")]
        pub offset: u32,
        /// / \brief Datatype enumeration
        #[prost(enumeration = "field::DataType", tag = "3")]
        pub datatype: i32,
        /// / \brief How many elements in the field
        #[prost(uint32, tag = "4")]
        pub count: u32,
    }
    /// Nested message and enum types in `Field`.
    pub mod field {
        /// Datatype for the point field.
        #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            ::prost::Enumeration
        )]
        #[repr(i32)]
        pub enum DataType {
            Int8 = 0,
            Uint8 = 1,
            Int16 = 2,
            Uint16 = 3,
            Int32 = 4,
            Uint32 = 5,
            Float32 = 6,
            Float64 = 7,
        }
        impl DataType {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    DataType::Int8 => "INT8",
                    DataType::Uint8 => "UINT8",
                    DataType::Int16 => "INT16",
                    DataType::Uint16 => "UINT16",
                    DataType::Int32 => "INT32",
                    DataType::Uint32 => "UINT32",
                    DataType::Float32 => "FLOAT32",
                    DataType::Float64 => "FLOAT64",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "INT8" => Some(Self::Int8),
                    "UINT8" => Some(Self::Uint8),
                    "INT16" => Some(Self::Int16),
                    "UINT16" => Some(Self::Uint16),
                    "INT32" => Some(Self::Int32),
                    "UINT32" => Some(Self::Uint32),
                    "FLOAT32" => Some(Self::Float32),
                    "FLOAT64" => Some(Self::Float64),
                    _ => None,
                }
            }
        }
    }
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DoubleV {
    /// / \brief Vector of double data
    #[prost(double, repeated, tag = "1")]
    pub data: ::prost::alloc::vec::Vec<f64>,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Clock {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    #[prost(message, optional, tag = "2")]
    pub system: ::core::option::Option<Time>,
    #[prost(message, optional, tag = "3")]
    pub real: ::core::option::Option<Time>,
    #[prost(message, optional, tag = "4")]
    pub sim: ::core::option::Option<Time>,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CmdVel2D {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    #[prost(double, tag = "2")]
    pub velocity: f64,
    #[prost(double, tag = "3")]
    pub theta: f64,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoseV {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    #[prost(message, repeated, tag = "2")]
    pub pose: ::prost::alloc::vec::Vec<Pose>,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Packet {
    #[prost(string, tag = "1")]
    pub topic: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(oneof = "packet::Content", tags = "3, 4, 5, 6, 7, 8, 9, 10, 11, 12")]
    pub content: ::core::option::Option<packet::Content>,
}
/// Nested message and enum types in `Packet`.
pub mod packet {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Content {
        #[prost(message, tag = "3")]
        CmdVel2d(super::CmdVel2D),
        #[prost(message, tag = "4")]
        Image(super::Image),
        #[prost(message, tag = "5")]
        StringMsgV(super::StringMsgV),
        #[prost(message, tag = "6")]
        WebRequest(super::WebRequest),
        #[prost(message, tag = "7")]
        Pose(super::Pose),
        #[prost(message, tag = "8")]
        Doublev(super::DoubleV),
        #[prost(message, tag = "9")]
        PoseV(super::PoseV),
        #[prost(message, tag = "10")]
        Time(super::Time),
        #[prost(message, tag = "11")]
        Clock(super::Clock),
        #[prost(message, tag = "12")]
        WorldStats(super::WorldStatistics),
    }
}
/// / \brief Fluid pressure data.
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FluidPressure {
    /// Other Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief Pressure reading in Pascals
    #[prost(double, tag = "2")]
    pub pressure: f64,
    /// / \brief Pressure variance. 0 is interpreted as variance unknown.
    #[prost(double, tag = "3")]
    pub variance: f64,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UInt64V {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief Vector of int data
    #[prost(uint64, repeated, tag = "2")]
    pub data: ::prost::alloc::vec::Vec<u64>,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Marker {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief The action to take
    /// /
    /// / Relevant Type: all
    #[prost(enumeration = "marker::Action", tag = "2")]
    pub action: i32,
    /// / \brief Namespace of the marker. A namespace groups id's together.
    /// /
    /// / Relevant Action: ADD_MODIFY, DELETE_MARKER, DELETE_ALL
    #[prost(string, tag = "3")]
    pub ns: ::prost::alloc::string::String,
    /// / \brief The id within the namespace of the visual. Each marker has a
    /// / unique id. It's up to the user to select id values.
    /// /
    /// / Relevant Action: ADD_MODIFY, DELETE_MARKER
    /// /
    /// / Relevant Type: all
    #[prost(uint64, tag = "4")]
    pub id: u64,
    /// / \brief The layer the visual belongs to.
    /// /
    /// / Relevant Action: ADD_MODIFY
    /// /
    /// / Relevant Type: all
    #[prost(int32, tag = "5")]
    pub layer: i32,
    /// / \brief The type of geometry.
    /// /
    /// / Relevant Action: ADD_MODIFY
    #[prost(enumeration = "marker::Type", tag = "6")]
    pub r#type: i32,
    /// / \brief How long to keep the visual alive before deletion. A value of
    /// / zero indicates forever. The lifetime is based on simulation-time, not
    /// / real-time.
    /// /
    /// / Relevant Action: ADD_MODIFY
    /// /
    /// / Relevant Type: all
    #[prost(message, optional, tag = "7")]
    pub lifetime: ::core::option::Option<Time>,
    /// / \brief Pose of the marker
    /// /
    /// / Relevant Action: ADD_MODIFY
    /// /
    /// / Relevant Type: all
    #[prost(message, optional, tag = "8")]
    pub pose: ::core::option::Option<Pose>,
    /// / \brief Scale of the marker.
    /// /
    /// / Relevant Action: ADD_MODIFY
    /// /
    /// / Relevant Type: all
    #[prost(message, optional, tag = "9")]
    pub scale: ::core::option::Option<Vector3d>,
    /// / \brief Marker color
    /// /
    /// / Relevant Action: ADD_MODIFY
    /// /
    /// / Relevant Type: all
    #[prost(message, optional, tag = "10")]
    pub material: ::core::option::Option<Material>,
    /// / \brief Used to specify geometry for a LINE_STRIP, LINE_LIST, POINTS,
    /// / TRIANGLE_LIST, TRIANGLE_FAN, TRIANGLE_STRIP
    /// /
    /// / Relevant Action: ADD_MODIFY
    /// /
    /// / Relevant Type: LINE_STRIP, LINE_LIST, POINTS, TRIANGLE_FAN, TRIANGLE_LIST,
    /// / TRIANGLE_STRIP
    #[prost(message, repeated, tag = "11")]
    pub point: ::prost::alloc::vec::Vec<Vector3d>,
    /// / \brief String to display. Only used for TEXT marker.
    /// /
    /// / Relevant Action: ADD_MODIFY
    /// /
    /// / Relevant Type: TEXT
    #[prost(string, tag = "12")]
    pub text: ::prost::alloc::string::String,
    /// / \brief Attach this marker to a "parent" visual.
    /// /
    /// / Relevant Action: ADD_MODIFY
    /// /
    /// / Relevant Type: all
    #[prost(string, tag = "13")]
    pub parent: ::prost::alloc::string::String,
    /// / \brief Defines what cameras render the marker.
    /// /
    /// / Relevant Action: ADD_MODIFY
    /// /
    /// / Relevant Type: all
    #[prost(enumeration = "marker::Visibility", tag = "14")]
    pub visibility: i32,
    /// / \brief Marker color for repeated types.
    /// /
    /// / Relevant Action: ADD_MODIFY
    /// /
    /// / Relevant Type: LINE_STRIP, LINE_LIST, POINTS, TRIANGLE_FAN, TRIANGLE_LIST,
    /// / TRIANGLE_STRIP
    #[prost(message, repeated, tag = "15")]
    pub materials: ::prost::alloc::vec::Vec<Material>,
}
/// Nested message and enum types in `Marker`.
pub mod marker {
    /// / \brief The marker type (shape/geometry)
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Type {
        None = 0,
        Box = 1,
        Cylinder = 2,
        LineList = 4,
        LineStrip = 3,
        Points = 5,
        Sphere = 6,
        Text = 7,
        TriangleFan = 8,
        TriangleList = 9,
        TriangleStrip = 10,
        Cone = 11,
        Arrow = 12,
        Axis = 13,
        Capsule = 14,
        Ellipsoid = 15,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::None => "NONE",
                Type::Box => "BOX",
                Type::Cylinder => "CYLINDER",
                Type::LineList => "LINE_LIST",
                Type::LineStrip => "LINE_STRIP",
                Type::Points => "POINTS",
                Type::Sphere => "SPHERE",
                Type::Text => "TEXT",
                Type::TriangleFan => "TRIANGLE_FAN",
                Type::TriangleList => "TRIANGLE_LIST",
                Type::TriangleStrip => "TRIANGLE_STRIP",
                Type::Cone => "CONE",
                Type::Arrow => "ARROW",
                Type::Axis => "AXIS",
                Type::Capsule => "CAPSULE",
                Type::Ellipsoid => "ELLIPSOID",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "NONE" => Some(Self::None),
                "BOX" => Some(Self::Box),
                "CYLINDER" => Some(Self::Cylinder),
                "LINE_LIST" => Some(Self::LineList),
                "LINE_STRIP" => Some(Self::LineStrip),
                "POINTS" => Some(Self::Points),
                "SPHERE" => Some(Self::Sphere),
                "TEXT" => Some(Self::Text),
                "TRIANGLE_FAN" => Some(Self::TriangleFan),
                "TRIANGLE_LIST" => Some(Self::TriangleList),
                "TRIANGLE_STRIP" => Some(Self::TriangleStrip),
                "CONE" => Some(Self::Cone),
                "ARROW" => Some(Self::Arrow),
                "AXIS" => Some(Self::Axis),
                "CAPSULE" => Some(Self::Capsule),
                "ELLIPSOID" => Some(Self::Ellipsoid),
                _ => None,
            }
        }
    }
    /// / \brief Visilibity defines what cameras render the marker.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Visibility {
        /// / \brief Only cameras for user interaction render the visual. Other
        /// / cameras, such as camera sensors, will not render the visual
        Gui = 0,
        /// / \brief All cameras render the visual.
        All = 1,
    }
    impl Visibility {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Visibility::Gui => "GUI",
                Visibility::All => "ALL",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "GUI" => Some(Self::Gui),
                "ALL" => Some(Self::All),
                _ => None,
            }
        }
    }
    /// / \brief How to interpret the data.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Action {
        /// / \brief Use this action to create a new marker or modify an existing
        /// / marker. A marker will be created if the provided id does not match
        /// / an existing marker, otherwise the marker with the provided id will
        /// / be modified.
        AddModify = 0,
        /// / \brief Use this action to delete an existing marking.
        /// / Nothing will happened if the provided id does not match an existing
        /// / marker.
        DeleteMarker = 1,
        /// / \brief Delete all the markers. If a namespace is provided,
        /// / then only the markers in the provided namespace are deleted.
        DeleteAll = 2,
    }
    impl Action {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Action::AddModify => "ADD_MODIFY",
                Action::DeleteMarker => "DELETE_MARKER",
                Action::DeleteAll => "DELETE_ALL",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ADD_MODIFY" => Some(Self::AddModify),
                "DELETE_MARKER" => Some(Self::DeleteMarker),
                "DELETE_ALL" => Some(Self::DeleteAll),
                _ => None,
            }
        }
    }
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarkerV {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief List of marker messages.
    #[prost(message, repeated, tag = "2")]
    pub marker: ::prost::alloc::vec::Vec<Marker>,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PropagationGrid {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    #[prost(message, repeated, tag = "2")]
    pub particle: ::prost::alloc::vec::Vec<PropagationParticle>,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Publish {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    #[prost(string, tag = "2")]
    pub topic: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub msg_type: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub host: ::prost::alloc::string::String,
    #[prost(uint32, tag = "5")]
    pub port: u32,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Publishers {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    #[prost(message, repeated, tag = "2")]
    pub publisher: ::prost::alloc::vec::Vec<Publish>,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorldControlState {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief A WorldControl that allows for control of world functions
    #[prost(message, optional, tag = "2")]
    pub world_control: ::core::option::Option<WorldControl>,
    /// / \brief A SerializedState that can contain information about the
    /// / state of an entity-comonent-system (ECS)
    #[prost(message, optional, tag = "3")]
    pub state: ::core::option::Option<SerializedState>,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CameraCmd {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    #[prost(string, tag = "2")]
    pub follow_model: ::prost::alloc::string::String,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogControl {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    #[prost(bool, tag = "2")]
    pub start: bool,
    #[prost(bool, tag = "3")]
    pub stop: bool,
    #[prost(bool, tag = "4")]
    pub paused: bool,
    #[prost(string, tag = "5")]
    pub base_path: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub encoding: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub record_resources: ::prost::alloc::string::String,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Bytes {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief Bytes data
    #[prost(bytes = "vec", tag = "2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VersionRange {
    /// / \brief Min version.
    #[prost(message, optional, tag = "1")]
    pub min: ::core::option::Option<Version>,
    /// / \brief Max version.
    #[prost(message, optional, tag = "2")]
    pub max: ::core::option::Option<Version>,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cessna {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief Current RPM of the propeller.
    #[prost(float, tag = "2")]
    pub propeller_speed: f32,
    /// / \brief Current left aileron angle in rads.
    #[prost(float, tag = "3")]
    pub left_aileron: f32,
    /// / \brief Current left flap angle in rads.
    #[prost(float, tag = "4")]
    pub left_flap: f32,
    /// / \brief Current right aileron angle in rads.
    #[prost(float, tag = "5")]
    pub right_aileron: f32,
    /// / \brief Current right flap angle in rads.
    #[prost(float, tag = "6")]
    pub right_flap: f32,
    /// / \brief Current elevators angle in rads.
    #[prost(float, tag = "7")]
    pub elevators: f32,
    /// / \brief Current ruddle angle in rads.
    #[prost(float, tag = "8")]
    pub rudder: f32,
    /// / \brief Target RPM of the propeller.
    #[prost(float, tag = "9")]
    pub cmd_propeller_speed: f32,
    /// / \brief Target left aileron angle in rads.
    #[prost(float, tag = "10")]
    pub cmd_left_aileron: f32,
    /// / \brief Target left flap angle in rads.
    #[prost(float, tag = "11")]
    pub cmd_left_flap: f32,
    /// / \brief Target right aileron angle in rads.
    #[prost(float, tag = "12")]
    pub cmd_right_aileron: f32,
    /// / \brief Target right flap angle in rads.
    #[prost(float, tag = "13")]
    pub cmd_right_flap: f32,
    /// / \brief Target elevators angle in rads.
    #[prost(float, tag = "14")]
    pub cmd_elevators: f32,
    /// / \brief Target ruddle angle in rads.
    #[prost(float, tag = "15")]
    pub cmd_rudder: f32,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FuelMetadata {
    /// / \brief Name of the resource.
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    /// / \brief Description of the resource.
    #[prost(string, tag = "4")]
    pub description: ::prost::alloc::string::String,
    /// / \brief Version number of the resource. This version is set by Fuel.
    #[prost(int32, tag = "5")]
    pub version: i32,
    /// / \brief Authors of this resource.
    #[prost(message, repeated, tag = "6")]
    pub authors: ::prost::alloc::vec::Vec<fuel_metadata::Contact>,
    /// / \brief Legal information, such as copyright and license specifications.
    #[prost(message, optional, tag = "7")]
    pub legal: ::core::option::Option<fuel_metadata::Legal>,
    /// / \brief Tags for a resource.
    #[prost(string, repeated, tag = "8")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// A list of key-value pairs that can contain arbitrary user data.
    #[prost(map = "string, string", tag = "9")]
    pub annotations: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// / \brief Resources that this resource depends on.
    #[prost(message, repeated, tag = "10")]
    pub dependencies: ::prost::alloc::vec::Vec<fuel_metadata::Dependency>,
    /// / \brief List of tools/libraries with version numbers that are compatible
    /// / with this resource.
    #[prost(message, repeated, tag = "11")]
    pub compatibilities: ::prost::alloc::vec::Vec<fuel_metadata::Compatibility>,
    /// / \brief Categories associated with this resource.
    #[prost(message, optional, tag = "12")]
    pub categories: ::core::option::Option<fuel_metadata::Categories>,
    /// / \brief A Fuel resource has to be one of the following.
    #[prost(oneof = "fuel_metadata::ResourceType", tags = "1, 2")]
    pub resource_type: ::core::option::Option<fuel_metadata::ResourceType>,
}
/// Nested message and enum types in `FuelMetadata`.
pub mod fuel_metadata {
    /// / \brief Contact information.
    #[derive(::rgz_derive::GzMessage)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Contact {
        /// / \brief Contact name.
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        /// / \brief Contact email.
        #[prost(string, tag = "2")]
        pub email: ::prost::alloc::string::String,
    }
    /// / \brief Legal information, including copyright and license specifications.
    #[derive(::rgz_derive::GzMessage)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Legal {
        /// / \brief Copyright information, such as "Copyright 1974, John Doe"
        #[prost(string, tag = "1")]
        pub copyright: ::prost::alloc::string::String,
        /// / \brief License, such as "Apache-2.0"
        #[prost(string, tag = "2")]
        pub license: ::prost::alloc::string::String,
    }
    /// / \brief Information about a model resource.
    #[derive(::rgz_derive::GzMessage)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Model {
        /// / \brief Main model file, e.g. "model.sdf".
        #[prost(string, tag = "1")]
        pub file: ::prost::alloc::string::String,
        /// / \brief Name and version of the file format used by the file.
        #[prost(message, optional, tag = "2")]
        pub file_format: ::core::option::Option<super::VersionedName>,
    }
    /// / \brief Information about a world resource.
    #[derive(::rgz_derive::GzMessage)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct World {
        /// / \brief Main world file, e.g. "world.sdf".
        #[prost(string, tag = "1")]
        pub file: ::prost::alloc::string::String,
        /// / \brief Name and version of the file format used by the file.
        #[prost(message, optional, tag = "2")]
        pub file_format: ::core::option::Option<super::VersionedName>,
    }
    /// / \brief Definition of a dependency
    #[derive(::rgz_derive::GzMessage)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Dependency {
        /// / \brief Dependency uri.
        #[prost(string, tag = "1")]
        pub uri: ::prost::alloc::string::String,
    }
    /// / \brief A message containing a tool name and a version or range of
    /// / versions, e.g.
    /// /   tools { name: "bullet" version_range { min: {major: 3 } } } }
    /// /   tools { name: "gazebo" version { major: 11 } }
    #[derive(::rgz_derive::GzMessage)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Compatibility {
        /// / \brief Name of the tool/library.
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        /// / \brief If version is omitted, it is assumed that the model is
        /// / compatible with all versions of the tool.
        #[prost(oneof = "compatibility::VersionType", tags = "2, 3")]
        pub version_type: ::core::option::Option<compatibility::VersionType>,
    }
    /// Nested message and enum types in `Compatibility`.
    pub mod compatibility {
        /// / \brief If version is omitted, it is assumed that the model is
        /// / compatible with all versions of the tool.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum VersionType {
            /// / \brief Exact version that the model is compatible with.
            #[prost(message, tag = "2")]
            Version(super::super::Version),
            /// / \brief A range of compatible versions.
            #[prost(message, tag = "3")]
            VersionRange(super::super::VersionRange),
        }
    }
    /// / \brief Categories associated with this resource. The set of
    /// / Fuel categories are available at
    /// / <https://fuel.ignitionrobotics.org/1.0/categories.>
    /// /
    /// / A limited number of categories can be assigned to a Fuel resource.
    #[derive(::rgz_derive::GzMessage)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Categories {
        /// / \brief First category.
        #[prost(string, tag = "1")]
        pub first: ::prost::alloc::string::String,
        /// / \brief Second category.
        #[prost(string, tag = "2")]
        pub second: ::prost::alloc::string::String,
    }
    /// / \brief A Fuel resource has to be one of the following.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ResourceType {
        #[prost(message, tag = "1")]
        Model(Model),
        #[prost(message, tag = "2")]
        World(World),
    }
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Actor {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief A unique name for the actor
    #[prost(message, optional, tag = "2")]
    pub entity: ::core::option::Option<Entity>,
    /// / \brief Pose of the actor
    #[prost(message, optional, tag = "3")]
    pub pose: ::core::option::Option<Pose>,
    /// / \brief Skin file which defines a visual
    /// / and the underlying skeleton which moves it
    #[prost(string, tag = "4")]
    pub skin_filename: ::prost::alloc::string::String,
    /// / \brief Scale the skin's size
    #[prost(float, tag = "5")]
    pub skin_scale: f32,
    /// / \brief Animations for the skeleton in the skin
    #[prost(message, repeated, tag = "6")]
    pub animations: ::prost::alloc::vec::Vec<actor::Animation>,
    /// / \brief Set this to true for the script to be repeated in a loop
    #[prost(bool, tag = "7")]
    pub script_loop: bool,
    /// / \brief Time (in secs) to wait before starting the script
    #[prost(float, tag = "8")]
    pub script_delay_start: f32,
    /// / \brief Set to true if the animation should start
    /// / as soon as the simulation starts playing
    #[prost(bool, tag = "9")]
    pub script_auto_start: bool,
    /// / \brief A series of keyframes to be followed
    #[prost(message, repeated, tag = "10")]
    pub trajectories: ::prost::alloc::vec::Vec<actor::Trajectory>,
    /// / \brief Unique id of actor's parent
    #[prost(message, optional, tag = "11")]
    pub parent: ::core::option::Option<Entity>,
}
/// Nested message and enum types in `Actor`.
pub mod actor {
    #[derive(::rgz_derive::GzMessage)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Animation {
        /// / \brief Unique name for animation
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        /// / \brief Path to animation file. Accepted formats: COLLADA, BVH
        #[prost(string, tag = "2")]
        pub filename: ::prost::alloc::string::String,
        /// / \brief Scale for the animation skeleton
        #[prost(float, tag = "3")]
        pub scale: f32,
        /// / \brief Set to true so the animation is interpolated on X
        #[prost(bool, tag = "4")]
        pub interpolate_x: bool,
    }
    #[derive(::rgz_derive::GzMessage)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Waypoint {
        /// / \brief Time in seconds, counted from the beginning of the script
        #[prost(float, tag = "1")]
        pub time: f32,
        /// / \brief Pose to be reached at the given time
        #[prost(message, optional, tag = "2")]
        pub pose: ::core::option::Option<super::Pose>,
    }
    #[derive(::rgz_derive::GzMessage)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Trajectory {
        /// / \brief Unique id for a trajectory
        #[prost(uint32, tag = "1")]
        pub id: u32,
        /// / \brief Type of an animation
        #[prost(string, tag = "2")]
        pub r#type: ::prost::alloc::string::String,
        /// / \brief Tension of the trajectory spline
        #[prost(float, tag = "3")]
        pub tension: f32,
        /// / \brief Points in the trajectory
        #[prost(message, repeated, tag = "4")]
        pub waypoints: ::prost::alloc::vec::Vec<Waypoint>,
    }
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LaserScan {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    #[prost(string, tag = "2")]
    pub frame: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub world_pose: ::core::option::Option<Pose>,
    #[prost(double, tag = "4")]
    pub angle_min: f64,
    #[prost(double, tag = "5")]
    pub angle_max: f64,
    #[prost(double, tag = "6")]
    pub angle_step: f64,
    #[prost(double, tag = "7")]
    pub range_min: f64,
    #[prost(double, tag = "8")]
    pub range_max: f64,
    #[prost(uint32, tag = "9")]
    pub count: u32,
    #[prost(double, tag = "10")]
    pub vertical_angle_min: f64,
    #[prost(double, tag = "11")]
    pub vertical_angle_max: f64,
    #[prost(double, tag = "12")]
    pub vertical_angle_step: f64,
    #[prost(uint32, tag = "13")]
    pub vertical_count: u32,
    #[prost(double, repeated, tag = "14")]
    pub ranges: ::prost::alloc::vec::Vec<f64>,
    #[prost(double, repeated, tag = "15")]
    pub intensities: ::prost::alloc::vec::Vec<f64>,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Parameter {
    /// / \brief Parameter name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// / \brief Serialized parameter value.
    #[prost(message, optional, tag = "2")]
    pub value: ::core::option::Option<::prost_types::Any>,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DvlTrackingTarget {
    /// / \brief Type of target used for tracking.
    #[prost(enumeration = "dvl_tracking_target::TargetType", tag = "1")]
    pub r#type: i32,
    /// / \brief Target range (or distance), in meters
    #[prost(message, optional, tag = "2")]
    pub range: ::core::option::Option<DvlRangeEstimate>,
    /// / \brief Target position estimate, in meters.
    #[prost(message, optional, tag = "3")]
    pub position: ::core::option::Option<DvlKinematicEstimate>,
}
/// Nested message and enum types in `DVLTrackingTarget`.
pub mod dvl_tracking_target {
    /// / \brief Target types
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum TargetType {
        /// / \brief Unspecific target type.
        DvlTargetUnspecified = 0,
        /// / \brief Bottom ground (ie. solid) target.
        DvlTargetBottom = 1,
        /// / \brief Water mass layer (ie. fluid) target.
        DvlTargetWaterMass = 2,
    }
    impl TargetType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                TargetType::DvlTargetUnspecified => "DVL_TARGET_UNSPECIFIED",
                TargetType::DvlTargetBottom => "DVL_TARGET_BOTTOM",
                TargetType::DvlTargetWaterMass => "DVL_TARGET_WATER_MASS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DVL_TARGET_UNSPECIFIED" => Some(Self::DvlTargetUnspecified),
                "DVL_TARGET_BOTTOM" => Some(Self::DvlTargetBottom),
                "DVL_TARGET_WATER_MASS" => Some(Self::DvlTargetWaterMass),
                _ => None,
            }
        }
    }
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Fog {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    #[prost(enumeration = "fog::FogType", tag = "2")]
    pub r#type: i32,
    #[prost(message, optional, tag = "3")]
    pub color: ::core::option::Option<Color>,
    #[prost(float, tag = "4")]
    pub density: f32,
    #[prost(float, tag = "5")]
    pub start: f32,
    #[prost(float, tag = "6")]
    pub end: f32,
}
/// Nested message and enum types in `Fog`.
pub mod fog {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum FogType {
        None = 0,
        Linear = 1,
        Exponential = 2,
        Exponential2 = 3,
    }
    impl FogType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                FogType::None => "NONE",
                FogType::Linear => "LINEAR",
                FogType::Exponential => "EXPONENTIAL",
                FogType::Exponential2 => "EXPONENTIAL2",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "NONE" => Some(Self::None),
                "LINEAR" => Some(Self::Linear),
                "EXPONENTIAL" => Some(Self::Exponential),
                "EXPONENTIAL2" => Some(Self::Exponential2),
                _ => None,
            }
        }
    }
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JointWrench {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    #[prost(string, tag = "2")]
    pub body_1_name: ::prost::alloc::string::String,
    #[prost(uint32, tag = "3")]
    pub body_1_id: u32,
    #[prost(string, tag = "4")]
    pub body_2_name: ::prost::alloc::string::String,
    #[prost(uint32, tag = "5")]
    pub body_2_id: u32,
    #[prost(message, optional, tag = "6")]
    pub body_1_wrench: ::core::option::Option<Wrench>,
    #[prost(message, optional, tag = "7")]
    pub body_2_wrench: ::core::option::Option<Wrench>,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Contact {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    #[prost(message, optional, tag = "2")]
    pub collision1: ::core::option::Option<Entity>,
    #[prost(message, optional, tag = "3")]
    pub collision2: ::core::option::Option<Entity>,
    #[prost(message, repeated, tag = "4")]
    pub position: ::prost::alloc::vec::Vec<Vector3d>,
    #[prost(message, repeated, tag = "5")]
    pub normal: ::prost::alloc::vec::Vec<Vector3d>,
    #[prost(double, repeated, tag = "6")]
    pub depth: ::prost::alloc::vec::Vec<f64>,
    #[prost(message, repeated, tag = "7")]
    pub wrench: ::prost::alloc::vec::Vec<JointWrench>,
    #[prost(message, optional, tag = "8")]
    pub world: ::core::option::Option<Entity>,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Contacts {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    #[prost(message, repeated, tag = "2")]
    pub contact: ::prost::alloc::vec::Vec<Contact>,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModelV {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    #[prost(message, repeated, tag = "2")]
    pub models: ::prost::alloc::vec::Vec<Model>,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RestPost {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief ID of this request message
    #[prost(uint32, tag = "2")]
    pub id: u32,
    /// / \brief Route to post to.
    #[prost(string, tag = "3")]
    pub route: ::prost::alloc::string::String,
    /// / \brief Data to post in JSON format
    #[prost(string, tag = "4")]
    pub json: ::prost::alloc::string::String,
}
/// / \brief Actuator commands.
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Actuators {
    /// Optional header data.
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief Position of the actuators in \[rad\] for angular actuators
    /// / and \[m\] for linear actuators.
    #[prost(double, repeated, tag = "2")]
    pub position: ::prost::alloc::vec::Vec<f64>,
    /// / \brief Velocities of the actuators in \[rad/s\] for angular actuators
    /// / and \[m/s\] for linear actuators.
    #[prost(double, repeated, tag = "3")]
    pub velocity: ::prost::alloc::vec::Vec<f64>,
    /// / \brief Everything that does not fit the above,
    /// / normalized between \[-1 ... 1\].
    #[prost(double, repeated, tag = "4")]
    pub normalized: ::prost::alloc::vec::Vec<f64>,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Joystick {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// Translation measurements along the x,y,z
    /// axes. These values should be normalized to
    /// the range -1...1
    #[prost(message, optional, tag = "2")]
    pub translation: ::core::option::Option<Vector3d>,
    /// Rotation measurements about the x,y,z
    /// axes. These values should be normalized to
    /// the range -1...1
    #[prost(message, optional, tag = "3")]
    pub rotation: ::core::option::Option<Vector3d>,
    /// Button measurements
    #[prost(int32, repeated, tag = "4")]
    pub buttons: ::prost::alloc::vec::Vec<i32>,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EntityPluginV {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief Entity the plugins belong to
    #[prost(message, optional, tag = "2")]
    pub entity: ::core::option::Option<Entity>,
    /// / \brief Plugin messages.
    #[prost(message, repeated, tag = "3")]
    pub plugins: ::prost::alloc::vec::Vec<Plugin>,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Joy {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// Axis data from a joystick.
    #[prost(float, repeated, tag = "2")]
    pub axes: ::prost::alloc::vec::Vec<f32>,
    /// Button data from a joystick
    #[prost(int32, repeated, tag = "3")]
    pub buttons: ::prost::alloc::vec::Vec<i32>,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tactile {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    #[prost(string, repeated, tag = "2")]
    pub collision_name: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(uint32, repeated, tag = "3")]
    pub collision_id: ::prost::alloc::vec::Vec<u32>,
    #[prost(double, repeated, tag = "4")]
    pub pressure: ::prost::alloc::vec::Vec<f64>,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Response {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    #[prost(int32, tag = "2")]
    pub id: i32,
    #[prost(string, tag = "3")]
    pub request: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub response: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "6")]
    pub serialized_data: ::prost::alloc::vec::Vec<u8>,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Empty {
    /// / \brief Unused field.
    #[prost(bool, tag = "1")]
    pub unused: bool,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UInt64 {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief Integer data
    #[prost(uint64, tag = "2")]
    pub data: u64,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Shadows {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    #[prost(enumeration = "shadows::ShadowType", tag = "2")]
    pub r#type: i32,
    #[prost(message, optional, tag = "3")]
    pub color: ::core::option::Option<Color>,
}
/// Nested message and enum types in `Shadows`.
pub mod shadows {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum ShadowType {
        StencilAdditive = 0,
        StencilModulative = 1,
        TextureAdditive = 2,
        TextureModulative = 3,
    }
    impl ShadowType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ShadowType::StencilAdditive => "STENCIL_ADDITIVE",
                ShadowType::StencilModulative => "STENCIL_MODULATIVE",
                ShadowType::TextureAdditive => "TEXTURE_ADDITIVE",
                ShadowType::TextureModulative => "TEXTURE_MODULATIVE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STENCIL_ADDITIVE" => Some(Self::StencilAdditive),
                "STENCIL_MODULATIVE" => Some(Self::StencilModulative),
                "TEXTURE_ADDITIVE" => Some(Self::TextureAdditive),
                "TEXTURE_MODULATIVE" => Some(Self::TextureModulative),
                _ => None,
            }
        }
    }
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParticleEmitterV {
    /// / \brief Optional header data.
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief List of particle emitters.
    #[prost(message, repeated, tag = "2")]
    pub particle_emitter: ::prost::alloc::vec::Vec<ParticleEmitter>,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParameterValue {
    /// / \brief Serialized protobuf message.
    #[prost(message, optional, tag = "1")]
    pub data: ::core::option::Option<::prost_types::Any>,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Subscribe {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    #[prost(string, tag = "2")]
    pub topic: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub host: ::prost::alloc::string::String,
    #[prost(uint32, tag = "4")]
    pub port: u32,
    #[prost(string, tag = "5")]
    pub msg_type: ::prost::alloc::string::String,
    #[prost(bool, tag = "6")]
    pub latching: bool,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogicalCameraImage {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief Pose of the logical camera.
    #[prost(message, optional, tag = "2")]
    pub pose: ::core::option::Option<Pose>,
    /// / \brief All the models seen by the LogicalCamera
    #[prost(message, repeated, tag = "3")]
    pub model: ::prost::alloc::vec::Vec<logical_camera_image::Model>,
}
/// Nested message and enum types in `LogicalCameraImage`.
pub mod logical_camera_image {
    /// / \brief Information about a model that is reported by a
    /// / LogicalCameraSensor
    #[derive(::rgz_derive::GzMessage)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Model {
        /// / \brief Name of the detected model
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        /// / \brief Pose of the detected model. The pose is relative to the
        /// / logical camera's pose.
        #[prost(message, optional, tag = "2")]
        pub pose: ::core::option::Option<super::Pose>,
    }
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TwistWithCovariance {
    /// / \brief Twist message.
    #[prost(message, optional, tag = "1")]
    pub twist: ::core::option::Option<Twist>,
    /// / \brief Row-major representation of the 6x6 covariance matrix
    /// / The orientation parameters use a fixed-axis representation.
    /// / In order, the parameters are:
    /// / (x, y, z, rotation about X axis, rotation about Y axis, rotation about Z axis)
    #[prost(message, optional, tag = "2")]
    pub covariance: ::core::option::Option<FloatV>,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Statistic {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief The data type.
    #[prost(enumeration = "statistic::DataType", tag = "2")]
    pub r#type: i32,
    /// / \brief Name associated with the statistic.
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    /// / \brief The statistic's value.
    #[prost(double, tag = "4")]
    pub value: f64,
}
/// Nested message and enum types in `Statistic`.
pub mod statistic {
    /// / \brief The type of data represented by this statistic.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum DataType {
        /// / \brief The data type has not been initialized.
        Uninitialized = 0,
        /// / \brief An average value is represented.
        Average = 1,
        /// / \brief A minimum value is represented.
        Minimum = 2,
        /// / \brief A maximum value is represented.
        Maximum = 3,
        /// / \brief A variance is represented.
        Variance = 4,
        /// / \brief A standard deviation is represented.
        Stddev = 5,
        /// / \brief A sample count is represented.
        SampleCount = 6,
        /// / \brief A root mean square value is represented.
        RootMeanSquare = 7,
        /// / \brief A maximum absolute value is represented.
        MaxAbsValue = 8,
    }
    impl DataType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                DataType::Uninitialized => "UNINITIALIZED",
                DataType::Average => "AVERAGE",
                DataType::Minimum => "MINIMUM",
                DataType::Maximum => "MAXIMUM",
                DataType::Variance => "VARIANCE",
                DataType::Stddev => "STDDEV",
                DataType::SampleCount => "SAMPLE_COUNT",
                DataType::RootMeanSquare => "ROOT_MEAN_SQUARE",
                DataType::MaxAbsValue => "MAX_ABS_VALUE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNINITIALIZED" => Some(Self::Uninitialized),
                "AVERAGE" => Some(Self::Average),
                "MINIMUM" => Some(Self::Minimum),
                "MAXIMUM" => Some(Self::Maximum),
                "VARIANCE" => Some(Self::Variance),
                "STDDEV" => Some(Self::Stddev),
                "SAMPLE_COUNT" => Some(Self::SampleCount),
                "ROOT_MEAN_SQUARE" => Some(Self::RootMeanSquare),
                "MAX_ABS_VALUE" => Some(Self::MaxAbsValue),
                _ => None,
            }
        }
    }
}
/// / \brief A named group of statistics.
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatisticsGroup {
    /// / \brief Optional header data.
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief Name of the group.
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// / \brief Statistics the belong to this group.
    #[prost(message, repeated, tag = "3")]
    pub statistics: ::prost::alloc::vec::Vec<Statistic>,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Metric {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief Unit of measurement such as seconds, meters, liters.
    #[prost(string, tag = "2")]
    pub unit: ::prost::alloc::string::String,
    /// / \brief Zero or more named groups of statistics. A statistic group is
    /// / used to bundle data into a logical set with an associated name.
    #[prost(message, repeated, tag = "3")]
    pub statistics_groups: ::prost::alloc::vec::Vec<StatisticsGroup>,
    /// / \brief Zero or more statistics.
    #[prost(message, repeated, tag = "4")]
    pub statistics: ::prost::alloc::vec::Vec<Statistic>,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogPlaybackControl {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief Pause/play the log file.
    #[prost(bool, tag = "2")]
    pub pause: bool,
    /// / \brief Make a relative jump. The value indicates the number of
    /// /        iterations that will be executed at once. If a negative
    /// /        value is specified, the playback will jump backwards.
    #[prost(sint32, tag = "3")]
    pub multi_step: i32,
    /// / \brief Jump to the beginning of the log file.
    #[prost(bool, tag = "4")]
    pub rewind: bool,
    /// / \brief Jump to the end of the log file.
    #[prost(bool, tag = "5")]
    pub forward: bool,
    /// / \brief Jump to a specific simulation time in the log file. The
    /// /        playback service will load the frame with the closest
    /// /        simulation time bigger than the "seek" value.
    #[prost(message, optional, tag = "6")]
    pub seek: ::core::option::Option<Time>,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OccupancyGrid {
    /// / \brief Optional header data. This should contain time of map validity
    /// / and the coordinate frame ID.
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief Metadata for the map.
    #[prost(message, optional, tag = "2")]
    pub info: ::core::option::Option<occupancy_grid::MapMetaInfo>,
    /// / \brief The map data, in row-major order, starting with (0,0).
    /// / Occupancy probabilities are in the range \[0,100\].  Unknown is -1.
    #[prost(bytes = "vec", tag = "3")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// Nested message and enum types in `OccupancyGrid`.
pub mod occupancy_grid {
    #[derive(::rgz_derive::GzMessage)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MapMetaInfo {
        /// / \brief The map load time
        #[prost(message, optional, tag = "1")]
        pub map_load_time: ::core::option::Option<super::Time>,
        /// / \brief The map resolution (meters/cell)
        #[prost(double, tag = "2")]
        pub resolution: f64,
        /// / \brief The map width (cells)
        #[prost(uint32, tag = "3")]
        pub width: u32,
        /// / \brief The map height (cells)
        #[prost(uint32, tag = "4")]
        pub height: u32,
        /// / \brief  The origin of the map.  This is the real-world pose of the
        /// / cell (0,0) in the map
        #[prost(message, optional, tag = "5")]
        pub origin: ::core::option::Option<super::Pose>,
    }
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PointCloud {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    #[prost(message, repeated, tag = "2")]
    pub points: ::prost::alloc::vec::Vec<Vector3d>,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Gps {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    #[prost(string, tag = "2")]
    pub link_name: ::prost::alloc::string::String,
    #[prost(double, tag = "3")]
    pub latitude_deg: f64,
    #[prost(double, tag = "4")]
    pub longitude_deg: f64,
    #[prost(double, tag = "5")]
    pub altitude: f64,
    #[prost(double, tag = "6")]
    pub velocity_east: f64,
    #[prost(double, tag = "7")]
    pub velocity_north: f64,
    #[prost(double, tag = "8")]
    pub velocity_up: f64,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DvlVelocityTracking {
    /// / \brief Message header.
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief Type of DVL.
    #[prost(enumeration = "dvl_velocity_tracking::DvlType", tag = "2")]
    pub r#type: i32,
    /// / \brief Locked on target.
    #[prost(message, optional, tag = "3")]
    pub target: ::core::option::Option<DvlTrackingTarget>,
    /// / \brief Estimated velocity of either target or sensor
    /// / w.r.t. the specified frame, in meters per second.
    #[prost(message, optional, tag = "4")]
    pub velocity: ::core::option::Option<DvlKinematicEstimate>,
    /// / \brief Tracking beams' state.
    #[prost(message, repeated, tag = "5")]
    pub beams: ::prost::alloc::vec::Vec<DvlBeamState>,
    /// / \brief Vendor-specific status (e.g. bitmask, error code).
    #[prost(int32, tag = "6")]
    pub status: i32,
}
/// Nested message and enum types in `DVLVelocityTracking`.
pub mod dvl_velocity_tracking {
    /// / \brief DVL types.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum DvlType {
        /// / \brief Unspecific DVL type.
        Unspecified = 0,
        /// / \brief Piston DVLs.
        Piston = 1,
        /// / \brief Phased array DVLs.
        PhasedArray = 2,
    }
    impl DvlType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                DvlType::Unspecified => "DVL_TYPE_UNSPECIFIED",
                DvlType::Piston => "DVL_TYPE_PISTON",
                DvlType::PhasedArray => "DVL_TYPE_PHASED_ARRAY",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DVL_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "DVL_TYPE_PISTON" => Some(Self::Piston),
                "DVL_TYPE_PHASED_ARRAY" => Some(Self::PhasedArray),
                _ => None,
            }
        }
    }
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Int32 {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief Integer data
    #[prost(int32, tag = "2")]
    pub data: i32,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LinkData {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub linear_velocity: ::core::option::Option<Vector3d>,
    #[prost(message, optional, tag = "4")]
    pub angular_velocity: ::core::option::Option<Vector3d>,
}
/// / \brief Meta information about a camera and the images produced by the
/// / camera. This data is typically published alongside a camera image stream
/// / on a topic called "camera_info".
/// /
/// / Format of this message as heavily borrowed from:
/// / <http://docs.ros.org/melodic/api/sensor_msgs/html/msg/CameraInfo.html>
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CameraInfo {
    /// / \brief Header data. The header timestamp is the time of image
    /// / acquisition. The header data map should have a 'frame_id' key with a
    /// / value that is the camera coordinate frame ID.
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief Width of the image produced by a camera in pixels.
    #[prost(uint32, tag = "2")]
    pub width: u32,
    /// / \brief Height of the image produced by a camera in pixels.
    #[prost(uint32, tag = "3")]
    pub height: u32,
    /// / \brief Distortion information for the camera images.
    #[prost(message, optional, tag = "4")]
    pub distortion: ::core::option::Option<camera_info::Distortion>,
    /// / \brief Camera intrinsics.
    #[prost(message, optional, tag = "5")]
    pub intrinsics: ::core::option::Option<camera_info::Intrinsics>,
    /// / \brief Camera projection information.
    #[prost(message, optional, tag = "6")]
    pub projection: ::core::option::Option<camera_info::Projection>,
    /// / \brief Rectification matrix (stereo cameras only).
    /// / A rotation matrix aligning the camera coordinate system to the ideal
    /// / stereo image plane so that epipolar lines in both stereo images are
    /// / parallel.
    /// / This field should be treated as a 3x3 row-major matrix.
    #[prost(double, repeated, tag = "7")]
    pub rectification_matrix: ::prost::alloc::vec::Vec<f64>,
}
/// Nested message and enum types in `CameraInfo`.
pub mod camera_info {
    /// / The distortion model used by the camera.
    #[derive(::rgz_derive::GzMessage)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Distortion {
        /// / \brief The distortion model used.
        #[prost(enumeration = "distortion::DistortionModelType", tag = "1")]
        pub model: i32,
        /// / \brief Distortion coefficients. The meaning of the coefficients changes
        /// / according to the distortion model:
        /// /
        /// / PLUMP_BOB: 5 parameters, in this order:
        /// /   * k1: radial distortion coefficient k1
        /// /   * k2: radial distortion coefficient k2
        /// /   * t1: tangential distortion coefficient t1
        /// /   * t2: tangential distortion coefficient t2
        /// /   * k3: radial distortion coefficient k3
        /// /
        /// / RATIONAL_POLYNOMIAL: 8 parameters
        /// /
        /// / EQUIDISTANT: 4 parameters, described in this paper:
        /// / <http://www.ee.oulu.fi/~jkannala/publications/tpami2006.pdf>
        #[prost(double, repeated, tag = "2")]
        pub k: ::prost::alloc::vec::Vec<f64>,
    }
    /// Nested message and enum types in `Distortion`.
    pub mod distortion {
        /// / \brief Types of distortion models.
        #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            ::prost::Enumeration
        )]
        #[repr(i32)]
        pub enum DistortionModelType {
            /// / \brief Plumb bob distortion model.
            PlumbBob = 0,
            /// / \brief Rational polynomial distortion model.
            RationalPolynomial = 1,
            /// / \brief Equidistant distortion model.
            Equidistant = 2,
        }
        impl DistortionModelType {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    DistortionModelType::PlumbBob => "PLUMB_BOB",
                    DistortionModelType::RationalPolynomial => "RATIONAL_POLYNOMIAL",
                    DistortionModelType::Equidistant => "EQUIDISTANT",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "PLUMB_BOB" => Some(Self::PlumbBob),
                    "RATIONAL_POLYNOMIAL" => Some(Self::RationalPolynomial),
                    "EQUIDISTANT" => Some(Self::Equidistant),
                    _ => None,
                }
            }
        }
    }
    /// / \brief Intrinsic camera matrix for the raw (distorted) images can be
    /// / generated using the fx, fy, cx, and cy parameters contained in this
    /// / message. For example the intrinsic camera matrix K would be:
    /// /     \[fx  s cx\]
    /// / K = \[ 0 fy cy\]
    /// /     \[ 0  0  1\]
    /// / Projects 3D points in the camera coordinate frame to 2D pixel
    /// / coordinates using the focal lengths (fx, fy) and principal point
    /// / (cx, cy).
    #[derive(::rgz_derive::GzMessage)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Intrinsics {
        /// / \brief 3x3 row-major matrix
        #[prost(double, repeated, tag = "1")]
        pub k: ::prost::alloc::vec::Vec<f64>,
    }
    /// / \brief The projection/camera matrix can be generated using the values in
    /// / this message. For example, the projection matrix P would be:
    /// /
    /// /     \[fx   s cx tx\]
    /// / P = \[ 0  fy cy ty\]
    /// /     \[ 0   0  1  0\]
    /// /
    /// / Where:
    /// / * fx is the X Focal length
    /// / * fy is the Y Focal length
    /// / * cx is the X principal point
    /// / * cy is the Y principal point
    /// / * tx is the X position of the second camera in this camera's frame.
    /// / * ty is the Y position of the second camera in this camera's frame.
    /// / * s is the axis skew.
    /// /
    /// / By convention, this matrix specifies the intrinsic (camera) matrix
    /// /  of the processed (rectified) image. That is, the left 3x3 portion
    /// /  is the normal camera intrinsic matrix for the rectified image.
    /// / It projects 3D points in the camera coordinate frame to 2D pixel
    /// /  coordinates using the focal lengths (fx, fy) and principal point
    /// /  (cx, cy) - these may differ from the values in the Intrinsics message.
    /// / For monocular cameras, tx = ty = 0. Normally, monocular cameras will
    /// /  also have R = the identity and P\[1:3,1:3\] = K.
    /// / For a stereo pair, tx  and ty are related to the
    /// /  position of the optical center of the second camera in the first
    /// /  camera's frame. We assume both cameras are in the same
    /// /  stereo image plane. The first camera always has tx = ty = 0. For
    /// /  the right (second) camera of a horizontal stereo pair, ty = 0 and
    /// /  tx = -fx * B, where B is the baseline between the cameras.
    /// / Given a 3D point \[X Y Z\]', the projection (x, y) of the point onto
    /// /  the rectified image is given by:
    /// /  \[u v w\]' = P * \[X Y Z 1\]'
    /// /         x = u / w
    /// /         y = v / w
    /// /  This holds for both images of a stereo pair.
    #[derive(::rgz_derive::GzMessage)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Projection {
        /// / \brief 3x4 row-major matrix
        #[prost(double, repeated, tag = "1")]
        pub p: ::prost::alloc::vec::Vec<f64>,
    }
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Discovery {
    /// / \brief Optional header data.
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief Version of the discovery protocol.
    #[prost(uint32, tag = "2")]
    pub version: u32,
    /// / \brief Process UUID.
    #[prost(string, tag = "3")]
    pub process_uuid: ::prost::alloc::string::String,
    /// / \brief The type of this message.
    #[prost(enumeration = "discovery::Type", tag = "4")]
    pub r#type: i32,
    /// / \brief Optional flags.
    #[prost(message, optional, tag = "5")]
    pub flags: ::core::option::Option<discovery::Flags>,
    /// / \brief Optional subscriber or publisher information.
    #[prost(oneof = "discovery::DiscContents", tags = "6, 7")]
    pub disc_contents: ::core::option::Option<discovery::DiscContents>,
}
/// Nested message and enum types in `Discovery`.
pub mod discovery {
    /// / \brief Discovery flags.
    #[derive(::rgz_derive::GzMessage)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Flags {
        /// / \brief Flag set when a discovery message is relayed.
        #[prost(bool, tag = "1")]
        pub relay: bool,
        /// / \brief Flag set when we want to avoid to relay a discovery message.
        /// / This is used to avoid loops.
        #[prost(bool, tag = "2")]
        pub no_relay: bool,
    }
    /// / \brief Information about a subscriber.
    #[derive(::rgz_derive::GzMessage)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Subscriber {
        #[prost(string, tag = "1")]
        pub topic: ::prost::alloc::string::String,
    }
    /// / \brief Information about a publisher.
    #[derive(::rgz_derive::GzMessage)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Publisher {
        /// / \brief Topic name.
        #[prost(string, tag = "1")]
        pub topic: ::prost::alloc::string::String,
        /// / \brief ZeroMQ address of the publisher.
        #[prost(string, tag = "2")]
        pub address: ::prost::alloc::string::String,
        /// / \brief Process UUID of the publisher.
        #[prost(string, tag = "3")]
        pub process_uuid: ::prost::alloc::string::String,
        /// / \brief Node UUID of the publisher.
        #[prost(string, tag = "4")]
        pub node_uuid: ::prost::alloc::string::String,
        /// / \brief The scope of this publisher.
        #[prost(enumeration = "publisher::Scope", tag = "5")]
        pub scope: i32,
        /// / \brief Information about a message or service publisher.
        #[prost(oneof = "publisher::PubType", tags = "6, 7")]
        pub pub_type: ::core::option::Option<publisher::PubType>,
    }
    /// Nested message and enum types in `Publisher`.
    pub mod publisher {
        /// / \brief Information about a message publisher.
        #[derive(::rgz_derive::GzMessage)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct MessagePublisher {
            /// / \brief ZeroMQ control address of the publisher.
            /// / \todo(caguero) Is this the same as 'socket_id' in the
            /// / ServicePublisher message?
            #[prost(string, tag = "1")]
            pub ctrl: ::prost::alloc::string::String,
            /// / \brief Message type advertised by this publisher.
            #[prost(string, tag = "2")]
            pub msg_type: ::prost::alloc::string::String,
            /// / \brief Whether the publication has been throttled.
            #[prost(bool, tag = "3")]
            pub throttled: bool,
            /// / \brief The maximum number of messages per second to be published.
            #[prost(uint64, tag = "4")]
            pub msgs_per_sec: u64,
        }
        /// / \brief Information about service provider.
        #[derive(::rgz_derive::GzMessage)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServicePublisher {
            /// / \brief ZeroMQ socket ID used by this publisher.
            #[prost(string, tag = "1")]
            pub socket_id: ::prost::alloc::string::String,
            /// / \brief The name of the request's protobuf message advertised.
            #[prost(string, tag = "2")]
            pub request_type: ::prost::alloc::string::String,
            /// / \brief The name of the response's protobuf message advertised.
            #[prost(string, tag = "3")]
            pub response_type: ::prost::alloc::string::String,
        }
        /// / \brief Defines the different options for the scope of a topic/service.
        #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            ::prost::Enumeration
        )]
        #[repr(i32)]
        pub enum Scope {
            /// / \brief Topic/service only available to subscribers in the same
            /// / process as the publisher.
            Process = 0,
            /// / \brief Topic/service only available to subscribers in the same
            /// / machine as the publisher.
            Host = 1,
            /// / \brief Topic/service available to any subscriber.
            All = 2,
        }
        impl Scope {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Scope::Process => "PROCESS",
                    Scope::Host => "HOST",
                    Scope::All => "ALL",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "PROCESS" => Some(Self::Process),
                    "HOST" => Some(Self::Host),
                    "ALL" => Some(Self::All),
                    _ => None,
                }
            }
        }
        /// / \brief Information about a message or service publisher.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum PubType {
            /// / \brief Message publisher.
            #[prost(message, tag = "6")]
            MsgPub(MessagePublisher),
            /// / \brief Service provider.
            #[prost(message, tag = "7")]
            SrvPub(ServicePublisher),
        }
    }
    /// / \brief Type of discovery message.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Type {
        /// / \brief Type not initialized.
        Uninitialized = 0,
        /// / \brief Advertise message.
        Advertise = 1,
        /// / \brief Subscribe message.
        Subscribe = 2,
        /// / \brief Unadvertise message.
        Unadvertise = 3,
        /// / \brief Hearbeat message.
        Heartbeat = 4,
        /// / \brief Bye message.
        Bye = 5,
        /// / \brief New connection message.
        NewConnection = 6,
        /// / \brief End connection message.
        EndConnection = 7,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Uninitialized => "UNINITIALIZED",
                Type::Advertise => "ADVERTISE",
                Type::Subscribe => "SUBSCRIBE",
                Type::Unadvertise => "UNADVERTISE",
                Type::Heartbeat => "HEARTBEAT",
                Type::Bye => "BYE",
                Type::NewConnection => "NEW_CONNECTION",
                Type::EndConnection => "END_CONNECTION",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNINITIALIZED" => Some(Self::Uninitialized),
                "ADVERTISE" => Some(Self::Advertise),
                "SUBSCRIBE" => Some(Self::Subscribe),
                "UNADVERTISE" => Some(Self::Unadvertise),
                "HEARTBEAT" => Some(Self::Heartbeat),
                "BYE" => Some(Self::Bye),
                "NEW_CONNECTION" => Some(Self::NewConnection),
                "END_CONNECTION" => Some(Self::EndConnection),
                _ => None,
            }
        }
    }
    /// / \brief Optional subscriber or publisher information.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum DiscContents {
        /// / \brief Subscriber information.
        #[prost(message, tag = "6")]
        Sub(Subscriber),
        /// / \brief Publisher information.
        #[prost(message, tag = "7")]
        Pub(Publisher),
    }
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Diagnostics {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    #[prost(message, repeated, tag = "2")]
    pub time: ::prost::alloc::vec::Vec<diagnostics::DiagTime>,
    #[prost(message, optional, tag = "3")]
    pub real_time: ::core::option::Option<Time>,
    #[prost(message, optional, tag = "4")]
    pub sim_time: ::core::option::Option<Time>,
    #[prost(double, tag = "5")]
    pub real_time_factor: f64,
}
/// Nested message and enum types in `Diagnostics`.
pub mod diagnostics {
    #[derive(::rgz_derive::GzMessage)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DiagTime {
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        #[prost(message, optional, tag = "2")]
        pub elapsed: ::core::option::Option<super::Time>,
        #[prost(message, optional, tag = "3")]
        pub wall: ::core::option::Option<super::Time>,
    }
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sky {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    #[prost(double, tag = "2")]
    pub time: f64,
    #[prost(double, tag = "3")]
    pub sunrise: f64,
    #[prost(double, tag = "4")]
    pub sunset: f64,
    #[prost(double, tag = "5")]
    pub wind_speed: f64,
    #[prost(double, tag = "6")]
    pub wind_direction: f64,
    #[prost(message, optional, tag = "7")]
    pub cloud_ambient: ::core::option::Option<Color>,
    #[prost(double, tag = "8")]
    pub humidity: f64,
    #[prost(double, tag = "9")]
    pub mean_cloud_size: f64,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Scene {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub ambient: ::core::option::Option<Color>,
    #[prost(message, optional, tag = "4")]
    pub background: ::core::option::Option<Color>,
    #[prost(message, optional, tag = "5")]
    pub sky: ::core::option::Option<Sky>,
    #[prost(bool, tag = "6")]
    pub shadows: bool,
    #[prost(message, optional, tag = "7")]
    pub fog: ::core::option::Option<Fog>,
    #[prost(bool, tag = "8")]
    pub grid: bool,
    #[prost(message, repeated, tag = "9")]
    pub model: ::prost::alloc::vec::Vec<Model>,
    #[prost(message, repeated, tag = "10")]
    pub light: ::prost::alloc::vec::Vec<Light>,
    #[prost(message, repeated, tag = "11")]
    pub joint: ::prost::alloc::vec::Vec<Joint>,
    /// / \brief Show/hide world origin indicator.
    #[prost(bool, tag = "12")]
    pub origin_visual: bool,
    /// / \brief Shadow caster material script.
    #[prost(message, optional, tag = "13")]
    pub shadow_caster_material_script: ::core::option::Option<material::Script>,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RestResponse {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief ID of the response message
    #[prost(uint32, tag = "2")]
    pub id: u32,
    /// / \brief Type of response
    #[prost(enumeration = "rest_response::Type", tag = "3")]
    pub r#type: i32,
    /// / \brief Message describing the response
    #[prost(string, tag = "4")]
    pub msg: ::prost::alloc::string::String,
}
/// Nested message and enum types in `RestResponse`.
pub mod rest_response {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Type {
        /// / \brief Rest service call was successful
        Success = 0,
        /// / \brief Error calling rest service
        Err = 1,
        /// / \brief Response to a login request
        Login = 2,
        /// / \brief Response to a logout request
        Logout = 3,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Success => "SUCCESS",
                Type::Err => "ERR",
                Type::Login => "LOGIN",
                Type::Logout => "LOGOUT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SUCCESS" => Some(Self::Success),
                "ERR" => Some(Self::Err),
                "LOGIN" => Some(Self::Login),
                "LOGOUT" => Some(Self::Logout),
                _ => None,
            }
        }
    }
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Wind {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    #[prost(message, optional, tag = "2")]
    pub linear_velocity: ::core::option::Option<Vector3d>,
    #[prost(bool, tag = "3")]
    pub enable_wind: bool,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Int64V {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief Vector of int data
    #[prost(int64, repeated, tag = "2")]
    pub data: ::prost::alloc::vec::Vec<i64>,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RestLogout {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief ID of this request message
    #[prost(uint32, tag = "2")]
    pub id: u32,
    /// / \brief the web service url
    #[prost(string, tag = "3")]
    pub url: ::prost::alloc::string::String,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EntityWrenchMap {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief The map of entity wrench messages.
    #[prost(map = "string, message", tag = "2")]
    pub wrenches: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        EntityWrench,
    >,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Physics {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    #[prost(enumeration = "physics::Type", tag = "2")]
    pub r#type: i32,
    #[prost(string, tag = "3")]
    pub solver_type: ::prost::alloc::string::String,
    #[prost(double, tag = "4")]
    pub min_step_size: f64,
    #[prost(int32, tag = "5")]
    pub precon_iters: i32,
    #[prost(int32, tag = "6")]
    pub iters: i32,
    #[prost(double, tag = "7")]
    pub sor: f64,
    #[prost(double, tag = "8")]
    pub cfm: f64,
    #[prost(double, tag = "9")]
    pub erp: f64,
    #[prost(double, tag = "10")]
    pub contact_max_correcting_vel: f64,
    #[prost(double, tag = "11")]
    pub contact_surface_layer: f64,
    #[prost(message, optional, tag = "12")]
    pub gravity: ::core::option::Option<Vector3d>,
    #[prost(bool, tag = "13")]
    pub enable_physics: bool,
    #[prost(double, tag = "14")]
    pub real_time_factor: f64,
    #[prost(double, tag = "15")]
    pub real_time_update_rate: f64,
    #[prost(double, tag = "16")]
    pub max_step_size: f64,
    /// The name of this physics profile (not to be confused with type)
    #[prost(string, tag = "17")]
    pub profile_name: ::prost::alloc::string::String,
    /// / \brief Magnetic field
    #[prost(message, optional, tag = "18")]
    pub magnetic_field: ::core::option::Option<Vector3d>,
}
/// Nested message and enum types in `Physics`.
pub mod physics {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Type {
        Ode = 0,
        Bullet = 1,
        Simbody = 2,
        Dart = 3,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Ode => "ODE",
                Type::Bullet => "BULLET",
                Type::Simbody => "SIMBODY",
                Type::Dart => "DART",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ODE" => Some(Self::Ode),
                "BULLET" => Some(Self::Bullet),
                "SIMBODY" => Some(Self::Simbody),
                "DART" => Some(Self::Dart),
                _ => None,
            }
        }
    }
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoseTrajectory {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub id: u32,
    #[prost(message, repeated, tag = "3")]
    pub pose: ::prost::alloc::vec::Vec<Pose>,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoRecord {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief True to start video recording
    #[prost(bool, tag = "2")]
    pub start: bool,
    /// / \brief True to stop video recording
    #[prost(bool, tag = "3")]
    pub stop: bool,
    /// / \brief Video encoding format, e.g. "mp4", "ogv"
    #[prost(string, tag = "4")]
    pub format: ::prost::alloc::string::String,
    /// / \brief filename of the recorded video
    #[prost(string, tag = "5")]
    pub save_filename: ::prost::alloc::string::String,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TopicInfo {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    #[prost(string, tag = "2")]
    pub msg_type: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub publisher: ::prost::alloc::vec::Vec<Publish>,
    #[prost(message, repeated, tag = "4")]
    pub subscriber: ::prost::alloc::vec::Vec<Subscribe>,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sonar {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    #[prost(string, tag = "2")]
    pub frame: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub world_pose: ::core::option::Option<Pose>,
    #[prost(double, tag = "4")]
    pub range_min: f64,
    #[prost(double, tag = "5")]
    pub range_max: f64,
    #[prost(double, tag = "6")]
    pub radius: f64,
    #[prost(double, tag = "7")]
    pub range: f64,
    /// / \brief The sonar collision shape.
    /// /        possible values are "cone", "sphere".
    /// /        If you set this value to "cone" you need to specify
    /// /        the `radius`.
    #[prost(string, tag = "8")]
    pub geometry: ::prost::alloc::string::String,
    /// / Location of the contact in the world frame.
    #[prost(message, optional, tag = "9")]
    pub contact: ::core::option::Option<Vector3d>,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RestLogin {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief ID of this request message
    #[prost(uint32, tag = "2")]
    pub id: u32,
    /// / \brief Rest service URL
    #[prost(string, tag = "3")]
    pub url: ::prost::alloc::string::String,
    /// / \brief Login user name
    #[prost(string, tag = "4")]
    pub username: ::prost::alloc::string::String,
    /// / \brief Login password
    #[prost(string, tag = "5")]
    pub password: ::prost::alloc::string::String,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorldModify {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    #[prost(string, tag = "2")]
    pub world_name: ::prost::alloc::string::String,
    #[prost(bool, tag = "3")]
    pub remove: bool,
    #[prost(bool, tag = "4")]
    pub create: bool,
    #[prost(bool, tag = "5")]
    pub cloned: bool,
    #[prost(string, tag = "6")]
    pub cloned_uri: ::prost::alloc::string::String,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Atmosphere {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief Type of the atmosphere model.
    #[prost(enumeration = "atmosphere::Type", tag = "2")]
    pub r#type: i32,
    /// / \brief Temperature at sea level in kelvins.
    #[prost(double, tag = "3")]
    pub temperature: f64,
    /// / \brief Pressure at sea level in pascals.
    #[prost(double, tag = "4")]
    pub pressure: f64,
    /// / \brief Mass density of the air at sea level in kg/m^3.
    #[prost(double, tag = "5")]
    pub mass_density: f64,
    /// / \brief Enable atmosphere model
    #[prost(bool, tag = "6")]
    pub enable_atmosphere: bool,
}
/// Nested message and enum types in `Atmosphere`.
pub mod atmosphere {
    /// / \brief Types of atmosphere models.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Type {
        /// / \brief Adiabatic atmosphere model.
        Adiabatic = 0,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Adiabatic => "ADIABATIC",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ADIABATIC" => Some(Self::Adiabatic),
                _ => None,
            }
        }
    }
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Imu {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    #[prost(string, tag = "2")]
    pub entity_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub orientation: ::core::option::Option<Quaternion>,
    #[prost(message, optional, tag = "4")]
    pub angular_velocity: ::core::option::Option<Vector3d>,
    #[prost(message, optional, tag = "5")]
    pub linear_acceleration: ::core::option::Option<Vector3d>,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WirelessNodes {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    #[prost(message, repeated, tag = "2")]
    pub node: ::prost::alloc::vec::Vec<WirelessNode>,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OdometryWithCovariance {
    /// / \brief Optional header data.
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief Estimated pose.
    #[prost(message, optional, tag = "2")]
    pub pose_with_covariance: ::core::option::Option<PoseWithCovariance>,
    /// / \brief Estimated linear and angular velocities.
    #[prost(message, optional, tag = "3")]
    pub twist_with_covariance: ::core::option::Option<TwistWithCovariance>,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JointCmd {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(int32, tag = "3")]
    pub axis: i32,
    #[prost(message, optional, tag = "5")]
    pub position: ::core::option::Option<Pid>,
    #[prost(message, optional, tag = "6")]
    pub velocity: ::core::option::Option<Pid>,
    #[prost(bool, tag = "7")]
    pub reset: bool,
    #[prost(message, optional, tag = "8")]
    pub force_optional: ::core::option::Option<Double>,
}
/// / \brief Holds all the information needed to reconstruct an entity and its
/// / components.
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SerializedEntityMap {
    /// / \brief The entity is uniquely identified by its ID.
    #[prost(uint64, tag = "1")]
    pub id: u64,
    /// / \brief All the components belonging to the entity.
    #[prost(map = "int64, message", tag = "2")]
    pub components: ::std::collections::HashMap<i64, SerializedComponent>,
    /// / \brief Whether the entity and all its components should be removed at the
    /// / current state.
    #[prost(bool, tag = "3")]
    pub remove: bool,
}
/// / \brief Holds all the information needed to reconstruct the state of an
/// / entity-component-system (ECS) architecture at a given time.
/// / An ECS's state consists of several entities, each with an arbitrary number
/// / of components tied to them.
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SerializedStateMap {
    /// / \brief Header data, which contains the simulation time.
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief All the entities currently in the simulation.
    #[prost(map = "uint64, message", tag = "2")]
    pub entities: ::std::collections::HashMap<u64, SerializedEntityMap>,
    /// / \brief Indicates if there is any one time component change,
    /// / or if all changes are periodic.
    #[prost(bool, tag = "3")]
    pub has_one_time_component_changes: bool,
}
/// / \brief All the data needed to step an ECS system, such as current
/// / simulation time and entity states.
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SerializedStepMap {
    /// / \brief Iteration information, such as sim time and paused state.
    #[prost(message, optional, tag = "1")]
    pub stats: ::core::option::Option<WorldStatistics>,
    /// / \brief State of entities and components.
    #[prost(message, optional, tag = "2")]
    pub state: ::core::option::Option<SerializedStateMap>,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UInt32V {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief Vector of int data
    #[prost(uint32, repeated, tag = "2")]
    pub data: ::prost::alloc::vec::Vec<u32>,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VisualV {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief Visual messages.
    #[prost(message, repeated, tag = "2")]
    pub visuals: ::prost::alloc::vec::Vec<Visual>,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModelConfiguration {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// Time when the pose should be enforced
    #[prost(message, optional, tag = "2")]
    pub time: ::core::option::Option<Time>,
    #[prost(string, repeated, tag = "3")]
    pub joint_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(double, repeated, tag = "4")]
    pub joint_positions: ::prost::alloc::vec::Vec<f64>,
    /// Specify model pose
    #[prost(message, optional, tag = "5")]
    pub pose: ::core::option::Option<Pose>,
    /// Option to set model pose by specifying pose of link
    #[prost(string, tag = "6")]
    pub link_name: ::prost::alloc::string::String,
}
#[derive(::rgz_derive::GzMessage)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Odometry {
    /// / \brief Optional header data
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// / \brief Estimated pose.
    #[prost(message, optional, tag = "2")]
    pub pose: ::core::option::Option<Pose>,
    /// / \brief Estimated velocity.
    #[prost(message, optional, tag = "3")]
    pub twist: ::core::option::Option<Twist>,
}
