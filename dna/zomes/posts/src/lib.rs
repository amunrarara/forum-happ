use hdk::prelude::*;


/**
 * Add your edits to the bottom of this file
 */
pub use posts_zome;

#[hdk_entry_helper]
struct Post {
    title: String,
    content: String
}

#[hdk_entry_defs]
#[unit_enum(UnitTypes)]
enum EntryTypes {
    Post(Post)
}

#[derive(Serialize,Deserialize,Debug)]
struct CreatePostInput {
    post: Post
}

#[hdk_extern]
fn create_post(input: CreatePostInput) -> ExternResult<ActionHash> {
    let action_hash = create_entry(EntryTypes::Post(input.post))?;
    Ok(action_hash)
}
