use darling::FromMeta;

#[derive(Debug, FromMeta)]
pub struct HandlerArgs {
    #[darling(default)]
    pub data: Option<String>,
}

#[derive(Debug, FromMeta)]
pub struct RouteDataArgs {
    pub path: String,
    #[darling(default)]
    pub data: Option<String>,
}

#[derive(Debug, FromMeta)]
pub struct RouteArgs {
    pub path: String,
}

#[derive(Debug, FromMeta)]
pub struct ModuleArgs {
    pub path: Option<String>,
}
