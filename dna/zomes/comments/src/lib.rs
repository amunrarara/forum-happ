use hdk::prelude::*;

/**
 * Add your edits to the bottom of this file
 */
pub use comments_zome;

#[hdk_entry_helper]
struct Comment {
    comment: String,
}


#[hdk_entry_defs]
#[unit_enum(UnitTypes)]
enum EntryTypes {
    Comment(Comment)
}

#[derive(Serialize,Deserialize,Debug)]
struct CreateCommentInput {
    comment_on: ActionHash,
    comment: String
}

#[hdk_extern]
fn create_comment(input: CreateCommentInput) -> ExternResult<ActionHash> {
    let comment = Comment {comment: input.comment};
    let ac_hash = create_entry(EntryTypes::Comment(comment))?;
    Ok(ac_hash)
}
