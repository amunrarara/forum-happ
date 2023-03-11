use hdk::prelude::*;

/**
 * Add your edits to the bottom of this file
 */
pub use posts_zome;

#[hdk_entry_helper]
struct Post {
    title: String,
    content: String,
}

#[hdk_entry_defs]
#[unit_enum(UnitTypes)]
enum EntryTypes {
    Post(Post),
}

#[hdk_link_types]
enum LinkTypes {
    PathToChannel,
    ChannelToPost,
}

#[derive(Serialize, Deserialize, Debug)]
struct CreatePostInput {
    post: Post,
    channel: String,
}

#[hdk_extern]
fn create_post(input: CreatePostInput) -> ExternResult<ActionHash> {
    let action_hash = create_entry(EntryTypes::Post(input.post))?;

    let path = Path::from(format!("all_posts.{}", input.channel.clone()));
    let typed_path = path.typed(LinkTypes::PathToChannel)?;
    typed_path.ensure()?;

    create_link(
        typed_path.path_entry_hash()?,
        action_hash.clone(),
        LinkTypes::ChannelToPost,
        (),
    )?;

    Ok(action_hash)
}

#[hdk_extern]
fn get_channel_posts(channel: String) -> ExternResult<Vec<ActionHash>> {
    let channel_path = Path::from(format!("all_posts.{}", channel)).typed(LinkTypes::PathToChannel);

    let links = get_links(
        channel_path.clone()?.path_entry_hash()?,
        LinkTypes::ChannelToPost,
        None,
    )?;

    let action_hashes: Vec<ActionHash> = links
        .into_iter()
        .map(|link| ActionHash::from(link.target))
        .collect();

    Ok(action_hashes)
}

#[hdk_extern]
fn get_all_channels(_: ()) -> ExternResult<Vec<String>> {
    let path = Path::from("all_posts".to_string());
    let typed_path = path.typed(LinkTypes::PathToChannel)?;
	let all_channel_paths =  typed_path.children_paths()?;

	let mut all_channels: Vec<String> = vec![];

	for path in all_channel_paths {
		let last_component = path.leaf().ok_or(wasm_error!(WasmErrorInner::Guest(String::from("There are no channels available"))))?;

		let channel = String::try_from(last_component).map_err(|err| wasm_error!(err))?;

		all_channels.push(channel);
	}

	Ok(all_channels)



	// let year_path = Path::from(format!("all_comments.{}", year)).typed(LinkTypes::PathTimeIndex);

	// let month_paths: Vec<Path> = year_path.children_paths()?;

	// let mut all_links: Vec<Link> = vec![];
	// for path in month_paths {
	//   let last_component: Component = path.leaf() // `leaf()` gets the latest component
	// 	.ok_or(wasm_error!(WasmErrorInner::Guest(String::from("The path is empty"))))?.clone();
	//   let month = String::try_from(last_component).map_err(|err| wasm_error!(err))?; // Converts the component to a string

	//   let mut links = get_links(path.path_entry_hash()?, LinkTypes::PathToComment, None)?; // Get all the links created above
	//   all_links.append(&mut links); // Collect the links
	// }

	// let all_links: Vec<ActionHash> = links.into_iter()
	//   .map(|link| ActionHash::from(link.target))
	//   .collect(); // Extract the action hash from the links

	// Ok(action_hashes)
}
