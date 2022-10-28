use darling::FromMeta;

#[derive(Debug, FromMeta)]
pub struct HandlerArgs {
    #[darling(default)]
    pub data: Option<String>,
}

#[derive(Debug, FromMeta)]
pub struct RouteArgs {
    pub path: String,
    #[darling(default)]
    pub data: Option<String>,
}
