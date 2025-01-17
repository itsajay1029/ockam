use crate::node::{get_node_name, initialize_node_if_default, NodeOpts};
use crate::util::{exitcode, extract_address_value, node_rpc, Rpc};
use crate::CommandGlobalOpts;
use anyhow::anyhow;
use clap::Args;
use ockam_api::nodes::models;
use ockam_api::route_to_multiaddr;
use ockam_core::api::Request;
use ockam_core::Route;

/// List TCP Inlets
#[derive(Args, Clone, Debug)]
pub struct ListCommand {
    #[command(flatten)]
    node: NodeOpts,
}

impl ListCommand {
    pub fn run(self, opts: CommandGlobalOpts) {
        initialize_node_if_default(&opts, &self.node.api_node);
        node_rpc(run_impl, (opts, self))
    }
}

async fn run_impl(
    ctx: ockam::Context,
    (opts, cmd): (CommandGlobalOpts, ListCommand),
) -> crate::Result<()> {
    let node_name = get_node_name(&opts.state, &cmd.node.api_node);
    let node_name = extract_address_value(&node_name)?;
    let mut rpc = Rpc::background(&ctx, &opts, &node_name)?;
    rpc.request(Request::get("/node/inlet")).await?;
    let response = rpc.parse_response::<models::portal::InletList>()?;

    if response.list.is_empty() {
        return Err(crate::Error::new(
            exitcode::IOERR,
            anyhow!("No Inlets found on this system!"),
        ));
    }

    for inlet_infor in response.list.iter() {
        println!("Inlet:");
        println!("  Alias: {}", inlet_infor.alias);
        println!("  TCP Address: {}", inlet_infor.bind_addr);
        if let Some(r) = Route::parse(inlet_infor.outlet_route.as_ref()) {
            if let Some(ma) = route_to_multiaddr(&r) {
                println!("  To Outlet Address: {ma}");
            }
        }
    }
    Ok(())
}
