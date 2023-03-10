use hdk::prelude::*;

/**
 * Add your edits to the bottom of this file
 */
pub use profiles_zome;

#[hdk_entry_helper]
struct Profile {
    nickname: String,
}

#[hdk_entry_defs]
#[unit_enum(UnitTypes)]
enum EntryTypes {
    Profile(Profile),
}

#[hdk_link_types]
enum LinkTypes {
    AgentToProfile,
}

#[hdk_extern]
fn get_agent_profile(agent_pub_key: AgentPubKey) -> ExternResult<Option<Profile>> {
    let links: Vec<Link> = get_links(
        agent_pub_key,
        LinkTypes::AgentToProfile,
        None,
      )?;

      if let Some(link) = links.first() {
        let profile = get(ActionHash::from(link.target.clone()), GetOptions::default())?;
        if let Some(profile) = profile {
            let p: Profile = profile.entry().to_app_option().map_err(|err| wasm_error!(err))?.ok_or(wasm_error!(WasmErrorInner::Guest("Could not deserialize element to profile".into())))?;
            return Ok(Some(p));
        }
        return Ok(None);
    }
    return Ok(None);
}

#[hdk_extern]
fn get_my_profile(_: ()) -> ExternResult<Option<Profile>> {
    let my_pub_key: AgentPubKey = agent_info()?.agent_initial_pubkey;
    let my_profile = get_agent_profile(my_pub_key).expect("No profile!").ok_or(wasm_error!(WasmErrorInner::Guest("Could not find a profile with your public key".into())))?;
    match Some(my_profile) {
        Some(p) => Ok(Some(p)),
        _ => Ok(None)
    }

    // Ok(Some(Profile {
    //     nickname: "Alice".to_string(),
    // }))
}

#[hdk_extern]
fn create_profile(profile: Profile) -> ExternResult<ActionHash> {
    let action_hash = create_entry(EntryTypes::Profile(profile))?;
    let my_pub_key: AgentPubKey = agent_info()?.agent_initial_pubkey;

    create_link(
        my_pub_key,
        action_hash.clone(),
        LinkTypes::AgentToProfile,
        ()
    )?;

    Ok(action_hash)
}
