table! {
    abstracts (sha1) {
        sha1 -> Varchar,
        content -> Text,
    }
}

table! {
    changelog (id) {
        id -> Int8,
        editgroup_id -> Uuid,
        timestamp -> Timestamp,
    }
}

table! {
    container_edit (id) {
        id -> Int8,
        editgroup_id -> Uuid,
        updated -> Timestamp,
        ident_id -> Uuid,
        rev_id -> Nullable<Uuid>,
        redirect_id -> Nullable<Uuid>,
        prev_rev -> Nullable<Uuid>,
        extra_json -> Nullable<Jsonb>,
    }
}

table! {
    container_ident (id) {
        id -> Uuid,
        is_live -> Bool,
        rev_id -> Nullable<Uuid>,
        redirect_id -> Nullable<Uuid>,
    }
}

table! {
    container_rev (id) {
        id -> Uuid,
        extra_json -> Nullable<Jsonb>,
        name -> Text,
        publisher -> Nullable<Text>,
        issnl -> Nullable<Varchar>,
        wikidata_qid -> Nullable<Varchar>,
        abbrev -> Nullable<Text>,
        coden -> Nullable<Varchar>,
    }
}

table! {
    creator_edit (id) {
        id -> Int8,
        editgroup_id -> Uuid,
        updated -> Timestamp,
        ident_id -> Uuid,
        rev_id -> Nullable<Uuid>,
        redirect_id -> Nullable<Uuid>,
        prev_rev -> Nullable<Uuid>,
        extra_json -> Nullable<Jsonb>,
    }
}

table! {
    creator_ident (id) {
        id -> Uuid,
        is_live -> Bool,
        rev_id -> Nullable<Uuid>,
        redirect_id -> Nullable<Uuid>,
    }
}

table! {
    creator_rev (id) {
        id -> Uuid,
        extra_json -> Nullable<Jsonb>,
        display_name -> Text,
        given_name -> Nullable<Text>,
        surname -> Nullable<Text>,
        orcid -> Nullable<Varchar>,
        wikidata_qid -> Nullable<Varchar>,
    }
}

table! {
    editgroup (id) {
        id -> Uuid,
        editor_id -> Uuid,
        created -> Timestamp,
        extra_json -> Nullable<Jsonb>,
        description -> Nullable<Text>,
    }
}

table! {
    editor (id) {
        id -> Uuid,
        username -> Text,
        is_admin -> Bool,
        registered -> Timestamp,
        active_editgroup_id -> Nullable<Uuid>,
    }
}

table! {
    file_edit (id) {
        id -> Int8,
        editgroup_id -> Uuid,
        updated -> Timestamp,
        ident_id -> Uuid,
        rev_id -> Nullable<Uuid>,
        redirect_id -> Nullable<Uuid>,
        prev_rev -> Nullable<Uuid>,
        extra_json -> Nullable<Jsonb>,
    }
}

table! {
    file_ident (id) {
        id -> Uuid,
        is_live -> Bool,
        rev_id -> Nullable<Uuid>,
        redirect_id -> Nullable<Uuid>,
    }
}

table! {
    file_release (file_rev, target_release_ident_id) {
        file_rev -> Uuid,
        target_release_ident_id -> Uuid,
    }
}

table! {
    file_rev (id) {
        id -> Uuid,
        extra_json -> Nullable<Jsonb>,
        size -> Nullable<Int8>,
        sha1 -> Nullable<Varchar>,
        sha256 -> Nullable<Varchar>,
        md5 -> Nullable<Varchar>,
        mimetype -> Nullable<Text>,
    }
}

table! {
    file_rev_url (id) {
        id -> Int8,
        file_rev -> Uuid,
        rel -> Text,
        url -> Text,
    }
}

table! {
    release_contrib (id) {
        id -> Int8,
        release_rev -> Uuid,
        creator_ident_id -> Nullable<Uuid>,
        raw_name -> Nullable<Text>,
        role -> Nullable<Text>,
        index_val -> Nullable<Int8>,
        extra_json -> Nullable<Jsonb>,
    }
}

table! {
    release_edit (id) {
        id -> Int8,
        editgroup_id -> Uuid,
        updated -> Timestamp,
        ident_id -> Uuid,
        rev_id -> Nullable<Uuid>,
        redirect_id -> Nullable<Uuid>,
        prev_rev -> Nullable<Uuid>,
        extra_json -> Nullable<Jsonb>,
    }
}

