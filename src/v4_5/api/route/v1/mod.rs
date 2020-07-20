
mod route;
pub use self::route::Route;
#[cfg(feature = "api")] pub use self::route::{ReadNamespacedRouteOptional, ReadNamespacedRouteResponse};
#[cfg(feature = "api")] pub use self::route::{ReadNamespacedRouteStatusOptional, ReadNamespacedRouteStatusResponse};

mod route_ingress;
pub use self::route_ingress::RouteIngress;

mod route_ingress_condition;
pub use self::route_ingress_condition::RouteIngressCondition;

mod route_port;
pub use self::route_port::RoutePort;

mod route_spec;
pub use self::route_spec::RouteSpec;

mod route_status;
pub use self::route_status::RouteStatus;

mod route_target_reference;
pub use self::route_target_reference::RouteTargetReference;

mod tls_config;
pub use self::tls_config::TLSConfig;
