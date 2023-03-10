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

#[hdk_link_types]
enum LinkTypes {
    CommentOnToComment
}

#[hdk_extern]
fn create_comment(input: CreateCommentInput) -> ExternResult<ActionHash> {
    let comment = Comment {comment: input.comment};
    let ac_hash = create_entry(EntryTypes::Comment(comment))?;

    create_link(
        input.comment_on,
        ac_hash.clone(),
        LinkTypes::CommentOnToComment,
        ()
    )?;

    Ok(ac_hash)
}

#[hdk_extern]
fn get_comments_on(ac_hash: ActionHash) -> ExternResult<Vec<Record>> {
    let links = get_links(
        ac_hash,
        LinkTypes::CommentOnToComment,
        None,
      )?;

      let mut comments = vec![];

      for link in links {
        let maybe_record = get(ActionHash::from(link.target.clone()), GetOptions::default())?;
        if let Some(record) = maybe_record {
            comments.push(record)
        }
      }

    Ok(comments)
}