table! {
    release_ident (id) {
        id -> Uuid,
        is_live -> Bool,
        rev_id -> Nullable<Uuid>,
        redirect_id -> Nullable<Uuid>,
    }
}

table! {
    release_ref (id) {
        id -> Int8,
        release_rev -> Uuid,
        target_release_ident_id -> Nullable<Uuid>,
        index_val -> Nullable<Int8>,
        key -> Nullable<Text>,
        extra_json -> Nullable<Jsonb>,
        container_title -> Nullable<Text>,
        year -> Nullable<Int8>,
        title -> Nullable<Text>,
        locator -> Nullable<Text>,
    }
}

table! {
    release_rev (id) {
        id -> Uuid,
        extra_json -> Nullable<Jsonb>,
        work_ident_id -> Uuid,
        container_ident_id -> Nullable<Uuid>,
        title -> Text,
        release_type -> Nullable<Text>,
        release_status -> Nullable<Text>,
        release_date -> Nullable<Date>,
        doi -> Nullable<Text>,
        pmid -> Nullable<Varchar>,
        pmcid -> Nullable<Varchar>,
        wikidata_qid -> Nullable<Varchar>,
        isbn13 -> Nullable<Varchar>,
        core_id -> Nullable<Varchar>,
        volume -> Nullable<Text>,
        issue -> Nullable<Text>,
        pages -> Nullable<Text>,
        publisher -> Nullable<Text>,
        language -> Nullable<Text>,
    }
}

table! {
    release_rev_abstract (id) {
        id -> Int8,
        release_rev -> Uuid,
        abstract_sha1 -> Varchar,
        mimetype -> Nullable<Text>,
        lang -> Nullable<Text>,
    }
}

table! {
    work_edit (id) {
        id -> Int8,
        editgroup_id -> Uuid,
        updated -> Timestamp,
        ident_id -> Uuid,
        rev_id -> Nullable<Uuid>,
        redirect_id -> Nullable<Uuid>,
        prev_rev -> Nullable<Uuid>,
        extra_json -> Nullable<Jsonb>,
    }
}

table! {
    work_ident (id) {
        id -> Uuid,
        is_live -> Bool,
        rev_id -> Nullable<Uuid>,
        redirect_id -> Nullable<Uuid>,
    }
}

table! {
    work_rev (id) {
        id -> Uuid,
        extra_json -> Nullable<Jsonb>,
    }
}

joinable!(changelog -> editgroup (editgroup_id));
joinable!(container_edit -> editgroup (editgroup_id));
joinable!(container_ident -> container_rev (rev_id));
joinable!(creator_edit -> editgroup (editgroup_id));
joinable!(creator_ident -> creator_rev (rev_id));
joinable!(file_edit -> editgroup (editgroup_id));
joinable!(file_ident -> file_rev (rev_id));
joinable!(file_release -> file_rev (file_rev));
joinable!(file_release -> release_ident (target_release_ident_id));
joinable!(file_rev_url -> file_rev (file_rev));
joinable!(release_contrib -> creator_ident (creator_ident_id));
joinable!(release_contrib -> release_rev (release_rev));
joinable!(release_edit -> editgroup (editgroup_id));
joinable!(release_ident -> release_rev (rev_id));
joinable!(release_ref -> release_ident (target_release_ident_id));
joinable!(release_ref -> release_rev (release_rev));
joinable!(release_rev -> container_ident (container_ident_id));
joinable!(release_rev -> work_ident (work_ident_id));
joinable!(release_rev_abstract -> abstracts (abstract_sha1));
joinable!(release_rev_abstract -> release_rev (release_rev));
joinable!(work_edit -> editgroup (editgroup_id));
joinable!(work_ident -> work_rev (rev_id));

allow_tables_to_appear_in_same_query!(
    abstracts,
    changelog,
    container_edit,
    container_ident,
    container_rev,
    creator_edit,
    creator_ident,
    creator_rev,
    editgroup,
    editor,
    file_edit,
    file_ident,
    file_release,
    file_rev,
    file_rev_url,
    release_contrib,
    release_edit,
    release_ident,
    release_ref,
    release_rev,
    release_rev_abstract,
    work_edit,
    work_ident,
    work_rev,
);
